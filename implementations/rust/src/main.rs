use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::fs;
use rand::Rng;

mod models;
use models::CenturyPerson;

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<CenturyPerson>, Box<dyn Error>> {
    let file = fs::File::open(path).expect("Nae file mate! Get it sorted");
    let reader = BufReader::new(file);
    let pope_list = serde_json::from_reader(reader)?;
    Ok(pope_list)
}

fn main() {
    let file_path = "resources\\popes.json";
    let pope_list = read_user_from_file(&file_path).unwrap();

    let mut rng = rand::thread_rng();
    let pontiff_number = rng.gen_range(1..pope_list.len());

    println!("{:?}", pope_list[pontiff_number]);
}