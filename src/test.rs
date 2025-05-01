#[cfg(test)]
mod tests {
    use crate::data::Pokemon; // Import Pokemon struct
    use crate::analysis::{average_stats_by_type}; // Keep only the used import

    fn mock_data() -> Vec<Pokemon> {
        vec![
            Pokemon {
                id: 1,
                name: "Test1".into(),
                r#type: "Fire".into(),
                hp: 60,
                attack: 80,
                defense: 50,
                s_attack: 70,
                s_defense: 65,
                speed: 90,
                height: 1.2,
                weight: 35.0,
                evo_set: String::new(),
                info: String::new(),
            },
            // Add more Pokemon data here...
        ]
    }

    #[test]
    fn test_average_stats_by_type() {
        let data = mock_data();
        let result = average_stats_by_type(&data);

        assert!(result.contains_key("Fire"));
        
        let fire_stats = result["Fire"];
        assert_eq!(fire_stats.0, 60); 
    }
}

