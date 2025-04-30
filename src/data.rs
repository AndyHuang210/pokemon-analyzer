use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub height: f32,
    pub weight: f32,
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub s_attack: u32,
    pub s_defense: u32,
    pub speed: u32,
    pub r#type: String,
    pub evo_set: String,
    pub info: String,
}


pub fn load_data(filename: &str) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::Reader::from_reader(reader);
    let mut pokedex = Vec::new();

    for result in rdr.deserialize() {
        let record: Pokemon = result?;
        pokedex.push(record);
    }

    Ok(pokedex)
}
