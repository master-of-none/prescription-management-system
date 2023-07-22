use chrono::prelude::*;
use std::io;

use mylib::*;

fn main() {
    let mut name = String::new();
    println!("Enter the patient's name");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    let mut dob = String::new();
    println!("Enter the patient's Date of birth (YYYY-MM-DD)");
    io::stdin()
        .read_line(&mut dob)
        .expect("Failed to read name");

    let dob = dob.trim();

    if !is_valid_dob(dob) {
        println!("Invalid date of birth given");
        return;
    }
    let age: Option<u64> = calculate_age(dob);

    let today = Local::now();
    let date = today.format("%Y-%m-%d").to_string();

    let patient = Patient::new(name, dob.to_string(), age, date);
    println!("Name: {}", patient.name);
    println!("DOB: {}", patient.dob);

    if let Some(age) = patient.age {
        println!("Age:{}", age);
    } else {
        println!("Age: Unknown")
    }
    println!("Date: {}", patient.date);

    let _ = connect_db();
}
