use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct World {
    pub width: u32,
    pub height: u32,
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json; // Utilisation de JSON pour sérialiser/désérialiser

    #[test]
    fn test_world_creation() {
        // Vérifie que la structure est initialisée correctement
        let world = World { width: 100, height: 200 };
        assert_eq!(world.width, 100);
        assert_eq!(world.height, 200);
    }

    #[test]
    fn test_world_serialization_to_json() {
        // Crée une structure World
        let world = World { width: 50, height: 75 };

        // Sérialise la structure en JSON
        let serialized = serde_json::to_string(&world).unwrap();

        // Vérifie que le JSON est correctement formaté
        assert_eq!(serialized, r#"{"width":50,"height":75}"#);
    }

    #[test]
    fn test_world_deserialization_from_json() {
        // Chaîne JSON représentative d'un "World"
        let json_data = r#"{"width":30,"height":40}"#;

        // Désérialise le JSON en un objet World
        let deserialized: World = serde_json::from_str(json_data).unwrap();

        // Vérifie que les valeurs sont celles attendues
        assert_eq!(deserialized.width, 30);
        assert_eq!(deserialized.height, 40);
    }

    #[test]
    fn test_world_round_trip_serialization() {
        // Crée une structure initiale
        let world = World { width: 128, height: 256 };

        // Sérialise en JSON
        let serialized = serde_json::to_string(&world).unwrap();

        // Désérialise à partir du JSON
        let deserialized: World = serde_json::from_str(&serialized).unwrap();

        // Vérifie que la structure désérialisée est égale à l'originale
        assert_eq!(deserialized, world);
    }
}