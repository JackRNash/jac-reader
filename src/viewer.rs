#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub mod viewing {
  use iced::keyboard::Event::KeyPressed;
  use iced::keyboard::KeyCode;
  use iced::{
    executor, Application, Clipboard, Command, Container, Element, Image, Length, Settings,
    Subscription,
  };
  use std::cmp;
  use std::convert::TryFrom;

  #[derive(Debug, Clone)]
  pub struct Comic {
    pages: Vec<Vec<u8>>,
  }

  #[derive(Debug)]
  pub struct Viewer {
    comic: Comic,
    page: u32,
    fullscreen: bool,
  }

  #[derive(Debug, Clone)]
  pub enum Message {
    ComicPage(Comic),
    KeyPress(iced_native::Event),
  }

  // message to initialize application state given a comic
  #[must_use]
  pub fn default_settings(comic: Vec<Vec<u8>>) -> iced::Settings<Message> {
    Settings::with_flags(Message::ComicPage(Comic { pages: comic }))
  }

  impl Application for Viewer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Message;

    fn new(flags: Self::Flags) -> (Viewer, Command<Self::Message>) {
      match flags {
        Message::ComicPage(comic) => (
          Viewer {
            comic,
            page: 0,
            fullscreen: false,
          },
          Command::none(),
        ),
        Message::KeyPress(..) => panic!("impossible"),
      }
    }

    fn title(&self) -> String {
      String::from("JAC Reader")
    }

    fn subscription(&self) -> Subscription<Message> {
      let fun = |event: iced_native::Event, _status: iced_native::event::Status| match event {
        iced_native::Event::Keyboard { .. } => Some(Message::KeyPress(event)),
        _ => None,
      };
      iced_native::subscription::events_with(fun)
    }

    fn mode(&self) -> iced::window::Mode {
      if self.fullscreen {
        iced::window::Mode::Fullscreen
      } else {
        iced::window::Mode::Windowed
      }
    }

    fn update(
      &mut self,
      message: Self::Message,
      _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
      match message {
        Message::ComicPage(comic) => {
          self.comic = comic;
          self.page = 0;
          Command::none()
        }
        Message::KeyPress(iced_native::Event::Keyboard(keyevent)) => {
          match keyevent {
            // on right press
            KeyPressed {
              key_code: KeyCode::Right,
              ..
            } => {
              let max_len = u32::try_from(self.comic.pages.len() - 1).unwrap();
              self.page = cmp::min(self.page + 1, max_len);
            }
            // on left press
            KeyPressed {
              key_code: KeyCode::Left,
              ..
            } => {
              // unsigned so have to be careful about overflow in comparison
              self.page = cmp::max(self.page, 1) - 1;
            }
            // on f
            KeyPressed {
              key_code: KeyCode::F,
              ..
            } => {
              // toggle
              self.fullscreen = !self.fullscreen;
            }
            // don't care about other button presses
            _ => (),
          }
          Command::none()
        }

        // not using any other events for now (mouse move etc)
        Message::KeyPress(..) => Command::none(),
      }
    }

    fn view(&mut self) -> Element<Self::Message> {
      let comic_page = self.comic.pages[self.page as usize].clone();
      let handle = iced::widget::image::Handle::from_memory(comic_page);

      let image = Image::new(handle).width(Length::Fill).height(Length::Fill);

      Container::new(image)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Container)
        .center_x()
        .center_y()
        .into()
    }
  }

  mod style {
    use iced::{container, Color};

    pub struct Container;

    impl container::StyleSheet for Container {
      fn style(&self) -> container::Style {
        container::Style {
          background: Color::from_rgb8(0x36, 0x39, 0x3F).into(),
          ..container::Style::default()
        }
      }
    }
  }
}
