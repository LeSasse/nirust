
use ndarray::prelude::*;
use itertools::iproduct;
use nifti::{
    IntoNdArray,
    NiftiObject,
    NiftiVolume,
    ReaderOptions, 
    writer::WriterOptions,
    NiftiHeader
};

use std::path::Path;
use log::{info, warn};


pub fn load_img(
    path: &Path
) -> (NiftiHeader, Array<f32, IxDyn>) {
    
    info!("Reading NIfTI at {:?}", path);
    let img = match ReaderOptions::new().read_file(path) {
        Ok(img) => img,
        Err(e) => panic!("Error: {}", e)
    };
    let header = img.header().clone();
    let volume = img.volume();
    let dims = volume.dim();
    let n_dims = dims.len();
    info!("Dimensions detected: {:?}", n_dims);
    info!("Consuming data...");
    let image_data = img.into_volume().into_ndarray::<f32>().unwrap();

    (header, image_data)
}

pub fn save_img(
    path: &Path,
    header: &NiftiHeader,
    image_data: Array<f32, IxDyn>
) {
    if path.exists() {
        warn!("{:?} exists, overwriting image!", path);
    }
    info!("Saving image at {:?}", path);
    match WriterOptions::new(path)
        .reference_header(header)
        .write_nifti(&image_data) {
            Ok(()) => {}
            Err(e) => {panic!("Error: {}", e)}
    }
}

pub fn get_affine(header: &NiftiHeader) -> Array2<f32> {
    
    arr2(
        &[
            header.srow_x,
            header.srow_y,
            header.srow_z,
            [0., 0., 0., 1.]
        ]
    )
}

// Function to transform voxel coordinates into real-world coordinates
// TODO: Refactor so that x, y, z can be individual arrays as well
pub fn coord_transform(
    x: f32,
    y: f32,
    z: f32,
    affine: &Array2<f32>
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
    target_shape: (usize, usize, usize)
) -> Array<f32, Ix3> {

    let mut resampled_data = Array::zeros(target_shape);

    let (dim_x, dim_y, dim_z) = target_shape;
    let mut counter = 0;
    for (i, j, k) in iproduct!(0..dim_x, 0..dim_y, 0..dim_z) {
        counter += 1;
        let (x_target, y_target, z_target) = coord_transform(
            i as f32,
            j as f32,
            k as f32,
            target_affine,
        );  
        
        for (i_src, j_src, k_src) in iproduct!() { 
            let (x_src, y_src, z_src) = coord_transform(
                i_src as f32,
                j_src as f32,
                k_src as f32,
                source_affine
            );
        }
        println!("point {} {} {}", x_target, y_target, z_target);
    }
    println!("{} counter", counter);
    
    resampled_data
}


fn euclidean_distance(
    point_a: (f32, f32, f32), point_b: (f32, f32, f32)
) -> f32 {
    let (a_x, a_y, a_z) = point_a;
    let (b_x, b_y, b_z) = point_b;
    
    0.5
}