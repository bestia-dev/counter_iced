// counter_iced/src/main.rs

#![doc=include_str!("../README.md")]
// this will avoid Windows to show the terminal when running the exe
#![windows_subsystem = "windows"]

use iced::widget::{button, column, text, Column};
use iced::window;
use iced::Center;

pub fn main() -> iced::Result {
    iced::application("counter_iced", Counter::update, Counter::view)
        .window(window::Settings {
            size: iced::Size::new(400.0, 300.0),
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_x(Center)
    }
}
