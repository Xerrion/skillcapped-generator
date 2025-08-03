use base64::{engine::general_purpose, Engine as _};
use std::time::Instant;

const WA4_CONFIG: &str = "ctdveirvrtdice";
const WA5_CONFIG: &str = "vridtcetvrdice";
const DEFAULT_CONFIG: &str = "vridtcetvrdice";
const MIN_NUMBER_LENGTH: usize = 4;

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

        self.is_valid_name_part(parts[0]) && self.is_valid_number_part(parts[1])
    }

    fn is_valid_name_part(&self, name_part: &str) -> bool {
        !name_part.is_empty() && name_part.chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn is_valid_number_part(&self, number_part: &str) -> bool {
        number_part.len() >= MIN_NUMBER_LENGTH && number_part.chars().all(|c| c.is_ascii_digit())
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
            Ok(decoded) => self.matches_expected_format(&decoded),
            Err(_) => false,
        }
    }

    fn matches_expected_format(&self, decoded: &str) -> bool {
        let expected_combinations = self.get_expected_combinations();
        let decoded_lower = decoded.to_lowercase();

        expected_combinations
            .iter()
            .any(|expected| self.is_matching_combination(expected, decoded, &decoded_lower))
    }

    fn is_matching_combination(&self, expected: &str, decoded: &str, decoded_lower: &str) -> bool {
        decoded_lower == expected.to_lowercase() || decoded == expected
    }

    fn get_expected_combinations(&self) -> Vec<String> {
        let battlenet_lower = self.battlenet_id.to_lowercase();
        let configs = self.get_wa_config_strings();

        configs
            .into_iter()
            .flat_map(|config| {
                vec![
                    format!("{battlenet_lower}{config}"),
                    format!("{}{config}", self.battlenet_id),
                ]
            })
            .collect()
    }

    fn get_wa_config_strings(&self) -> Vec<&'static str> {
        vec![WA4_CONFIG, WA5_CONFIG]
    }

    fn get_addon_config(&self) -> Result<String, String> {
        match self.version.as_str() {
            "retail" | "classic" => Ok(DEFAULT_CONFIG.to_string()),
            _ => Err("Invalid version".to_string()),
        }
    }

    fn decode_import_string(&self, encoded_string: &str) -> Result<String, String> {
        let cleaned = self.clean_base64_string(encoded_string);

        general_purpose::STANDARD
            .decode(&cleaned)
            .map_err(|e| format!("Base64 decode error: {e}"))
            .and_then(|bytes| {
                String::from_utf8(bytes).map_err(|e| format!("UTF-8 decode error: {e}"))
            })
    }

    fn clean_base64_string(&self, input: &str) -> String {
        input
            .chars()
            .filter(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '='))
            .collect()
    }

    pub fn get_wa_configs(&self) -> (String, String) {
        let wa4 = self.build_wa_config(
            &[99, 116, 100, 118, 101, 105],
            &[114, 118, 114, 116, 105, 100, 99, 101],
        );
        let wa5 = self.build_wa_config(
            &[118, 114, 105, 100, 116, 99],
            &[101, 116, 118, 114, 100, 105, 99, 101],
        );
        (wa4, wa5)
    }

    fn build_wa_config(&self, part1: &[u8], part2: &[u8]) -> String {
        let part1_str: String = part1.iter().map(|&c| c as char).collect();
        let part2_str: String = part2.iter().map(|&c| c as char).collect();
        format!("{part1_str}{part2_str}")
    }
}
