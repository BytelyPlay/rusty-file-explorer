use log::{error, warn};
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
    let weak_main_window = main_window.as_weak();
    
    main_window.global::<Callbacks>().on_fs_entry_clicked(
        move |fs_entry: FsEntryData| {
            let opt_main_window = main_window_as_strong_or_log(
                weak_main_window.clone()
            );
            
            if let Some(main_window) = opt_main_window {
                file_ops_callbacks::fs_entry_clicked(
                    fs_entry, main_window
                );
            }
        }
    );
}

fn main_window_as_strong_or_log(
    weak_main_window: Weak<MainWindow>
) -> Option<MainWindow> {
    let opt_main_window = weak_main_window.upgrade();

    if opt_main_window.is_none() {
        warn!("Couldn't upgrade Weak<MainWindow> while calling callback\
        , this could mean that the \
        callback was somehow called after the MainWindow went out of scope.")
    }
    opt_main_window
}