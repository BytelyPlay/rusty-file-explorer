use slint::Image;
use crate::compiled_ui::DisplayableFsEntryData;

pub struct FsEntryData {
    // TODO: Figure out the icon. It should support raw RGBA or SVGs or PNGs. An enum would probably help.
    // pub icon: Vec<u8>,
    pub name: String
}

impl FsEntryData {
    pub fn convert_to_displayable_fs_entry_data(self) -> DisplayableFsEntryData {
        DisplayableFsEntryData {
            icon: Image::default(),
            name: self.name.into()
        }
    }
}