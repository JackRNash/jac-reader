#![forbid(unsafe_code)]
use iced::keyboard::Event::KeyPressed;
use iced::keyboard::KeyCode;
use iced::{
    executor, Application, Clipboard, Command, Container, Element, Image, Length, Settings,
    Subscription,
};
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use std::cmp;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Cursor;
use std::path::Path;

fn main() {
    let mut unarchived = uncompress(Path::new("foo123.cbz"));
    let comic = parse_png(&mut unarchived);
    Viewer::run(Settings::with_flags(Message::ComicPage(Comic {
        pages: comic,
    })))
    .unwrap();
}

fn uncompress(path: &Path) -> zip::ZipArchive<File> {
    let ext = match path.extension() {
        None => panic!("not a cbr/cbz"),
        Some(e) => e,
    };
    match ext.to_str() {
        Some("cbz") => unzip(path),
        Some("cbr") => unrar(path),
        _ => panic!("unsupported file type"),
    }
}

// assumes path is a zip
// TODO: figure out more generic return type
fn unzip(path: &Path) -> zip::ZipArchive<File> {
    let archive = zip::ZipArchive::new(File::open(path).unwrap()).unwrap();
    // let mut v: Vec<Box<dyn Read>> = Vec::with_capacity(archive.len());
    // for i in 0..archive.len() {
    //     let file = archive.by_index(i).unwrap();
    //     v.push(file);
    // }
    archive
}

// assumes path is a rar
fn unrar(_path: &Path) -> zip::ZipArchive<File> {
    panic!("unimplemented!");
}

// TODO: abstract stream logic out, dispatch to parse_png or parse_jpg
//       based on file type during the stream.

fn parse_png(archive: &mut zip::ZipArchive<File>) -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.is_file() && file.name().ends_with(".png") {
            println!("{}", file.name());
            let decoder = png::Decoder::new(&mut file);
            let mut reader = decoder.read_info().unwrap();
            let height = reader.info().height;
            let width = reader.info().width;
            let mut buf = vec![0; reader.output_buffer_size()];
            reader.next_frame(&mut buf).unwrap();

            // let mut data = Vec::new();
            // file.read_to_end(&mut data).unwrap();
            // println!(
            //     "w:{} h:{} expected:{} actual:{}",
            //     width,
            //     height,
            //     width * height,
            //     buf.len() / 3 // 3 channels
            // );
            let img: image::RgbImage = ImageBuffer::from_raw(width, height, buf).unwrap();

            let mut cursor = Cursor::new(Vec::new());
            // img.write_to(&mut cursor, ImageOutputFormat::Png);
            DynamicImage::ImageRgb8(img)
                .write_to(&mut cursor, ImageOutputFormat::Png)
                .expect("Failed to encode image data to memory");
            // let foo = Handle::from_memory(cursor.into_inner());
            v.push(cursor.into_inner());
        }
    }
    v
}

#[derive(Debug, Clone)]
struct Comic {
    pages: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Viewer {
    comic: Comic,
    page: u32,
    fullscreen: bool,
}

#[derive(Debug, Clone)]
enum Message {
    ComicPage(Comic),
    BlankPage,
    KeyPress(iced_native::Event),
}

impl Application for Viewer {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Message;

    fn new(flags: Self::Flags) -> (Viewer, Command<Self::Message>) {
        match flags {
            Message::ComicPage(comic) => (
                Viewer {
                    comic: comic,
                    page: 0,
                    fullscreen: false,
                },
                Command::none(),
            ),
            _ => panic!("impossible"),
        }
    }

    fn title(&self) -> String {
        String::from("JAC Reader")
    }

    fn subscription(&self) -> Subscription<Message> {
        // TODO: find way to just get keyboard / key press events
        //       current method causes tons of uneccessary redraws
        iced_native::subscription::events().map(Message::KeyPress)
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
            Message::BlankPage => Command::none(),
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
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let comic_page = self.comic.pages[self.page as usize].clone();
        let handle = iced::widget::image::Handle::from_memory(comic_page);

        let image = Image::new(handle).width(Length::Fill).height(Length::Fill);

        Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
