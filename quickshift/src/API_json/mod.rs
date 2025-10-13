use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputParams {
    pub email: String,
    pub ramos_pasados: Vec<String>,
    pub ramos_prioritarios: Vec<String>,
    pub horarios_preferidos: Vec<String>,
}

pub fn parse_json_input(json_str: &str) -> Result<InputParams, serde_json::Error> {
    serde_json::from_str::<InputParams>(json_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_input() {
        let json_data = r#"
        {
            "ramos_pasados": ["Matemáticas", "Física"],
            "ramos_prioritarios": ["Programación", "Álgebra"],
            "horarios_preferidos": ["08:00-10:00", "14:00-16:00"]
        }
        "#;

        let params = parse_json_input(json_data).unwrap();
        assert_eq!(params.ramos_pasados, vec!["Matemáticas", "Física"]);
        assert_eq!(params.ramos_prioritarios, vec!["Programación", "Álgebra"]);
        assert_eq!(params.horarios_preferidos, vec!["08:00-10:00", "14:00-16:00"]);
    }
}
