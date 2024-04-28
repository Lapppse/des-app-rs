use des_ndtp::{Block, Error, FromHexStr, MainKey, ToHexString};
use iced::widget::{
    checkbox, column, component, container, row, text, text_input, Component, Space,
};
use iced::{Alignment, Element, Length};

#[derive(Debug, Clone)]
pub enum Event {
    KeyInput(String),
    BlockInput(String),
    EncodeToggled(bool),
    Ignore(String),
}

#[derive(Debug, Clone, Default)]
pub struct BlockPage {
    key: Option<MainKey>,
    block: Option<Block>,
    encoded_block: Option<Block>,
    key_input: String,
    block_input: String,
    encode: bool,
    error: (Option<Error>, Option<Error>),
    encode_time: Option<time::Duration>,
}

impl<Message> Component<Message> for BlockPage {
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        let (old_block, old_key, old_encode) = (self.block.clone(), self.key.clone(), self.encode);
        match event {
            Event::BlockInput(input) => {
                if input.len() <= 16 {
                    self.block_input = input.to_uppercase();
                }
                if self.block_input.len() == 16 {
                    let parsed_block = Block::from_hex_str(&self.block_input);
                    match parsed_block {
                        Ok(block) => {
                            self.block = Some(block);
                            self.error.0 = None;
                        }
                        Err(e) => {
                            self.block = None;
                            self.error.0 = Some(e);
                        }
                    }
                }
            }
            Event::KeyInput(input) => {
                if input.len() <= 16 {
                    self.key_input = input.to_uppercase();
                }
                if self.key_input.len() == 16 {
                    let parsed_key = MainKey::from_hex_str(&self.key_input);
                    match parsed_key {
                        Ok(key) => {
                            self.key = Some(key);
                            self.error.1 = None;
                        }
                        Err(e) => {
                            self.key = None;
                            self.error.1 = Some(e);
                        }
                    }
                }
            }
            Event::EncodeToggled(encode) => self.encode = encode,
            Event::Ignore(_) => {}
        }

        // can't chain if let and usual if in rust 1.76.0
        if old_block == self.block && old_key == self.key && old_encode == self.encode {
            return None;
        }
        if let (Some(key), Some(block)) = (&self.key, &self.block) {
            let start: time::OffsetDateTime;
            let end: time::OffsetDateTime;
            if self.encode {
                start = time::OffsetDateTime::now_utc();
                self.encoded_block = Some(block.encode(key).unwrap());
                end = time::OffsetDateTime::now_utc();
                self.encode_time = Some(end - start);
            } else {
                start = time::OffsetDateTime::now_utc();
                self.encoded_block = Some(block.decode(key).unwrap());
                end = time::OffsetDateTime::now_utc();
                self.encode_time = Some(end - start);
            }
        }
        None
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event, iced::Theme, iced::Renderer> {
        let inputs = row![
            Space::with_width(Length::FillPortion(1)),
            container(
                column![
                    text_input("Input Main Key", &self.key_input)
                        .on_input(Event::KeyInput)
                        .size(24),
                    text_input("Input Text block", &self.block_input)
                        .on_input(Event::BlockInput)
                        .size(24),
                    container(
                        checkbox(String::from("Encode"), self.encode)
                            .size(30)
                            .text_size(24)
                            .on_toggle(Event::EncodeToggled)
                    )
                    .padding(20)
                ]
                .align_items(Alignment::Center)
                .spacing(10),
            )
            .width(Length::FillPortion(6)),
            Space::with_width(Length::FillPortion(1))
        ];

        let mut outputs = column![].spacing(10);
        if let Some(error) = &self.error.0 {
            outputs = outputs.push(text(format!("Error: {}", error)).size(24))
        }
        if let Some(error) = &self.error.1 {
            outputs = outputs.push(text(format!("Error: {}", error)).size(24))
        }
        if let Some(key) = &self.key {
            outputs = outputs.push(text(format!("Main key: {}", key.to_upper_hex())).size(24))
        }
        if let Some(block) = &self.block {
            outputs = outputs.push(text(format!("Block: {}", block.to_upper_hex())).size(24))
        };
        if let Some(time) = &self.encode_time {
            outputs = outputs
                .push(text(format!("Encrypted in {} µs", time.whole_microseconds())).size(20))
        }
        if let Some(encoded_block) = &self.encoded_block {
            let mode = match &self.encode {
                true => "Cipher",
                false => "Plain",
            };
            outputs = outputs.push(
                text_input(
                    &encoded_block.to_upper_hex(),
                    &format!("{} text: {}", mode, encoded_block.to_upper_hex()),
                )
                .on_input(Event::Ignore)
                .size(24),
            );
        }
        let outputs = row![
            Space::with_width(Length::FillPortion(1)),
            container(outputs).width(Length::FillPortion(6)),
            Space::with_width(Length::FillPortion(1))
        ];

        let content = column![inputs, outputs]
            .align_items(Alignment::Center)
            .spacing(20);

        container(content).center_x().center_y().into()
    }
}

impl<'a, Message> From<BlockPage> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(key_page: BlockPage) -> Self {
        component(key_page)
    }
}
