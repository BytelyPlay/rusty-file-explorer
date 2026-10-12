use crate::compiled_ui::MainWindow;
use crate::utils::slint_helpers::app_context::AppContext;
use crate::utils::slint_helpers::app_context_raw_data::AppContextRawData;
use slint::{EventLoopError, Weak};
use std::sync;
use std::sync::RwLock;

/// This is basically just AppContext, but with the Send trait.
/// Just made for the upgrade_on_slint_event_loop function.
pub struct AppContextWrapper {
    weak_main_window: Weak<MainWindow>,
    raw_data: sync::Weak<RwLock<AppContextRawData>>
}

impl AppContextWrapper {
    pub fn upgrade_on_slint_event_loop(
        self,
        f: impl FnOnce(AppContext) + Send + 'static
    ) -> Result<(), EventLoopError> {
        self.weak_main_window.upgrade_in_event_loop(move |main_window| {
            let opt_raw_data = self.raw_data.upgrade();

            if let None = opt_raw_data {
                // TODO: Handle this
                todo!()
            }

            f(
                AppContext::new_with_raw_data(
                    main_window,
                    opt_raw_data.unwrap()
                )
            )
        })
    }
    pub fn new(
        weak_main_window: Weak<MainWindow>,
        raw_data: std::sync::Weak<RwLock<AppContextRawData>>
    ) -> Self {
        AppContextWrapper {
            weak_main_window,
            raw_data,
        }
    }
}

impl Clone for AppContextWrapper {
    fn clone(&self) -> Self {
        Self::new(
            self.weak_main_window.clone(),
            self.raw_data.clone()
        )
    }
}