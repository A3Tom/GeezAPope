use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::fs;

use models::CenturyPerson;

mod models;

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<CenturyPerson>, Box<dyn Error>> {
    let file = fs::File::open(path).expect("Nae file mate! Get it sorted");
    let reader = BufReader::new(file);
    let pope_list = serde_json::from_reader(reader)?;
    Ok(pope_list)
}

fn main() {
    let file_path = "..\\..\\resources\\popes.json";
    let pope_list = read_user_from_file(&file_path).unwrap();
    println!("{:?}'", pope_list);
}