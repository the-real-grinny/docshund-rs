use tesseract::*;
use std::env::*;

const DEBUG:bool = true;

fn print_type_of_thing<T>(_: &T) {
    println!("This is the type of that result: {}", std::any::type_name::<T>());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target = &args[1];
    let datapath = &args[2];
    let language = &args[3];
    let scanner = Tesseract::new(Some(datapath),Some(language)).expect("Couldn't create Tesseract object");
    if DEBUG { print_type_of_thing(&scanner); }
    let mut test = scanner.set_image(target).expect("Couldn't find image");
    if DEBUG { print_type_of_thing(&test); }
    println!("This is the test text: \n{}",test.get_text().expect("Couldn't extract text"));
}
