use base64::{engine::general_purpose, Engine as _};
use std::time::Instant;

pub struct App {
    pub battlenet_id: String,
    pub use_lowercase: bool,
    pub version: String,
    pub last_input: Instant,
    pub copy_feedback: Option<Instant>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            battlenet_id: String::new(),
            use_lowercase: false,
            version: "retail".to_string(),
            last_input: Instant::now(),
            copy_feedback: None,
        }
    }

    pub fn reset_input(&mut self) {
        self.battlenet_id.clear();
    }

    pub fn toggle_version(&mut self) {
        self.version = match self.version.as_str() {
            "classic" => "retail".to_string(),
            "retail" => "classic".to_string(),
            _ => "retail".to_string(),
        };
    }

    pub fn add_char(&mut self, c: char) {
        self.battlenet_id.push(c);
        self.last_input = Instant::now();
    }

    pub fn remove_char(&mut self) {
        self.battlenet_id.pop();
    }

    pub fn sanitize_input(&mut self) {
        self.battlenet_id
            .retain(|c| c.is_ascii_alphanumeric() || c == '#');
    }

    pub fn is_valid_battlenet_id(&self) -> bool {
        if self.battlenet_id.is_empty() {
            return false;
        }

        let parts: Vec<&str> = self.battlenet_id.split('#').collect();
        if parts.len() != 2 {
            return false;
        }

        let name_part = parts[0];
        let number_part = parts[1];

        // Name part must not be empty and contain only alphanumeric characters
        if name_part.is_empty() || !name_part.chars().all(|c| c.is_ascii_alphanumeric()) {
            return false;
        }

        // Number part must be at least 4 digits long and contain only digits
        number_part.len() >= 4 && number_part.chars().all(|c| c.is_ascii_digit())
    }

    pub fn generate_code(&self) -> Result<String, String> {
        let addon_config = self.get_addon_config()?;

        let mut input = self.battlenet_id.clone();
        if self.use_lowercase {
            input = input.to_lowercase();
        }
        input.push_str(&addon_config);

        Ok(general_purpose::STANDARD.encode(input))
    }

    pub fn validate_code(&self, encoded_string: &str) -> bool {
        match self.decode_import_string(encoded_string) {
            Ok(decoded) => {
                let battlenet_lower = self.battlenet_id.to_lowercase();
                let decoded_lower = decoded.to_lowercase();

                let wa4_config = "ctdveirvrtdice";
                let wa5_config = "vridtcetvrdice";

                let expected_combinations = vec![
                    format!("{}{}", battlenet_lower, wa4_config),
                    format!("{}{}", self.battlenet_id, wa4_config),
                    format!("{}{}", battlenet_lower, wa5_config),
                    format!("{}{}", self.battlenet_id, wa5_config),
                ];

                for expected in expected_combinations {
                    if decoded_lower == expected.to_lowercase() || decoded == expected {
                        return true;
                    }
                }
                false
            }
            Err(_) => false,
        }
    }

    fn get_addon_config(&self) -> Result<String, String> {
        match self.version.as_str() {
            "retail" | "classic" => Ok("vridtcetvrdice".to_string()),
            _ => Err("Invalid version".to_string()),
        }
    }

    fn decode_import_string(&self, encoded_string: &str) -> Result<String, String> {
        let cleaned: String = encoded_string
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || *c == '+' || *c == '/' || *c == '=')
            .collect();

        match general_purpose::STANDARD.decode(&cleaned) {
            Ok(bytes) => String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {}", e)),
            Err(e) => Err(format!("Base64 decode error: {}", e)),
        }
    }

    pub fn get_wa_configs(&self) -> (String, String) {
        let wa4_part1: String = vec![99, 116, 100, 118, 101, 105]
            .into_iter()
            .map(|c| c as u8 as char)
            .collect();
        let wa4_part2: String = vec![114, 118, 114, 116, 105, 100, 99, 101]
            .into_iter()
            .map(|c| c as u8 as char)
            .collect();
        let wa4 = format!("{}{}", wa4_part1, wa4_part2);

        let wa5_part1: String = vec![118, 114, 105, 100, 116, 99]
            .into_iter()
            .map(|c| c as u8 as char)
            .collect();
        let wa5_part2: String = vec![101, 116, 118, 114, 100, 105, 99, 101]
            .into_iter()
            .map(|c| c as u8 as char)
            .collect();
        let wa5 = format!("{}{}", wa5_part1, wa5_part2);

        (wa4, wa5)
    }
}
