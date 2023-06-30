use mylib::*;

fn main() {
    let name: String = String::from("ABC");
    let dob: String = String::from("1997-05-21");
    let age: Option<u64> = calculate_age(dob.as_str());
    let date: String = String::from("2023-06-30");

    let patient = Patient::new(name, dob, age, date);
    println!("Name: {}", patient.name);
    println!("DOB: {}", patient.dob);

    if let Some(age) = patient.age {
        println!("Age:{}", age);
    } else {
        println!("Age: Unknown")
    }
    println!("Date: {}", patient.date);
}
