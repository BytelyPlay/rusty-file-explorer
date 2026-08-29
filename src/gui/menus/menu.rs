use iced::Element;
use crate::utils::message::Message;

pub trait Menu {
    fn create_menu(&self) -> Element<Message>;
}