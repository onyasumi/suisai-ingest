use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::io::ErrorKind::NotFound;

pub struct Date {
    pub year: String,
    pub month_day: String
}

const MONTH: &[&str] = &[
    "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"
];

pub fn get_date(file: &Path) -> Result<Date, Box<dyn Error>> {

    if !file.is_file() {
        return Err(format!("Invalid path: {} not exist or is not a file", file.to_str().unwrap_or("")).into())
    }

    let result = String::from_utf8(match Command::new("exiftool")
        .arg("-DateTimeOriginal")
        .arg("-fast2")
        .arg("-s3")
        .arg(file.to_str().unwrap_or(""))
        .output() {
        Ok(result) => result.stdout,
        Err(e) => {
            return if e.kind() == NotFound {
                Err("exiftool not found".into())
            } else {
                Err(e.into())
            }
        }
    })?;

    let date_arr: Box<[&str]> = result.split([':', ' ']).take(3).collect();

    if date_arr.len() < 3 {
        return Err("DateTimeOriginal field missing or corrupted".into())
    }

    let date: Date = Date {
        year: date_arr[0].to_string(),
        month_day: format!("{} {}", MONTH[date_arr[1].parse::<usize>()?], date_arr[2])
    };


    Ok(date)

}