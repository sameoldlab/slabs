use std::time::{Instant, SystemTime, UNIX_EPOCH};

use iced::widget::{button, column, row, text, text_input};
use iced::{
    Alignment, Color, Element, Event, Font, Length, Padding, Subscription, Task as Command, Theme,
    event, time,
};
use iced_layershell::Application;
use iced_layershell::reexport::{Anchor, KeyboardInteractivity, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings, StartMode};
use iced_layershell::to_layer_message;

pub fn main() -> Result<(), iced_layershell::Error> {
    let binded_output_name = std::env::args().nth(1);
    let start_mode = match binded_output_name {
        Some(output) => StartMode::TargetScreen(output),
        None => StartMode::Active,
    };

    Slabs::run(Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 30)),
            exclusive_zone: 30,
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode,
            keyboard_interactivity: KeyboardInteractivity::None,
            layer: Layer::Overlay,
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Slabs {
    time: String,
}

#[to_layer_message]
#[derive(Debug, Clone)]
#[doc = "Some docs"]
enum Message {
    IcedEvent(Event),
    TimeTick(time_format::TimeStamp),
}

impl Application for Slabs {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                time: "".to_string(),
            },
            Command::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("supply.same.Slabs")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent);
        time::every(time::Duration::from_millis(800)).map(|_| {
            let t = time_format::now().unwrap();
            Message::TimeTick(t)
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::IcedEvent(event) => {
                println!("hello {event:?}");
                Command::none()
            }
            Message::TimeTick(ts) => {
                self.time = time_format::strftime_local("%a %b %d, %I:%M:%S", ts).unwrap();

                Command::none()
            }
            _ => unreachable!(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let start = row![text("workspace").size(13), text("window").size(13),]
            .spacing(8)
            .width(Length::Fill);

        let center = row![text(&self.time).size(13),].width(Length::Fill);

        let end = row![
            text("media").size(13),
            text("sni").size(13),
            text("bat").size(13),
            text("vol").size(13),
        ]
        .spacing(8)
        .width(Length::Shrink);

        row![start, center, end]
            .align_y(Alignment::Center)
            .spacing(2)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Padding::from([0, 8]))
            .into()
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        use iced_layershell::Appearance;
        let palette = theme.extended_palette();
        Appearance {
            background_color: palette.background.base.color,
            text_color: palette.background.base.text,
        }
    }
    fn theme(&self) -> Self::Theme {
        Theme::CatppuccinMocha
    }
}
