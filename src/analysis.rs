use crate::data::Pokemon;
use std::collections::HashMap;

pub fn average_stats_by_type(pokedex: &[Pokemon]) -> HashMap<String, (u32, u32, u32, u32)> {
    let mut totals: HashMap<String, (u32, u32, u32, u32)> = HashMap::new();
    let mut counts: HashMap<String, u32> = HashMap::new();

    for p in pokedex {
        let entry = totals.entry(p.r#type.clone()).or_insert((0, 0, 0, 0));
        entry.0 += p.hp;
        entry.1 += p.attack;
        entry.2 += p.defense;
        entry.3 += 1;

        *counts.entry(p.r#type.clone()).or_insert(0) += 1;
    }

    for (ptype, total) in totals.iter_mut() {
        let count = counts.get(ptype).unwrap_or(&1);
        total.0 /= *count;
        total.1 /= *count;
        total.2 /= *count;
        total.3 = *count;
    }

    totals
}
pub fn top_pokemon_by_type(pokedex: &[Pokemon]) -> HashMap<String, Pokemon>{
    let mut top: HashMap<String, Pokemon> = HashMap::new();
    for p in pokedex {
        let p_type = p.r#type.clone();
        let p_total = p.hp + p.attack + p.defense + p.s_attack + p.s_defense + p.speed;

        top.entry(p_type.clone())
            .and_modify(|curr| {
                let curr_total = curr.hp + curr.attack + curr.defense + curr.s_attack + curr.s_defense + curr.speed;
                if p_total > curr_total {
                    *curr = p.clone();
                }
            })
            .or_insert_with(|| p.clone());
    }

    top
}
