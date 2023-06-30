use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug)]
pub struct Patient {
    pub name: String,
    pub dob: String,
    pub age: Option<u64>,
    pub date: String,
}

impl Patient {
    pub fn new(name: String, dob: String, age: Option<u64>, date: String) -> Patient {
        Patient {
            name,
            dob,
            age,
            date,
        }
    }
}

pub fn calculate_age(dob: &str) -> Option<u64> {
    let d_o_b = chrono::NaiveDate::parse_from_str(dob, "%Y-%m-%d").ok()?;

    let current_date = Local::now().naive_local();
    let age = current_date.year() - d_o_b.year();

    let adjusted_age = if current_date.month() < d_o_b.month()
        || (current_date.month() == d_o_b.month() && current_date.day() < d_o_b.day())
    {
        age - 1
    } else {
        age
    };

    Some(adjusted_age as u64)
}

pub fn is_valid_dob(dob: &str) -> bool {
    if let Ok(parsed_date) = NaiveDate::parse_from_str(dob, "%Y-%m-%d") {
        let month = parsed_date.month();

        if month < 1 || month > 12 {
            return false;
        }

        let day = parsed_date.day();

        if day < 1 || day > 31 {
            return false;
        }

        true
    } else {
        false
    }
}
