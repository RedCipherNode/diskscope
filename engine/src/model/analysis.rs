use crate::model::empty_directory::EmptyDirectory;
use crate::model::empty_file::EmptyFile;
use crate::model::extension_statistic::ExtensionStatistic;
use crate::model::largest_directory::LargestDirectory;
use crate::model::largest_file::LargestFile;

pub struct Analysis {
    pub total_files: u64,
    pub total_directories: u64,
    pub total_logical_size: u64,

    pub extension_statistics: Vec<ExtensionStatistic>,

    pub largest_directories: Vec<LargestDirectory>,
    pub largest_files: Vec<LargestFile>,
    pub empty_files: Vec<EmptyFile>,
    pub empty_directories: Vec<EmptyDirectory>,
}
