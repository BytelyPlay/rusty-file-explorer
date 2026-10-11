use crate::compiled_ui::{Callbacks, FsEntryData, MainWindow, State};
use crate::utils::slint_helpers::app_context_raw_data::AppContextRawData;
use crate::utils::slint_helpers::app_context_wrapper::AppContextWrapper;
use crate::utils::slint_helpers::model_rc_accessors::ModelRcAccessor;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub type AppContextRawDataWrappedType = Arc<RwLock<AppContextRawData>>;

/// This is basically just a wrapper for MainWindow, which is reference-counted.
/// So in turn, when you clone AppContext. You just create a new reference to main_window.
pub struct AppContext {
    main_window: MainWindow,
    raw_data: AppContextRawDataWrappedType
}

impl AppContext {
    pub fn new(main_window: MainWindow) -> Self {
        AppContext {
            main_window,
            raw_data: Arc::new(
                RwLock::new(
                    AppContextRawData {
                        open_path: PathBuf::new()
                    }
                )
            )
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
    /// This doesn't change anything other than the directory bar.
    pub fn get_open_directory(&self) -> PathBuf {
        self.raw_data
            .write()
            .unwrap()
            .open_path
            .clone()
    }
    /// This doesn't change anything other than the directory bar.
    pub fn set_open_directory(&self, path: PathBuf) {
        self.main_window.global::<State>()
            .set_currently_open_directory(
                path.as_path()
                    .to_string_lossy()
                    .to_string()
                    .into()
            );
        self.raw_data
            .write()
            .unwrap()
            .open_path = path;
    }
    pub fn as_wrapper(&self) -> AppContextWrapper {
        AppContextWrapper::new(
            self.main_window.as_weak(),
            Arc::downgrade(&self.raw_data)
        )
    }
}
impl Clone for AppContext {
    /// This doesn't copy the main_window, it just creates a new strong reference.
    /// Same thing with the raw data.
    fn clone(&self) -> Self {
        Self {
            main_window: self.main_window.clone_strong(),
            raw_data: self.raw_data.clone()
        }
    }
}