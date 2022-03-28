use tesseract::*;
use std::env::*;

const DEBUG:bool = false; // toggles all debug output, like print_type_of_thing

const VERSION: &str = "0.1.1";

// used to debug type unwraps right
// since Tesseract -loves- using <Result> types
fn print_type_of_thing<T>(_: &T) {
    println!("This is the type of that result: {}", std::any::type_name::<T>());
}


fn preface_text() {
    println!("========================");
    println!("docshund-rs v{}",VERSION);
    println!("========================");
}


fn end_text() {
    println!("========================");
    println!("End output");
    println!("========================");
}


fn print_loop() {
    preface_text();
    let args: Vec<String> = std::env::args().collect();
    let target = &args[1]; // image file you're OCRing
    let data_path = &args[2]; // the directory that holds the tesseract language data
    let language = &args[3]; // the specific language you're using
    let scanner = Tesseract::new(Some(data_path),Some(language)).expect("Couldn't create Tesseract object");
    if DEBUG { print_type_of_thing(&scanner); }
    let mut image_text = scanner.set_image(target).expect("Couldn't find image");
    if DEBUG { print_type_of_thing(&image_text); }
    println!("{}",image_text.get_text().expect("Couldn't extract text"));
    end_text();
}


fn main() {
    print_loop();    
}
