use std::path::PathBuf;
pub trait HasPath {
    fn get_path(&self) -> PathBuf;
    fn get_item_name(&self) -> String;
}
