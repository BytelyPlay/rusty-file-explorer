pub mod callbacks;
pub mod compiled_ui;
pub mod utils;
pub mod errors;

use std::rc::{Rc, Weak};
use slint::ComponentHandle;
use crate::callbacks::file_callback_setup::setup_callbacks;
use crate::compiled_ui::MainWindow;
use crate::utils::fill_initial_files::fill_initial_files;
use crate::utils::slint_helpers::app_context::AppContext;

fn main() {
    let main_window = MainWindow::new()
        .expect("Something went wrong creating the main window.");

    let app_context = Rc::new(
        AppContext::new(
            main_window.clone_strong()
        )
    );

    setup_callbacks(Rc::downgrade(&app_context));
    fill_initial_files(Rc::downgrade(&app_context));

    main_window
        .run()
        .expect("Something went wrong running the main window.");
}