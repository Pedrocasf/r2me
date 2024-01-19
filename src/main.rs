use std::env;
use std::fs;

use crate::ClassLoader::Class::Class;
pub mod ClassLoader;
pub mod BaseTypes;
fn main() {
    let args:Vec<String> = env::args().collect();
    let main_class_path = &args[1];
    println!("{:}", main_class_path);
    let main_class_data = fs::read(main_class_path).expect("CLASS FILE NOT FOUND");
    println!("{:#X?}", main_class_data);
    let main_class_obj = Class::new(main_class_data);
    println!("{:#X?}", main_class_obj);
}
