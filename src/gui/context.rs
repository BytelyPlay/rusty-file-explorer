use iced::Element;
use crate::gui::menus::menu::Menu;
use crate::utils::message::Message;

pub struct Context {
    menu_in_use: Box<dyn Menu>
}

impl Context {
    pub fn new(menu: Box<dyn Menu>) -> Context {
        Self {
            menu_in_use: menu
        }
    }
    pub fn use_menu(&mut self, menu: Box<dyn Menu>) {
        self.menu_in_use = menu;
    }
    pub fn update(&mut self, _message: Message) {
        todo!("Not implemented")
    }
    pub fn view(&self) -> Element<Message> {
        (*self.menu_in_use).create_menu()
    }
}