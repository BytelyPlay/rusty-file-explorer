pub mod callbacks;
pub mod compiled_ui;
pub mod utils;

use slint::ComponentHandle;
use crate::callbacks::file_callback_setup::setup_callbacks;
use crate::compiled_ui::MainWindow;
use crate::utils::fill_initial_files::fill_initial_files;

fn main() {
    let main_window = MainWindow::new()
        .expect("Something went wrong creating the main window.");

    setup_callbacks(main_window.as_weak());
    fill_initial_files(main_window.as_weak());

    main_window
        .run()
        .expect("Something went wrong running the main window.");
}