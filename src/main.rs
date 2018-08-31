extern crate chrono;
// extern crate termion;

use chrono::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::fmt;
use std::fs;

#[derive(Hash)]
enum Tier {
    One,
    Two,
	Three,
}

#[derive(Hash)]
struct Person<'a> {
	name: &'a str,
	tier: Tier,
    pmoc: Vec<&'a str>,
    time_zone: i8, //UTC
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.name)
    }
}

impl<'a> PartialEq for Person<'a> {
    fn eq(&self, other: &Person) -> bool {
        self.name == other.name
    }
}
impl<'a> Eq for Person<'a> {}

fn check_timezone(current_timezone: i8) -> i8 {
//    let timezone_range = -12..12;
    let mut input = String::new();
    let mut new_timezone = 0;
    if current_timezone == 0 {
        println!("\nWhat UTC offset are you in?");

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            let trimmed = input.trim();

            match trimmed.parse::<i8>() {
                Ok(num) => {
                    new_timezone = num;
                    break;
                },
                Err(..) => println!("Please enter a negative or positive integer (e.g. -12 .. 12)"),
            };
        }
    }
    return new_timezone;
}

fn main() {
    let mut contacts: HashMap<Person, Date<Local>> = HashMap::new();
    let mut current_timezone: i8 = 0;

    println!("Welcome to Conma, what would you like to do?");
    current_timezone = check_timezone(current_timezone);

    println!("\n1. Update contact(s).");
    println!("2. Import contact(s).");
    println!("3. View calendar.");
    println!("0. Quit.\n");

    let vec = vec!["WA"];
    let vec2 = vec!["TG"];

    contacts.insert(Person { name: "Farid M", tier: Tier::One,
        pmoc: vec, time_zone: -5 }, Local::now().date());
    contacts.insert(Person { name: "Richard L", tier: Tier::One,
        pmoc: vec2, time_zone: -5  }, Local::now().date());
    list_contacts(contacts);
    import_contacts();
}

fn list_contacts(contacts: HashMap<Person, Date<Local>>) {
	for (person, date) in &contacts {
		println!("{}: \"{}\"", person, date);
	}
}

fn update_contact(contacts: &mut HashMap<Person, Date<Local>>) {
    let tp = Person {
        name: "",
        tier: Tier::Three,
        pmoc: vec![],
        time_zone: -3,
    };
	if contacts.insert(tp, Local::now().date()).is_none() {
		println!("Contact updated successfully.");
	} else {
		println!("Contact does not exist.")
	}
}

fn update_contact_tier(name: &str, tier: Tier) {
	unimplemented!()
}

fn import_contacts(){
    println!("contents: {}", read_file("data/result.json"));
}

fn read_file(filepath: &str) -> String {
//    let file = File::open(filepath).expect("could not open file.");
    return fs::read_to_string(filepath).expect("reading file went wrong.");
}