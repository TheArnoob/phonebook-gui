use std::collections::BTreeMap;

use entry::PhoneEntry;
use iced::widget::{button, row, text, Column, Text};
use iced::{executor, Alignment, Application, Command, Settings, Theme};
use iced_aw::{Grid, GridRow};

mod database;
mod entry;

pub fn main() -> iced::Result {
    PhoneBook::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct PhoneBook {
    phone_book_data: BTreeMap<String, PhoneEntry>,
}

impl Default for PhoneBook {
    fn default() -> Self {
        PhoneBook {
            phone_book_data: a_map(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SavePhonebook,
    LoadPhonebook,
}

fn a_map() -> BTreeMap<String, PhoneEntry> {
    let mut map = BTreeMap::new();
    map.insert(
        "Jack".to_string(),
        PhoneEntry {
            mobile: "0504131252".to_string(),
            work: "0204432224".to_string(),
        },
    );
    map.insert(
        "Mark".to_string(),
        PhoneEntry {
            mobile: "0504327583".to_string(),
            work: "0203344555".to_string(),
        },
    );
    map.insert(
        "Marry".to_string(),
        PhoneEntry {
            mobile: "0203344555".to_string(),
            work: "0504131252".to_string(),
        },
    );
    map
}

impl Application for PhoneBook {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (PhoneBook::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A phone book graphical user interface")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::SavePhonebook => Command::none(),
            Message::LoadPhonebook => Command::none(),
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let phone_numbers_grid = Grid::new();

        let header = GridRow::new()
            .push(Text::new("Name"))
            .push(text("Mobile"))
            .push(text("Work"));
        let mut phone_numbers_grid = phone_numbers_grid.push(header);

        for entry in self.phone_book_data.iter() {
            let entry1 = GridRow::new()
                .push(Text::new(entry.0.clone() + "    "))
                .push(text(entry.1.mobile.clone() + "    "))
                .push(text(entry.1.work.clone()));
            phone_numbers_grid = phone_numbers_grid.push(entry1);
        }

        let c = Column::new();
        let buttons_row = row![
            button("Save phone book").on_press(Message::SavePhonebook),
            button("Load phone book").on_press(Message::LoadPhonebook)
        ]
        .padding(20)
        .align_items(Alignment::Center);
        let c = c.push(buttons_row);
        let c = c.push(phone_numbers_grid);
        c.into()
    }
}
