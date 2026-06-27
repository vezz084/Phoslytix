use std::{ffi::OsString, path::PathBuf};

// The user can define the kind of dataset. Supported right now are Object Detection, Classification, Segmentation
#[derive(Debug)]
pub enum DatasetType {
    Detection,
    Classification,
    Segmentation,
}

// The user can specify the image source.
// TopLevel means that a root directory would be provided and everything inside that directory (upto one level) must be loaded
// Paths means that a root directory would be provided and a list of filenames that are to be loaded
// TopLevelWithExt means that a user can provide a root directory and some extension(s) that are to be loaded
#[derive(Debug)]
pub enum ImageSource {
    TopLevel {
        root_directory: PathBuf,
    },
    Paths {
        root_directory: PathBuf,
        filenames: Vec<PathBuf>,
    },
    TopLevelWithExt {
        root_directory: PathBuf,
        extensions: Vec<OsString>,
    },
}

#[derive(Debug)]
pub struct Dataset {
    pub dataset_name: String,
    pub image_source: ImageSource,
    dataset_id: u8,
    pub dataset_type: DatasetType,
}

impl Dataset {
    pub fn new(dataset_name: String, image_source: ImageSource, dataset_type: DatasetType) -> Self {
        // TODO: Some validation check before creating dataset.
        // - Does the directory contain images?
        // - Do the provided paths resolve to images?
        Dataset {
            dataset_name,
            image_source,
            dataset_id: 0,
            dataset_type,
        }
    }

    pub fn load_images() {
        !todo!()
    }
}
