use std::path::PathBuf;

use crate::db::{Dataset, DatasetType, ImageSource};

mod db;

fn main() {
    //  Create an object detection dataset;
    let dataset_name = String::from("ObjDetDataset");
    let image_source = ImageSource::TopLevel {
        root_directory: PathBuf::from(r"/home/dg084/datasets/obj-det-dataset/coco/images"),
    };
    let dataset_type = DatasetType::Detection;

    let my_dataset = Dataset::new(dataset_name, image_source, dataset_type);

    dbg!(&my_dataset);
}
