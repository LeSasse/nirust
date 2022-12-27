

use ndarray::prelude::*;
use nifti::NiftiHeader;
use std::f32::NAN;
use log::{info, warn};

use crate::image::{get_affine, coord_transform};


// find the x index at which the
// first non-negative real world coordinate appears
fn _find_x_origin(n_x: i32, affine: &Array2<f32>) -> f32 {
   for x_i in 0..n_x {
        let x_i = x_i as f32;
        let (x_coord, _, _) = coord_transform(x_i, 0., 0., affine);
        if x_coord < 0. {
            continue;
        } else {
            return x_i
        }
    }
    warn!("No origin for x could be found! Returning the highest index...");
    
    n_x as f32
}

pub fn mask_hemi(
    header: &NiftiHeader,
    image_data: &mut Array<f32, IxDyn>,
    side: &str
) {
//) -> &'a mut Array<f32, IxDyn> {

    let dims = image_data.shape();
    let n_dims = dims.len();
    let affine = get_affine(header);    
    // how many slices are there in the x direction?
    let n_x = dims[0] as i32;
    // left of origin (i.e. negative real-world coordinates are 'left')
    let x_origin = _find_x_origin(n_x, &affine);
    info!("image dimensions are {:?}", &dims);

    let x_origin = x_origin as i32;
        
    // Slice according to side parameter and n of dimensions
    // TODO: This feels like code duplication and unnecessary ifs,
    // but not sure how to do this better using rust + ndarray
    if side == "left" {
        if n_dims == 3 {
            image_data.slice_mut(s![0..x_origin, .., ..]).fill(NAN);
        } else if n_dims == 4 {
            image_data.slice_mut(s![0..x_origin, .., .., ..]).fill(NAN);
        }
    } else if side == "right" {   
        if n_dims == 3 {
            image_data.slice_mut(s![x_origin..n_x, .., ..]).fill(NAN);
        } else if n_dims == 4 {
            image_data.slice_mut(s![x_origin..n_x, .., .., ..]).fill(NAN);
        }
    } else {
        panic!("Error: 'side' parameter can be 'left' or 'right'!");
    }
}

