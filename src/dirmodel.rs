use std::path::PathBuf;

use gtk::{gio::{File, FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE, FILE_ATTRIBUTE_STANDARD_NAME, FILE_ATTRIBUTE_THUMBNAIL_IS_VALID, FILE_ATTRIBUTE_THUMBNAIL_PATH}, CustomSorter, DirectoryList, FilterListModel, SingleSelection, SortListModel};

use crate::{filter::custom_filter, utils::attrs_to_str};

pub fn wallpaper_dir_model(dir: PathBuf) -> Option<SingleSelection>{
    if !dir.exists() {return None;};
    if !dir.is_dir() {return None;};

    let file = File::for_path(dir);
    let attrs = attrs_to_str([
        FILE_ATTRIBUTE_STANDARD_CONTENT_TYPE,
        FILE_ATTRIBUTE_THUMBNAIL_PATH,
        FILE_ATTRIBUTE_THUMBNAIL_IS_VALID,
        FILE_ATTRIBUTE_STANDARD_NAME,
    ]);
    let model = DirectoryList::new(Some(&attrs), Some(&file));
    let filtered_model = FilterListModel::new(Some(model), Some(custom_filter()));
    let selection_model = SingleSelection::new(Some(filtered_model));
    Some(selection_model)
}