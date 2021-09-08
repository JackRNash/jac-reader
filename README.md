# jac-reader
Just Another Comic Reader


### What is this?
A personal project to learn Rust. JAC Reader will read in comics in `.cbz` or `.cbr` format containing `.png` or `.jpg` images. It will support single page and book mode, as well as full screen. Stretch goals including having a system for persistent meta data like keeping track of the page you're on.

So far, it can read `.cbz` comics containg `.pngs` and toggle between fullscreen mode with the `f` key.

It was inspired by YAC (Yet another comic) reader and library. YAC struggled on higher resolution screens and had issues with comics that left a page blank on the physical copy but just omitted a page on the digital one. I hope for JAC to remedy these issues.

#### Note
Things are currently in a proof of concept phase. I'm trying to figure how certain pieces fit together (e.g. decompressing the file and then parsing the images from it). Once I'm convinced I understand these pieces well enough, I'll refactor / rewrite things to more maintaible. Right now the code is messy, although I try to do a pass over things after each new feature to keep it in check.


### Running it
Right now there's no way to read in a specific file (it's hardcoded at the moment, see note above), but you would run the application with a simple
```
cargo run --release
```
