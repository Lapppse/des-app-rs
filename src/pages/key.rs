use des_ndtp::{Error, FromHexStr, MainKey, ShiftDirection, ToHexString};
use iced::widget::{
    column, component, container, horizontal_space, row, slider, text, text_input, Component,
};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Event {
    KeyInput(String),
    RoundChanged(u8),
}

#[derive(Debug, Clone)]
pub struct KeyPage {
    key: Option<MainKey>,
    key_input: String,
    round: u8,
    round_key: Option<MainKey>,
    error: Option<Error>,
}

impl Default for KeyPage {
    fn default() -> Self {
        Self {
            key: None,
            key_input: String::new(),
            round: 1, // Doing it manually because of that
            round_key: None,
            error: None,
        }
    }
}

impl<Message> Component<Message> for KeyPage {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::KeyInput(input) => {
                if input.len() <= 16 {
                    self.key_input = input.to_uppercase();
                }
                if self.key_input.len() == 16 {
                    self.round_key = None;
                    let parsed_key = MainKey::from_hex_str(&self.key_input);
                    match parsed_key {
                        Ok(key) => {
                            self.key = Some(key);
                            self.error = None;
                        }
                        Err(e) => {
                            self.key = None;
                            self.error = Some(e);
                        }
                    }
                }
            }
            Event::RoundChanged(round) => {
                self.round = round;
            }
        }
        if let Some(key) = &self.key {
            self.round_key = Some(key.get_round_key(self.round, ShiftDirection::Left).unwrap())
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, iced::Theme, iced::Renderer> {
        let inputs = row![
            horizontal_space(Length::FillPortion(1)),
            container(
                column![
                    text_input("Input Main Key", &self.key_input)
                        .on_input(Event::KeyInput)
                        .size(24),
                    container(slider(1..=16, self.round, Event::RoundChanged).height(40))
                        .max_width(400)
                        .padding(10),
                ]
                .align_items(Alignment::Center)
                .spacing(10),
            )
            .width(Length::FillPortion(6)),
            horizontal_space(Length::FillPortion(1))
        ];

        let mut outputs = column![].spacing(10);
        if let Some(error) = &self.error {
            outputs = outputs.push(text(format!("Error: {}", error)).size(24))
        }
        if let Some(key) = &self.key {
            outputs = outputs.push(text(format!("Main Key: {}", key.to_upper_hex())).size(24))
        };
        if let Some(round_key) = &self.round_key {
            outputs = outputs.push(
                text(format!(
                    "Round: {} key: {}",
                    self.round,
                    round_key.as_bitvec().to_upper_hex()
                ))
                .size(24),
            )
        }

        let content = column![inputs, outputs]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content).center_x().center_y().into()
    }
}

impl<'a, Message> From<KeyPage> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(key_page: KeyPage) -> Self {
        component(key_page)
    }
}
