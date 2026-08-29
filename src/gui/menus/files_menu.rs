use iced::Element;
use iced::widget::{button, row};
use crate::gui::menus::menu::Menu;
use crate::utils::message::Message;

pub struct FilesMenu {
    // This is simply here to not let it be constructed in other places.
    // TODO: Replace with a macro.
    _priv: ()
}

impl Menu for FilesMenu {
    fn create_menu(&self) -> Element<Message> {
        
    }
}
impl FilesMenu {
    pub fn new() -> FilesMenu {
        Self {
            _priv: ()
        }
    }
}