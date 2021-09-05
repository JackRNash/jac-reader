#![forbid(unsafe_code)]
use iced::{
    executor, Application, Clipboard, Command, Container, Element, Image, Length, Settings,
};
use std::fs::File;
use std::path::Path;

fn main() {
    let mut unarchived = uncompress(Path::new("redfoo.cbz"));
    let comic = parse_png(&mut unarchived);
    Viewer::run(Settings::with_flags(ComicMessage::ComicPage(Comic {
        pages: comic,
    })));
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
    let unzipped = zip::ZipArchive::new(File::open(path).unwrap()).unwrap();
    // let mut v: Vec<Box<dyn Read>> = Vec::with_capacity(unzipped.len());
    // for i in 0..unzipped.len() {
    //     let file = unzipped.by_index(i).unwrap();
    //     v.push(file);
    // }
    unzipped
}

// assumes path is a rar
fn unrar(_path: &Path) -> zip::ZipArchive<File> {
    panic!("unimplemented!");
}

fn parse_png(unarchived: &mut zip::ZipArchive<File>) -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = Vec::with_capacity(unarchived.len());
    for i in 0..unarchived.len() {
        let file = unarchived.by_index(i).unwrap();
        if file.is_file() {
            println!("{}", file.name());
            let decoder = png::Decoder::new(file);
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size()];
            reader.next_frame(&mut buf).unwrap();
            v.push(buf);
        }
    }
    v
}

#[derive(Debug, Clone)]
struct Comic {
    pages: Vec<Vec<u8>>,
}

#[derive(Debug)]
enum Viewer {
    Loading,
    Loaded { comic: Comic },
    Errored,
}

#[derive(Debug, Clone)]
enum ComicMessage {
    ComicPage(Comic),
    BlankPage,
}

impl Application for Viewer {
    type Executor = executor::Default;
    type Message = ComicMessage;
    type Flags = ComicMessage;

    fn new(flags: Self::Flags) -> (Viewer, Command<Self::Message>) {
        match flags {
            ComicMessage::ComicPage(comic) => (Viewer::Loaded { comic: comic }, Command::none()),
            ComicMessage::BlankPage => (Viewer::Loading, Command::none()),
        }
    }

    fn title(&self) -> String {
        String::from("JAC Reader")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            ComicMessage::ComicPage(comic) => {
                *self = Viewer::Loaded { comic };
                Command::none()
            }
            ComicMessage::BlankPage => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let handle = match self {
            Viewer::Loaded { comic } => {
                let page = comic.pages[0].clone();
                iced::widget::image::Handle::from_pixels(931, 600, page)
                // iced::widget::image::Handle::from_memory(page[..4000].to_vec())
                // iced::widget::image::Handle::from("blue.png")
            }
            Viewer::Errored => iced::widget::image::Handle::from_path("error.png"),
            Viewer::Loading => iced::widget::image::Handle::from_path("loading.png"),
        };
        let image = Image::new(handle).width(Length::Fill).height(Length::Fill);

        Container::new(image)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}