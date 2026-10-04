use crate::compiled_ui::{Callbacks, FsEntryData, MainWindow, State};
use crate::utils::slint_helpers::model_rc_helpers;
use slint::ComponentHandle;

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
    pub fn get_files(&self) -> model_rc_helpers::InnerVecModel<FsEntryData> {
        let files_rc = &mut self.main_window.global::<State>()
            .get_files();
        let opt_inner_vec_model = model_rc_helpers::get_inner_vec_model(
            files_rc
        );
        if let Some(inner_vec_model) = opt_inner_vec_model {
            inner_vec_model
        } else {
            model_rc_helpers::set_inner_model_to_vec_model(
                files_rc
            )
        }
    }
}