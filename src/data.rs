// Purpose: Loads and represents Pokémon data from a CSV file.

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
    pub hp: u32, // Base HP stat
    pub attack: u32,  // Base Attack stat
    pub defense: u32,  // Base Defense stat
    pub s_attack: u32,  // Base Special Attack stat
    pub s_defense: u32,  // Base Special Defense stat
    pub speed: u32,  // Base Speed stat
    pub r#type: String,
    pub evo_set: String,
    pub info: String,
}


pub fn load_data(filename: &str) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let file = File::open(filename)?; 
    let reader = BufReader::new(file); // Wraps the file in a buffered reader for efficient reading.
    let mut rdr = csv::Reader::from_reader(reader);
    let mut pokedex = Vec::new();  //Initializes an empty Vec to collect Pokémon records.
    

// Iterates over the CSV rows, trying to deserialize each row into a Pokemon struct.
// Uses the ? operator to return an error if deserialization fails.
// Appends each valid record to the pokedex vector.
    for result in rdr.deserialize() {
        let record: Pokemon = result?;
        pokedex.push(record);
    }

    Ok(pokedex)
}
