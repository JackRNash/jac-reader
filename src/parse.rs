#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

pub mod parsing {
  use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
  use std::fs::File;
  use std::io::Cursor;
  use std::path::Path;

  #[must_use]
  /// # Panics
  /// Will panic on unsupported file types (for now)
  /// TODO: implement better handling (possibly change return to result)
  pub fn uncompress(path: &Path) -> zip::ZipArchive<File> {
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
    zip::ZipArchive::new(File::open(path).unwrap()).unwrap()
  }

  // assumes path is a rar
  fn unrar(_path: &Path) -> zip::ZipArchive<File> {
    panic!("unimplemented!");
  }

  // TODO: abstract stream logic out, dispatch to parse_png or parse_jpg
  //       based on file type during the stream.

  /// # Panics
  /// Panics on error parsing image
  /// TODO: find graceful solution
  pub fn parse_png(archive: &mut zip::ZipArchive<File>) -> Vec<Vec<u8>> {
    let mut v: Vec<Vec<u8>> = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
      let mut file = archive.by_index(i).unwrap();
      if file.is_file()
        && file
          .name()
          .rsplit('.')
          .next()
          .map(|ext| ext.eq_ignore_ascii_case("png"))
          == Some(true)
      {
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
}
