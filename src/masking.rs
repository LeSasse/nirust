

use ndarray::prelude::*;
use std::f32::NAN;
use nifti::{
    writer::WriterOptions,
    error::NiftiError,
    NiftiObject,
    ReaderOptions,
    NiftiVolume,
    IntoNdArray
};
use log::{info, warn};

use crate::image::{get_affine, coord_transform};


// find the x index at which the
// first non-negative real world coordinate appears
fn _find_x_origin(n_x: i32, affine: &Array2<f32>) -> f32 {
   for x_i in 0..n_x {
        let x_i = x_i as f32;
        let (x_coord, _, _) = coord_transform(x_i, 0., 0., &affine);
        if x_coord < 0. {
            continue;
        } else {
            return x_i
        }
    }
    warn!("No origin for x could be found! Returning the highest index...");
    let res = n_x as f32;
    return res
}

pub fn mask_hemi (
    input_nifti: String,
    output_nifti: String,
    side: &str
) -> Result<(), NiftiError> {

    match side {
        "left"  => info!("Masking left hemisphere..."),
        "right" => info!("Masking right hemisphere ..."),
        _ => panic!("Error: 'side' parameter can be 'left' or 'right'!")
    }
    
    info!("Reading NIfTI at {}", input_nifti);
    let img = ReaderOptions::new().read_file(input_nifti)?;
    
    let header = img.header().clone();
    let volume = img.volume();
    let dims = volume.dim();
    let n_dims = dims.len();
    info!("Dimensions detected: {:?}", n_dims);    

    let affine = get_affine(&header);    
    // how many slices are there in the x direction?
    let n_x = dims[0];
    // left of origin (i.e. negative real-world coordinates are 'left')
    let x_origin = _find_x_origin(n_x.into(), &affine);
    let mut image_data = img.into_volume().into_ndarray::<f32>().unwrap();
    info!("image dimensions are {:?}", image_data.shape());
        
    let n_x = n_x as i32;
    let x_origin = x_origin as i32;
        
    // Slice according to side parameter and n of dimensions
    // TODO: This feels like code duplication and unnecessary ifs,
    // but not sure how to do this better using rust + ndarray
    
    if n_dims == 3 {
        if side == "left" {
            image_data.slice_mut(s![0..x_origin, .., ..]).fill(NAN);
        } else if side == "right" {
            image_data.slice_mut(s![x_origin..n_x, .., ..]).fill(NAN);
        }
    } else if n_dims == 4 {   
        if side == "left" {
            image_data.slice_mut(s![0..x_origin, .., .., ..]).fill(NAN);
        } else if side == "right" {
            image_data.slice_mut(s![x_origin..n_x, .., .., ..]).fill(NAN);
        }
    }
    
    match WriterOptions::new(output_nifti).reference_header(
        &header
    ).write_nifti(&image_data) {
        Ok(()) => {}
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
    Ok(())
}

