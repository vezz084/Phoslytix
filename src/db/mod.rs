use std::path::PathBuf;

#[derive(Debug)]
pub enum DatasetType {
    Detection,
    Classification,
    Segmentation,
}

#[derive(Debug)]
pub struct Dataset {
    pub dataset_name: String,
    pub root_dir: PathBuf,
    dataset_id: u8,
    pub dataset_type: DatasetType,
}

impl Dataset {
    pub fn new(dataset_name: String, root_dir: PathBuf, dataset_type: DatasetType) -> Self {
        Dataset {
            dataset_name,
            root_dir,
            dataset_id: 0,
            dataset_type,
        }
    }
}
