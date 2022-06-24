#[macro_use]
extern crate rocket;

use rand::seq::IteratorRandom; // 0.7.3
use rocket::fs::{relative, FileServer};
use rocket::serde::json::Json;
use std::path::{Path, PathBuf};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

/// Return a random line from a given file in static
fn random_line_from(file_with_lines: &PathBuf) -> Option<String> {
    let path = Path::new(relative!("static")).join(file_with_lines);
    let f = BufReader::new(File::open(path).unwrap());

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    lines
        .choose(&mut rand::thread_rng())
}

#[get("/cat")]
fn cat() -> Option<Json<String>> {
    random_line_from(&PathBuf::from("data/cat_facts.txt")).map(Json)
}

#[get("/dog")]
fn dog() -> Option<Json<String>> {
    random_line_from(&PathBuf::from("data/dog_facts.txt")).map(Json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/facts", routes![dog, cat])
        .mount("/", FileServer::from(relative!("static")))
}
