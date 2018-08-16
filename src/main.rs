extern crate chrono;
extern crate termion;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

enum Tier {
    One,
    Two,
	Three,
}

struct Person {
	name: String,
	tier: Tier,
    pmoc: [String],
}

static CONTACTS: HashMap<Person, DateTime<Local>> = HashMap::new();

fn main() {
    println!("Welcome to Conma, what would you like to do?");

	CONTACTS.insert(Person { name: "Farid M".to_string(), tier: Tier::One, pmoc: *["Whatsapp".to_string()] }, Local::now());
	CONTACTS.insert(Person { name: "Richard L".to_string(), tier: Tier::One, pmoc: *["Telegram".to_string()] }, Local::now());
	
	list_contacts();
}

fn list_contacts() {
	for (person, date) in &CONTACTS {
		println!("{}: \"{}\"", person, date);
	}
}

fn update_contact((name, tier, pmoc): (String, String, String)) {
	if CONTACTS.insert(person, Local::now()) != None {
		println!("Contact updated successfully.");
	} else {
		println!("Contact does not exist.")
	}
}

fn update_contact_tier(name: String, tier: String) {
	
}

fn import_contacts(file: String) {

}