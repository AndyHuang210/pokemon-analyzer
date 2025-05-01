#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Pokemon;

    fn mock_data() -> Vec<Pokemon> { // Generates mock Pokémon data for test cases.
        vec![
            Pokemon { name: "Test1".into(), type1: "Fire".into(), hp: 60, attack: 80, defense: 50, total: 190, legendary: false },
            Pokemon { name: "Test2".into(), type1: "Fire".into(), hp: 70, attack: 90, defense: 60, total: 220, legendary: true },
            Pokemon { name: "Test3".into(), type1: "Water".into(), hp: 50, attack: 60, defense: 70, total: 180, legendary: false },
        ]
    }

    #[test] // Test for average_stats_by_type() and Verifies that the average HP for "Fire" type Pokémon is calculated correctly.
    fn test_average_stats_by_type() {
        let result = average_stats_by_type(&mock_data());
        assert!(result.contains_key("Fire"));
        let avg = result.get("Fire").unwrap();
        assert_eq!((avg.hp * 2.0).round() as u32, 130);
    }

    #[test]
    fn test_top_pokemon_by_type() { // Ensures the function identifies the strongest Pokémon by total stats for each type.
        let result = top_pokemon_by_type(&mock_data());
        assert_eq!(result["Fire"].name, "Test2");
    }

    #[test]
    fn test_legendary_count() { // Computing the correct legendary count.
        let (leg, tot) = legendary_count(&mock_data());
        assert_eq!(leg, 1);
        assert_eq!(tot, 3);
    }
}