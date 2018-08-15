extern crate chrono;

use chrono::prelude::*;
use std::collections::HashMap;

enum Tier {
	one,
	two,
	three,
}

struct Person {
	name: String,
	tier: Tier,
}

static contacts:HashMap<Person, DateTime<Local>> = HashMap::new();

fn main() {
    println!("Hello, world!");

	contacts.insert(Person { name: "Farid M", tier: Tier::one }, Local::now());
	contacts.insert(Person { name: "Richard L", tier: Tier::one }, Local::now());
	
	listContacts();
}

fn listContacts() {
	for (person, date) in &contacts {
		println!("{}: \"{}\"", person, date);
	}
}

fn updateContact(person: Person) {
	if contacts.insert(person, Local::now()) != None {
		println!("Contact updated successfully.");
	} else {
		println!("Contact does not exist.")
	}
}

fn updateContactTier(name: String, tier: String) {
	
}