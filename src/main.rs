use crate::gui::context::Context;
use crate::gui::menus::files_menu::FilesMenu;

pub mod utils;
pub mod gui;

pub fn main() -> iced::Result {
    iced::application(|| {
        Context::new(
            Box::new(
                FilesMenu::new()
            )
        )
    }, Context::update, Context::view)
        .run()
}
