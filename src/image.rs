
use ndarray::prelude::*;
use nifti::{NiftiHeader};

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
    
    let res = (world_x, world_y, world_z);
    res
    
}

pub fn squeeze_header(header: &NiftiHeader) -> NiftiHeader {
    let mut new_header = header.clone();
    new_header
}

