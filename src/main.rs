extern crate chrono;
extern crate termion;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io;

enum Tier {
    One,
    Two,
	Three,
}

struct Person {
	name: String,
	tier: Tier,
    pmoc: Vec<String>,
    timeZone: i8, //UTC
}

static CONTACTS: HashMap<Person, Date<Local>> = HashMap::new();
static CURRENT_TIME_ZONE: i8 = undefined;

fn check_timezone() {
    let acceptableRange = -12..12;
    if CURRENT_TIME_ZONE == undefined {
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

	CONTACTS.insert(Person { name: "Farid M".to_string(), tier: Tier::One,
        pmoc: *["WA".to_string()], timeZone: -5 }, Local::now().date());
	CONTACTS.insert(Person { name: "Richard L".to_string(), tier: Tier::One,
        pmoc: *["TG".to_string()], timeZone: -5  }, Local::now().date());

	list_contacts();
}

fn list_contacts() {
	for (person, date) in &CONTACTS {
		println!("{}: \"{}\"", person, date);
	}
}

fn update_contact((name, tier, pmoc): (String, String, String)) {
	if CONTACTS.insert(person, Local::now().date()) != None {
		println!("Contact updated successfully.");
	} else {
		println!("Contact does not exist.")
	}
}

fn update_contact_tier(name: String, tier: String) {
	
}

fn import_contacts(file: String) {

}
