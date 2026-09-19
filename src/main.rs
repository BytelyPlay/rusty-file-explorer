slint::include_modules!();

fn main() {
    let main_window = MainWindow::new()
        .expect("Something went wrong creating the main window.");

    main_window
        .run()
        .expect("Something went wrong running the main window.");
}