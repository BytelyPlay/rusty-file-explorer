use log::error;
use slint::{ComponentHandle, Weak};
use crate::callbacks::file_ops_callbacks;

use crate::compiled_ui::{Callbacks, FsEntryData, MainWindow};

pub fn setup_callbacks(weak_main_window: Weak<MainWindow>) {
    if let Some(main_window) = weak_main_window.upgrade() {
        setup_callbacks_internal(&main_window);
    } else {
        error!("For some reason, weak pointer to the main window was not able to be upgraded.");
    }
}

fn setup_callbacks_internal(main_window: &MainWindow) {
    main_window.global::<Callbacks>().on_fs_entry_clicked(
        |fs_entry: FsEntryData| {
            main_window.as_weak();
            file_ops_callbacks::fs_entry_clicked(fs_entry);
        }
    );
}

fn callback_internal<T>() -> Box<dyn T>
where
T: Fn(){

}