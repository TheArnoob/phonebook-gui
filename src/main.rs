use database::PhoneBookDB;
use entry::PhoneEntry;
use iced::widget::{button, row, text, Column, Text};
use iced::{executor, Alignment, Application, Color, Command, Settings, Theme};
use iced_aw::{Grid, GridRow};
use std::collections::BTreeMap;
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
    error_state: String,
}

impl Default for PhoneBook {
    fn default() -> Self {
        PhoneBook {
            phone_book_data: a_map(),
            error_state: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SavePhonebook,
    LoadPhonebook,
    ClearPhonebook,
}

fn a_map() -> BTreeMap<String, PhoneEntry> {
    let mut map = BTreeMap::new();
    map.insert(
        "Jack".to_string(),
        PhoneEntry {
            mobile: 0504131252.to_string(),
            work: 0204432224.to_string(),
        },
    );
    map.insert(
        "Mark".to_string(),
        PhoneEntry {
            mobile: 0504327583.to_string(),
            work: 0203344555.to_string(),
        },
    );
    map.insert(
        "Marry".to_string(),
        PhoneEntry {
            mobile: 0503344555.to_string(),
            work: 0204131252.to_string(),
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
            Message::SavePhonebook => {
                let fd = rfd::FileDialog::new();
                let file = fd.save_file();

                let phone_book_db = match file {
                    Some(file) => PhoneBookDB { file_path1: file },
                    None => return Command::none(),
                };

                match phone_book_db.write(&self.phone_book_data) {
                    Ok(_) => (),
                    Err(e) => self.error_state = format!("Failed to save file: {e}"),
                }

                Command::none()
            }
            Message::LoadPhonebook => {
                let fd = rfd::FileDialog::new();
                let filename = fd.pick_file();

                let phone_book_db = match filename {
                    Some(f) => PhoneBookDB { file_path1: f },
                    None => return Command::none(),
                };

                let data_map = phone_book_db.read();

                let data_map1 = match data_map {
                    Ok(data) => data,
                    Err(err) => {
                        self.error_state = format!("Failed to open file: {err}");
                        return Command::none();
                    }
                };

                self.phone_book_data = data_map1;
                self.error_state = String::new();

                Command::none()
            }

            Message::ClearPhonebook => {
                self.phone_book_data.clear();

                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let mut phone_numbers_grid = Grid::new();

        let header = GridRow::new()
            .push(Text::new("Name  "))
            .push(text("Mobile  "))
            .push(text("Work"));
        if !self.phone_book_data.is_empty() {
            phone_numbers_grid = phone_numbers_grid.push(header);
        }

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
            button("Load phone book").on_press(Message::LoadPhonebook),
            button("Clear phone book").on_press(Message::ClearPhonebook)
        ]
        .padding(20)
        .align_items(Alignment::Center);
        let error_state_field = Text::new(&self.error_state).style(Color::from_rgb8(255, 0, 0));
        let c = c.push(buttons_row);
        let c = c.push(error_state_field);
        let c = c.push(phone_numbers_grid);
        c.into()
    }
}
