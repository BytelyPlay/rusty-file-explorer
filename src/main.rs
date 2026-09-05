slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = AppWindow::new()?;

    main_window
        .run()
        .expect("Something went wrong TODO: Make this message better.");

    Ok(())
}
