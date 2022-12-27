
use ndarray::prelude::*;

use nifti::{
    IntoNdArray,
    NiftiObject,
    NiftiVolume,
    ReaderOptions, 
    writer::WriterOptions,
    NiftiHeader
};

use std::path::Path;
use log::info;


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

