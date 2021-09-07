# jac-reader
Just Another Comic Reader


### What is this?
A personal project to learn Rust. JAC Reader will read in comics in `.cbz` or `.cbr` format containing `.png` or `.jpg` images. It will support single page and book mode, as well as full screen. Stretch goals including having a system for persistent meta data like keeping track of the page you're on.

So far, it can read `.cbz` comics containg `.pngs` and toggle between fullscreen mode with the `f` key.

It was inspired by YAC (Yet another comic) reader and library. YAC struggled on higher resolution screens and had issues with comics that left a page blank on the physical copy but just omitted a page on the digital one. I hope for JAC to remedy these issues.
