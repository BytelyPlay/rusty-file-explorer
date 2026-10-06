use crate::compiled_ui::{Callbacks, FsEntryData, MainWindow, State};
use slint::{ComponentHandle, Model, VecModel};
use crate::utils::slint_helpers::model_rc_accessors::ModelRcAccessor;

pub struct AppContext {
    main_window: MainWindow
}

impl AppContext {
    pub fn new(main_window: MainWindow) -> Self {
        AppContext {
            main_window
        }
    }
    pub fn get_callbacks(&self) -> Callbacks {
        self.main_window.global::<Callbacks>()
    }
    pub fn get_files(&self) -> ModelRcAccessor {
        let state = self.main_window.global::<State>();

        let files_rc = self.main_window.global::<State>()
            .get_files();
    }
}