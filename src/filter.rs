use gtk::{gio::FileInfo, glib::{object::Cast, Object}, CustomFilter};

use crate::utils::is_image;

pub fn custom_filter() -> CustomFilter {
    CustomFilter::new(|obj | filter_images(obj))
}
pub fn filter_images(obj: &Object) -> bool {
    let file_info = match obj.downcast_ref::<FileInfo>() {
        Some(file_info) => file_info,
        None => return false,
    };
    return is_image(file_info);

}