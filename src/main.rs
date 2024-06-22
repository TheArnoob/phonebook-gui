use database::PhoneBookDB;
use entry::PhoneEntry;
use iced::widget::{button, row, text, text_input, Column, Text};
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
    is_adding: bool,
    is_modifying: bool,
    name_to_be_modified: String,
    new_entry_name: String,
    new_entry_phone_number: String,
    new_entry_work_number: String,
    filter: String,
}

impl Default for PhoneBook {
    fn default() -> Self {
        PhoneBook {
            phone_book_data: BTreeMap::new(),
            error_state: String::new(),
            is_adding: false,
            is_modifying: false,
            name_to_be_modified: String::new(),
            new_entry_name: String::new(),
            new_entry_phone_number: String::new(),
            new_entry_work_number: String::new(),
            filter: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    AddRow,
    Cancel,
    ClearPhonebook,
    EditNewEntryName(String),
    EditNewEntryWorkNumber(String),
    EditNewEntryPhoneNumber(String),
    Insert,
    LoadPhonebook,
    Modify(String),
    Remove(String),
    SavePhonebook,
    EditEntryPhoneNumber(String),
    EditEntryWorkNumber(String),
    Filter(String),
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
        String::from("A phone book GUI")
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

            Message::AddRow => {
                self.is_adding = true;

                Command::none()
            }

            Message::EditNewEntryName(s) => {
                self.new_entry_name = s;
                Command::none()
            }
            Message::EditNewEntryPhoneNumber(s) => {
                self.new_entry_phone_number = s;
                Command::none()
            }

            Message::EditNewEntryWorkNumber(s) => {
                self.new_entry_work_number = s;
                Command::none()
            }
            Message::Insert => {
                if self.phone_book_data.contains_key(&self.new_entry_name) {
                    self.error_state = String::from("The entry already exists");
                } else if self.new_entry_name.is_empty() {
                    self.error_state = String::from("The name is empty.");
                } else {
                    self.phone_book_data.insert(
                        self.new_entry_name.clone(),
                        PhoneEntry {
                            mobile: self.new_entry_phone_number.clone(),
                            work: self.new_entry_work_number.clone(),
                        },
                    );

                    self.is_adding = false;

                    self.new_entry_name.clear();
                    self.new_entry_phone_number.clear();
                    self.new_entry_work_number.clear();
                    self.error_state = String::new();
                }

                Command::none()
            }
            Message::Cancel => {
                self.is_adding = false;

                Command::none()
            }

            Message::Remove(name) => {
                self.phone_book_data
                    .remove_entry(&name)
                    .expect("Name must exist");
                Command::none()
            }

            Message::Modify(name) => {
                self.is_modifying = !self.is_modifying;
                self.name_to_be_modified = name;
                Command::none()
            }

            Message::EditEntryPhoneNumber(phone_number) => {
                let mutable_entry = self
                    .phone_book_data
                    .get_mut(&self.name_to_be_modified)
                    .expect("Name must exist.");
                mutable_entry.mobile = phone_number;
                Command::none()
            }

            Message::EditEntryWorkNumber(work_number) => {
                let mutable_entry = self
                    .phone_book_data
                    .get_mut(&self.name_to_be_modified)
                    .expect("Name must exist.");
                mutable_entry.work = work_number;

                Command::none()
            }

            Message::Filter(filter) => {
                self.filter = filter.clone();
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let mut phone_numbers_grid = Grid::new();

        let filter_text =
            row![text_input("Filter", &self.filter).on_input(|filter| { Message::Filter(filter) })];

        let header = GridRow::new()
            .push(Text::new("Name  "))
            .push(text("Mobile  "))
            .push(text("Work"));
        if !self.phone_book_data.is_empty() {
            phone_numbers_grid = phone_numbers_grid.push(header);
        }

        for entry in self.phone_book_data.iter() {
            let mut entry1 = GridRow::new().push(Text::new(entry.0.clone() + "    "));
            if self.is_modifying == true && self.name_to_be_modified == entry.0.clone() {
                entry1 = entry1
                    .push(
                        text_input("", &entry.1.mobile)
                            .on_input(|phone_number| Message::EditEntryPhoneNumber(phone_number)),
                    )
                    .push(
                        text_input("", &entry.1.work)
                            .on_input(|work_number| Message::EditEntryWorkNumber(work_number)),
                    )
                    .push(row![
                        button("Done modifying").on_press(Message::Modify(entry.0.clone())),
                        button("Remove entry").on_press(Message::Remove(entry.0.clone())),
                    ]);
            } else {
                entry1 = entry1
                    .push(text(entry.1.mobile.clone() + "    "))
                    .push(text(entry.1.work.clone()))
                    .push(row![
                        button("Modify entry").on_press(Message::Modify(entry.0.clone())),
                        button("Remove entry").on_press(Message::Remove(entry.0.clone())),
                    ]);
            }

            if entry.0.contains(&self.filter) {
                phone_numbers_grid = phone_numbers_grid.push(entry1);
            }
        }

        let c = Column::new();
        let buttons_row = row![
            button("Save phone book").on_press(Message::SavePhonebook),
            button("Load phone book").on_press(Message::LoadPhonebook),
            button("Clear phone book").on_press(Message::ClearPhonebook),
            button("Add phone book entry").on_press(Message::AddRow)
        ]
        .padding(20)
        .align_items(Alignment::Center);
        let row = row![
            text_input("Name", &self.new_entry_name)
                .on_input(|name| Message::EditNewEntryName(name)),
            text_input("Phone number", &self.new_entry_phone_number)
                .on_input(|phone_number| Message::EditNewEntryPhoneNumber(phone_number)),
            text_input("Work number", &self.new_entry_work_number)
                .on_input(|work_number| Message::EditNewEntryWorkNumber(work_number)),
            button("Add entry").on_press(Message::Insert),
            button("Cancel").on_press(Message::Cancel)
        ]
        .padding(20)
        .align_items(Alignment::Center);

        let error_state_field = Text::new(&self.error_state).style(Color::from_rgb8(255, 0, 0));
        let mut c = c.push(buttons_row);
        if self.is_adding == true {
            c = c.push(row);
        }
        let c = c.push(error_state_field);

        let c = c.push(filter_text);

        let c = c.push(phone_numbers_grid);

        c.into()
    }
}
