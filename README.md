# Docshund-RS

Tesseract OCR CLI app written with Rust, through tesseract-rs

## How to use

This readme assumes you're running this build on Linux; don't even ask how to get it working on Windows, I don't know.

WSL2 is your best friend if you're on Windows

1. Pull down the dependencies. At some point I'll have a fully accurate list but right now just install whatever `cargo build` whines about until the build succeeds.

2. Grab the language data files from [the Tesseract tessdata Github repo](https://github.com/tesseract-ocr/tessdata_fast)

3. `cargo run <image you want to OCR> <tessdata file path (JUST the path)> <language you're using>`

4. Et voila. You now have OCR barf in your console.
