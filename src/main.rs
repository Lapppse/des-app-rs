#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::widget::{button, column, component, container, text};
use iced::{
    alignment, executor, window, Alignment, Application, Command, Element, Length, Settings, Size,
    Theme,
};
use pages::{BlockPage, KeyPage};

mod pages;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            min_size: Some(Size {
                height: 600.0,
                width: 800.0,
            }),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };
    DesApp::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangePage(Pages),
    Input(String),
    ChangeTheme,
}

#[derive(Debug, Clone)]
pub enum Pages {
    Key,
    Block,
}

#[derive(Debug, Clone)]
pub struct DesApp {
    current_page: Pages, // FIXME: I can use simple iced Element or something instead of enum variant. Or not. I have to save state somewhere
    key_page: KeyPage,
    block_page: BlockPage,
    input: String,
    theme: Theme,
}

impl Application for DesApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                current_page: Pages::Key,
                key_page: KeyPage::default(),
                block_page: BlockPage::default(),
                input: String::from("what?"),
                theme: Theme::Dark,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let page = match self.current_page {
            Pages::Key => "Key",
            Pages::Block => "Block",
        };
        format!("NDTP DES | {}", page)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ChangePage(page) => self.current_page = page,
            Message::Input(input) => self.input = input,
            Message::ChangeTheme => {
                if self.theme == Theme::Dark {
                    self.theme = Theme::Light;
                } else {
                    self.theme = Theme::Dark;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let key_page = self.key_page.clone();
        let block_page = self.block_page.clone();
        let page_text = match self.current_page {
            Pages::Key => "Key page",
            Pages::Block => "Block page",
        };

        let content = column![
            button("Change appearance").on_press(Message::ChangeTheme),
            text(page_text).size(36),
            {
                match self.current_page {
                    Pages::Key => component(key_page),
                    Pages::Block => component(block_page),
                }
            },
            {
                let next_page_text = match self.current_page {
                    Pages::Key => "Block Page",
                    Pages::Block => "Key Page",
                };
                let next_page = match self.current_page {
                    Pages::Key => Pages::Block,
                    Pages::Block => Pages::Key,
                };
                container(
                    button(
                        text(format!("Go to {} ->", next_page_text))
                            .size(28)
                            .width(Length::Fill)
                            .height(Length::Shrink)
                            .vertical_alignment(alignment::Vertical::Center)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .on_press(Message::ChangePage(next_page))
                    .padding(20),
                )
                .padding(40)
                .height(Length::Fill)
                .align_y(alignment::Vertical::Bottom)
            }
        ]
        .padding(20)
        .spacing(40)
        .align_items(Alignment::Center);

        container(content).center_y().into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
