//! The `nirust::image` module provides functions to read and write NIfTI
//! images, access specific information from the header (for example, the
//! affine matrix), and perform simple transformations of an image (for
//! example resampling an image using `resample_3d_nifti`).

use itertools::iproduct;
use ndarray::prelude::*;
use ndarray_linalg::solve::Inverse;
use nifti::{
    writer::WriterOptions, IntoNdArray, NiftiHeader, NiftiObject, NiftiVolume,
    ReaderOptions,
};

use log::{info, warn};
use std::path::Path;

/// Load a 3D or 4D NIfTI image.
///
/// The function loads the image into memory and returns a tuple containing
/// the header with the image metadata, and an ndarray with the voxel-wise
/// image data.
/// 
/// Parameters
/// ----------
/// path : Path to the NIfTI image
///
pub fn load_img(path: &Path) -> (NiftiHeader, Array<f32, IxDyn>) {
    info!("Reading NIfTI at {:?}", path);
    let img = match ReaderOptions::new().read_file(path) {
        Ok(img) => img,
        Err(e) => panic!("Error: {}", e),
    };
    let header = img.header().clone();
    let volume = img.volume();
    let dims = volume.dim();
    let n_dims = dims.len();
    info!("Dimensions detected: {:?}", n_dims);
    let image_data = img.into_volume().into_ndarray::<f32>().unwrap();

    (header, image_data)
}

/// Save a 3D or 4D NIfTI image to disk.
///
/// Parameters
/// ----------
/// path : Path and filename of the data to be saved.
///
/// header : Header metadata for the NIfTI image to be stored.
///
/// image_data : ndarray containing the voxel-wise image data.
///
pub fn save_img(
    path: &Path,
    header: &NiftiHeader,
    image_data: Array<f32, IxDyn>,
) {
    if path.exists() {
        warn!("{:?} exists, overwriting image!", path);
    }
    info!("Saving image at {:?}", path);
    match WriterOptions::new(path)
        .reference_header(header)
        .write_nifti(&image_data)
    {
        Ok(()) => {}
        Err(e) => {
            panic!("Error: {}", e)
        }
    }
}

/// Extract the affine matrix from a NiftiHeader.
/// 
/// At the moment, the function only returns the sform affine. I need to still
/// implement obtaining the qform or fall-back affines if appropriate.
/// The function should eventually follow [the conventions outlined by the 
/// Python-based nibabel library](https://nipy.org/nibabel/nifti_images.html#choosing-the-image-affine):
///
/// > 1. If sform_code != 0 (‘unknown’) use the sform affine; else
/// > 2. If qform_code != 0 (‘unknown’) use the qform affine; else
/// > 3. Use the fall-back affine.
///
/// Parameters
/// ----------
/// header : Header metadata for the NIfTI image to be stored.
///
pub fn get_affine(header: &NiftiHeader) -> Array2<f32> {
    arr2(&[
        header.srow_x,
        header.srow_y,
        header.srow_z,
        [0., 0., 0., 1.],
    ])
}

/// Convert voxel coordinates into "real-world" coordinates of the reference
/// space. Practically, the function can also be used to transform the
/// "real-world" coordinates into voxel coordinates by providing the inverse
/// of the affine matrix.
pub fn coord_transform(
    x: f32,
    y: f32,
    z: f32,
    affine: &Array2<f32>,
) -> (f32, f32, f32) {
    let voxel_coords = arr2(&[[x], [y], [z], [1.]]);
    let world_coords = affine.dot(&voxel_coords);

    let world_x: f32 = *world_coords.slice(s![0, 0]).into_scalar();
    let world_y: f32 = *world_coords.slice(s![1, 0]).into_scalar();
    let world_z: f32 = *world_coords.slice(s![2, 0]).into_scalar();

    (world_x, world_y, world_z)
}

/// Resample a 3D NIfTI image.
/// The idea of resampling is to generate an empty array with the
/// output shape, and interpolate the values in this array using
/// the source array. That is, the voxel coordinates of the output array are
/// converted to "real-world" coordinates in the reference space using the
/// target affine. The inverse of the target affine can then be used to convert
/// "real-world" coordinates into voxel-coordinates of the source array.
/// For more info on coordinate systems and affines, see
/// [the excellent nibabel documentation](https://nipy.org/nibabel/coordinate_systems.html).
/// Currently, this function only uses "nearest-neighbour" interpolation, i.e.
/// by simply using the source voxel coordinates to select the value from the
/// source array and assigning it to the corresponding voxel coordinate in the
/// target array.
/// There are plans to add further interpolation techniques. 
///
/// Parameters
/// ----------
/// source : ndarray containing the data of the image that is to be resampled.
///
/// source_affine : affine matrix that belongs to the image to be resampled.
///
/// target_affine : affine matrix of the image that is to be matched.
///
/// target_shape : shape of the output array, i.e. the shape of the image after
/// resampling.
pub fn resample_3d_nifti(
    source: &Array<f32, Ix3>,
    source_affine: &Array2<f32>,
    target_affine: &Array2<f32>,
    target_shape: (usize, usize, usize),
) -> Array<f32, Ix3> {
    let mut resampled_data: Array<f32, Ix3> = Array::zeros(target_shape);

    let x_dim_src = source.shape()[0];
    let y_dim_src = source.shape()[1];
    let z_dim_src = source.shape()[2];

    let x_dim_targ = resampled_data.shape()[0];
    let y_dim_targ = resampled_data.shape()[1];
    let z_dim_targ = resampled_data.shape()[2];

    // transform coords for reference
    let mut target_indices = Array::zeros((4, resampled_data.len()));
    for (row, (i, j, k)) in
        iproduct!(0..x_dim_targ, 0..y_dim_targ, 0..z_dim_targ).enumerate()
    {
        target_indices
            .slice_mut(s![.., row])
            .assign(&array!(i as f32, j as f32, k as f32, 1.));
    }
    let source_indices = source_affine
        .inv()
        .unwrap()
        .dot(target_affine)
        .dot(&target_indices);

    for (col_src, col_targ) in source_indices
        .axis_iter(Axis(1))
        .zip(target_indices.axis_iter(Axis(1)))
    {
        let i_src = _handle_index_format(&col_src[0], &x_dim_src);
        let j_src = _handle_index_format(&col_src[1], &y_dim_src);
        let k_src = _handle_index_format(&col_src[2], &z_dim_src);

        let i_targ = _handle_index_format(&col_targ[0], &x_dim_targ);
        let j_targ = _handle_index_format(&col_targ[1], &y_dim_targ);
        let k_targ = _handle_index_format(&col_targ[2], &z_dim_targ);

        resampled_data
            .slice_mut(s![i_targ, j_targ, k_targ])
            .assign(&source.slice(s![i_src, j_src, k_src]));
    }

    resampled_data
}

fn _handle_index_format(x: &f32, x_max: &usize) -> i32 {
    let x_max_as_i = *x_max as i32;
    let x_as_i = *x as i32;
    if x_as_i < 0 {
        0
    } else if x_as_i >= x_max_as_i {
        x_max_as_i - 1
    } else {
        x_as_i
    }
}
