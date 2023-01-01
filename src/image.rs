
use smartcore::neighbors::knn_classifier::*;
//use smartcore::math::distance::*;

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
        let (x, y, z) = coord_transform(
            i as f32, j as f32, k as f32, source_affine
        );
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
    ).enumerate() {    
        let (x, y, z) = coord_transform(
            i as f32, j as f32, k as f32, target_affine
        );
        coords_target.slice_mut(s![row, 0]).fill(x);
        coords_target.slice_mut(s![row, 1]).fill(y);
        coords_target.slice_mut(s![row, 2]).fill(z);
    }
    
    let knn = KNNClassifier::fit(
        &coords_source, &labels_source, Default::default()
    ).unwrap();
    
    let y_hat = knn.predict(&coords_target).unwrap();    
    println!("{:?}", &y_hat);    
    println!("{:?}", &y_hat.len());    
    // println!("{:?}", labels_source);
        
    // }
    
    // let tree = KdTree::new(coords_source);
    
    // // get the nearest neighbours
    // for row in coords_target.axis_iter(Axis(0)) {
    //     let (neighbour_index, _) = tree.nearest(&row);
    //     println!("{:?}", neighbour_index);
    // }
    
    return resampled_data            
}
    // for (i, j, k) in iproduct!(
    //     0..resampled_data.shape()[0],
    //     0..resampled_data.shape()[1],
    //     0..resampled_data.shape()[2]
    // ) {
        
    //     let coord_target = coord_transform(
    //         i as f32, j as f32, k as f32, target_affine
    //     );
        
    //     let mut min: Option<f32> = None;
    //     let mut val_nneigh: Option<f32> = None;
    //     // find nearest neighbour and corresponding value
    //     for ((i_src, j_src, k_src), value) in source.indexed_iter() {
    //         let coord_source = coord_transform(
    //             i_src as f32, j_src as f32, k_src as f32, source_affine
    //         );
    //         let x = euclidean_distance(&coord_target, &coord_source);
    //         if let Some(current_min) = min {
    //             if x < current_min {
    //                 min = Some(x);
    //                 val_nneigh = Some(*value)
    //             }
    //         } else {
    //             min = Some(x);
    //             val_nneigh = Some(*value);
    //         }
    //     }
    //     let min = min.unwrap();
    //     let val_nneigh = val_nneigh.unwrap();
        
    //     resampled_data.slice_mut(s![i, j, k]).fill(val_nneigh);
    // }
    


fn euclidean_distance(
    point_a: &(f32, f32, f32), point_b: &(f32, f32, f32)
) -> f32 {
    let (a_x, a_y, a_z) = point_a;
    let (b_x, b_y, b_z) = point_b;
    
    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2) + (a_z - b_z).powi(2)).sqrt()
}