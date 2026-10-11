use slint::Image;
use crate::compiled_ui::FsEntryData;

pub struct RawFsEntryData {
    // TODO: Figure out the icon. It should support raw RGBA or SVGs or PNGs. An enum would probably help.
    // pub icon: Vec<u8>,
    pub name: String
}

impl RawFsEntryData {
    pub fn convert_to_fs_entry_data(self) -> FsEntryData {
        FsEntryData {
            icon: Image::default(),
            name: self.name.into()
        }
    }
}