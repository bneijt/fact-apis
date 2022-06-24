#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use std::path::{PathBuf, Path};


fn random_line_from(file_with_lines: &PathBuf) -> String {
    let mut path = Path::new(relative!("static")).join(file_with_lines);
    return String::from("hello")
}

#[get("/cat")]
fn cat() -> String {
    return random_line_from(&PathBuf::from("cat_facts.txt"));
}

#[get("/dog")]
fn dog() -> String{
    return random_line_from(&PathBuf::from("dog_facts.txt"));
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/facts", routes![dog, cat])
        .mount("/", FileServer::from(relative!("static")))
}
