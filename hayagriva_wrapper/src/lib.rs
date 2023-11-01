use wasm_bindgen::prelude::*;
use hayagriva::io::from_biblatex_str;
use hayagriva::io::to_yaml_str;

#[wasm_bindgen]
pub fn convert_biblatex_to_hayagriva(bib_str: &str) -> String {
    let result = from_biblatex_str(bib_str);
    match result {
        Ok(library) => {
            if library.is_empty() {
                return "Error parsing Bibtex".to_string();
            } else {
                return to_yaml_str(&library).unwrap_or("Error converting to YAML".to_string());
            }
        }
        Err(errors) => {
            let mut error_str = String::new();
            error_str.push_str("Error parsing Bibtex: \n");
            for error in errors {
                error_str.push_str("* ");
                error_str.push_str(&error.to_string());
                error_str.push_str("\n");
            }
            return error_str;
        }
    }
}
