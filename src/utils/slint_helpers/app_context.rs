use crate::compiled_ui::{Callbacks, FsEntryData, MainWindow, State};
use crate::utils::slint_helpers::model_rc_accessors::ModelRcAccessor;
use slint::ComponentHandle;

/// This is basically just a wrapper for MainWindow, which is reference-counted.
/// So in turn, when you clone AppContext. You just create a new reference to main_window.
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
    pub fn get_files(&self) -> ModelRcAccessor<FsEntryData> {
        // Stupid, but I kind of have to do this
        let cloned_self_1 = self.clone();
        let cloned_self_2 = self.clone();

        ModelRcAccessor::new(
            Box::new(
                move || cloned_self_1.main_window.global::<State>()
                    .get_files()
            ),
            Box::new(
                move |files| cloned_self_2.main_window.global::<State>()
                    .set_files(files)
            )
        )
    }
}
impl Clone for AppContext {
    /// This doesn't copy the main_window, it just creates a new strong reference.
    fn clone(&self) -> Self {
        Self {
            main_window: self.main_window.clone_strong()
        }
    }
}