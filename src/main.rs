use std::path::PathBuf;

use crate::db::{Dataset, DatasetType};

mod db;

fn main() {
    //  Create an object detection dataset;
    let dataset_name = String::from("ObjDetDataset");
    let images_root_dir = PathBuf::from(r"/home/dg084/datasets/obj-det-dataset/coco/images");
    let dataset_type = DatasetType::Detection;

    let my_dataset = Dataset::new(dataset_name, images_root_dir, dataset_type);

    dbg!(&my_dataset);
}
