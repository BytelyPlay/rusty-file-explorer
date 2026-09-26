use log::error;
use slint::Weak;
use crate::callbacks::file_ops_callbacks;

use crate::compiled_ui::MainWindow;

pub fn setup_callbacks(weak_main_window: Weak<MainWindow>) {
    if let Some(main_window) = weak_main_window.upgrade() {
        main_window.on_fs_entry_clicked(
            file_ops_callbacks::fs_entry_clicked
        );
    } else {
        error!("For some reason, weak pointer to the main window was not able to be upgraded.");
    }
}