use log::{info, warn};
use ndarray::prelude::*;
use nifti::NiftiHeader;
use std::f32::NAN;
use std::option::Option::Some;

use crate::image::{coord_transform, get_affine, resample_3d_nifti};

pub fn parcellate(
    image_data: &Array<f32, IxDyn>,
    image_header: &NiftiHeader,
    parcellation_data: &Array<f32, Ix3>,
    parcellation_header: &NiftiHeader,
) -> Array<f32, IxDyn> {
    let img_shape = image_data.shape();
    let parc_shape = parcellation_data.shape();

    let x = img_shape[0];
    let y = img_shape[1];
    let z = img_shape[2];

    let i = parc_shape[0];
    let j = parc_shape[1];
    let k = parc_shape[2];

    if (i, j, k) != (x, y, z) {
        warn!("Image and parcellation have different spatial shape");
        warn!("Resampling parcellation to image...");

        let image_affine = get_affine(image_header);
        let parc_affine = get_affine(parcellation_header);
        let parcellation_data_resampled = resample_3d_nifti(
            parcellation_data,
            &parc_affine,
            &image_affine,
            (x, y, z),
        );
        parcellate_any(image_data, &parcellation_data_resampled)
    } else {
        parcellate_any(image_data, parcellation_data)
    }
}

pub fn mask_hemi(
    header: &NiftiHeader,
    image_data: &mut Array<f32, IxDyn>,
    side: &str,
) {
    let dims = image_data.shape();
    let n_dims = dims.len();
    let affine = get_affine(header);
    // how many slices are there in the x direction?
    let n_x = dims[0] as i32;
    // left of origin (i.e. negative real-world coordinates are 'left')
    let x_origin = _find_x_origin(n_x, &affine);
    info!("image dimensions are {:?}", &dims);

    let x_origin = x_origin as i32;

    match (side, n_dims) {
        ("left", 3) => image_data.slice_mut(s![0..x_origin, .., ..]).fill(NAN),
        ("left", 4) => {
            image_data.slice_mut(s![0..x_origin, .., .., ..]).fill(NAN)
        }
        ("right", 3) => {
            image_data.slice_mut(s![x_origin..n_x, .., ..]).fill(NAN)
        }
        ("right", 4) => image_data
            .slice_mut(s![x_origin..n_x, .., .., ..])
            .fill(NAN),
        _ => panic!("Error: 'side' parameter can be 'left' or 'right'!"),
    }
    info!("Done masking the {} side of the image!", side);
}

fn parcellate_any(
    image_data: &Array<f32, IxDyn>,
    parcellation_data: &Array<f32, Ix3>,
) -> Array<f32, IxDyn> {
    let dims = image_data.shape().len();
    info!("Image to parcellate has {} dimensions.", dims);
    if dims == 3 {
        _parcellate_3d(image_data, parcellation_data).into_dyn()
    } else if dims == 4 {
        _parcellate_4d(image_data, parcellation_data).into_dyn()
    } else {
        panic!("Not a 3D or 4D image!");
    }
}

fn _parcellate_3d(
    image_data: &Array<f32, IxDyn>,
    parcellation_data: &Array<f32, Ix3>,
) -> Array<f32, Ix1> {
    let n_rois = _find_max_val(parcellation_data) as i32;
    info!("{} ROIs detected in parcellation!", n_rois);
    let mut means_rois = Array::<f32, Ix1>::zeros(n_rois as usize);
    for roi in 1..=n_rois {
        let index_array: Array<bool, Ix3> =
            parcellation_data.mapv(|x| x == roi as f32);
        let roi_data = Array::from_iter(
            image_data
                .iter()
                .zip(index_array.iter())
                .filter_map(|(x, y)| if *y { Some(*x) } else { None }),
        );
        means_rois
            .slice_mut(s![roi - 1])
            .fill(roi_data.mean().unwrap());
    }
    means_rois
}

fn _parcellate_4d(
    image_data: &Array<f32, IxDyn>,
    parcellation_data: &Array<f32, Ix3>,
) -> Array<f32, Ix2> {
    let n_rois = _find_max_val(parcellation_data) as i32;
    info!("{} ROIs detected in parcellation!", n_rois);

    let time_dim = image_data.shape()[3] as usize;
    let mut mean_timeseries =
        Array::<f32, Ix2>::zeros((time_dim, n_rois as usize));

    for roi in 1..=n_rois {
        let mut vox_counter = 0.;
        let mut mean_timeseries_roi = Array::<f32, Ix1>::zeros(time_dim);

        for ((i, j, k), parc_val) in parcellation_data.indexed_iter() {
            if *parc_val == roi as f32 {
                vox_counter += 1.;
                mean_timeseries_roi =
                    mean_timeseries_roi + image_data.slice(s![i, j, k, ..]);
            }
        }
        mean_timeseries_roi /= vox_counter;
        mean_timeseries
            .slice_mut(s![.., roi - 1])
            .assign(&mean_timeseries_roi);
    }
    mean_timeseries
}

fn _find_max_val(array: &Array<f32, Ix3>) -> f32 {
    let mut val = 0.;
    for x in array.iter() {
        if *x > val {
            val = *x
        }
    }
    val
}

// find the x index at which the
// first non-negative real world coordinate appears
fn _find_x_origin(n_x: i32, affine: &Array2<f32>) -> f32 {
    for x_i in 0..n_x {
        let x_i = x_i as f32;
        let (x_coord, _, _) = coord_transform(x_i, 0., 0., affine);
        if x_coord < 0. {
            continue;
        } else {
            return x_i;
        }
    }
    warn!("No origin for x could be found! Returning the highest index...");

    n_x as f32
}
