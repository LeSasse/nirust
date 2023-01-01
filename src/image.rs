use smartcore::neighbors::knn_classifier::*;
//use smartcore::math::distance::*;

use itertools::iproduct;
use ndarray::prelude::*;
use nifti::{
    writer::WriterOptions, IntoNdArray, NiftiHeader, NiftiObject, NiftiVolume,
    ReaderOptions,
};

use log::{info, warn};
use std::path::Path;

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

pub fn get_affine(header: &NiftiHeader) -> Array2<f32> {
    arr2(&[
        header.srow_x,
        header.srow_y,
        header.srow_z,
        [0., 0., 0., 1.],
    ])
}

pub fn coord_transform(
    x: f32,
    y: f32,
    z: f32,
    affine: &Array2<f32>,
) -> (f32, f32, f32) {
    let voxel_coords = arr2(&[[x], [y], [z], [1.]]);
    let world_coords = affine.dot(&voxel_coords);
    //(world_coords[0], world_coords[1], world_coords[2])
    //info!("{:?}", world_coords);

    let world_x: f32 = *world_coords.slice(s![0, 0]).into_scalar();
    let world_y: f32 = *world_coords.slice(s![1, 0]).into_scalar();
    let world_z: f32 = *world_coords.slice(s![2, 0]).into_scalar();

    (world_x, world_y, world_z)
}

pub fn resample_3d_nifti(
    source: &Array<f32, Ix3>,
    source_affine: &Array2<f32>,
    target_affine: &Array2<f32>,
    target_shape: (usize, usize, usize),
) -> Array<f32, Ix3> {
    // The idea of resampling is to generate an empty array with the
    // "new, correct" shape, and interpolate the values in this array using
    // the source array to "resample/interpolate"
    let mut resampled_data: Array<f32, Ix3> = Array::zeros(target_shape);

    let x_dim_src = source.shape()[0];
    let y_dim_src = source.shape()[1];
    let z_dim_src = source.shape()[2];

    // transform coords from source
    let n_elem_src = x_dim_src * y_dim_src * z_dim_src;
    let mut labels_source: Vec<i32> = Vec::new();
    let mut coords_source = Array::zeros((n_elem_src, 3));
    for (row, ((i, j, k), value)) in source.indexed_iter().enumerate() {
        let (x, y, z) =
            coord_transform(i as f32, j as f32, k as f32, source_affine);
        coords_source.slice_mut(s![row, 0]).fill(x);
        coords_source.slice_mut(s![row, 1]).fill(y);
        coords_source.slice_mut(s![row, 2]).fill(z);
        labels_source.push(*value as i32);
    }

    // transform coords for reference
    let n_elem_targ = target_shape.0 * target_shape.1 * target_shape.2;
    let mut coords_target = Array::zeros((n_elem_targ, 3));
    for (row, (i, j, k)) in iproduct!(
        0..resampled_data.shape()[0],
        0..resampled_data.shape()[1],
        0..resampled_data.shape()[2]
    )
    .enumerate()
    {
        let (x, y, z) =
            coord_transform(i as f32, j as f32, k as f32, target_affine);
        coords_target.slice_mut(s![row, 0]).fill(x);
        coords_target.slice_mut(s![row, 1]).fill(y);
        coords_target.slice_mut(s![row, 2]).fill(z);
    }

    // fit on the source image and predict values of target array based
    // on euclidean distance between coordinates
    let knn =
        KNNClassifier::fit(&coords_source, &labels_source, Default::default())
            .unwrap();

    let y_hat = knn.predict(&coords_target).unwrap();

    // fill values back into the resampled data array
    for (row, (i, j, k)) in iproduct!(
        0..resampled_data.shape()[0],
        0..resampled_data.shape()[1],
        0..resampled_data.shape()[2]
    )
    .enumerate()
    {
        resampled_data
            .slice_mut(s![i, j, k])
            .fill(y_hat[row] as f32)
    }
    resampled_data
}
