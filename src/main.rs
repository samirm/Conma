extern crate chrono;
// extern crate termion;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;
use std::fmt;

#[derive(Hash)]
enum Tier {
    One,
    Two,
	Three,
}

#[derive(Hash)]
struct Person {
	name: String,
	tier: Tier,
    pmoc: Vec<String>,
    timeZone: i8, //UTC
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Person) -> bool {
        self.name == other.name
    }
}
impl Eq for Person {}

static CONTACTS: HashMap<Person, Date<Local>> = HashMap::new();
static CURRENT_TIME_ZONE: i8 = 0;

fn check_timezone() {
    let acceptableRange = -12..12;
    let mut input = String::new();
    if CURRENT_TIME_ZONE == 0 {
        println!("What UTC offset are you in?");

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read line.");

            CURRENT_TIME_ZONE = match input.trim().parse() {
                Ok(num) => num,
                Err(..) => println!("Please enter a negative or positive integer (e.g. -12 .. 12)"),
            };
        }
    }
}

fn main() {
    println!("Welcome to Conma, what would you like to do?");
    println!("\n 1. Update contact(s).");
    println!("\n 2. Import contact(s).");
    println!("\n 3. View calendar.");
    println!("\n 0. Quit.");

    let vec = Vec::new();
    vec.push("WA".to_string());

    let vec2 = Vec::new();
    vec.push("TG".to_string());

	CONTACTS.insert(Person { name: "Farid M".to_string(), tier: Tier::One,
        pmoc: vec, timeZone: -5 }, Local::now().date());
	CONTACTS.insert(Person { name: "Richard L".to_string(), tier: Tier::One,
        pmoc: vec2, timeZone: -5  }, Local::now().date());

	list_contacts();
}

fn list_contacts() {
	for (person, date) in &CONTACTS {
		println!("{}: \"{}\"", person, date);
	}
}

fn update_contact((name, tier, pmoc, timeZone): (String, String, String, String)) {
    let tp: Person;
	if CONTACTS.insert(tp, Local::now().date()) != None {
		println!("Contact updated successfully.");
	} else {
		println!("Contact does not exist.")
	}
}

fn update_contact_tier(name: String, tier: String) {
	
}

fn import_contacts(file: String) {

}
