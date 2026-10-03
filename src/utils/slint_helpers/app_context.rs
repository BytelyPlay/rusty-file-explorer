use slint::{ComponentHandle, Model, ModelRc, VecModel};
use crate::compiled_ui::{FsEntryData, MainWindow, State};

pub struct AppContext<'a> {
    main_window: &'a MainWindow
}

impl<'a> AppContext<'a> {
    pub fn set_files(&self, vec: Vec<FsEntryData>) {
        self.main_window.global::<State>()
            .set_files(
                ModelRc::new(
                    VecModel::from(
                        vec
                    )
                )
            );
    }
    pub fn get_files_model(&self) -> Vec<FsEntryData> {
        self.main_window.global::<State>()
            .get_files()
            .iter()
            .collect()
    }
}