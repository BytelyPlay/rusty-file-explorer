pub mod callbacks;
pub mod compiled_ui;

use slint::ComponentHandle;
use crate::callbacks::file_callback_setup::setup_callbacks;
use crate::compiled_ui::MainWindow;

fn main() {
    let main_window = MainWindow::new()
        .expect("Something went wrong creating the main window.");

    setup_callbacks(main_window.as_weak());

    main_window
        .run()
        .expect("Something went wrong running the main window.");
}