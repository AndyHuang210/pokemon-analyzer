// Purpose: Program entry point, coordinates data loading and analysis, and prints results.

mod data;
mod analysis;
mod test;

use crate::data::load_data;
use crate::analysis::{average_stats_by_type, top_pokemon_by_type, legendary_count};

// Load the Pokémon data.
fn main() {
    let pokedex = load_data("pokedex.csv").expect("Failed to load data");

// Display average stats by Pokémon type.
    println!("Average Stats by Type:");
    for (ptype, (hp, atk, def, count)) in average_stats_by_type(&pokedex) {
        println!(
            "{} ({} Pokémon): HP={}, ATK={}, DEF={}",
            ptype, count, hp, atk, def
        );
    }
// Display the strongest Pokémon (by total stats) in each type.
    println!("Top Pokémon by Type:");
    for (ptype, poke) in top_pokemon_by_type(&pokedex) {
        let total_stats = poke.hp + poke.attack + poke.defense + poke.s_attack + poke.s_defense + poke.speed;
        println!("{}: {} with total stats {}", ptype, poke.name, total_stats);
    }
}
