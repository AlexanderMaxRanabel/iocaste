use std::{fs, path::Path};

use chrono::prelude::*;
use colored::*;

pub async fn parse_and_create_dir(url: String) -> anyhow::Result<String> {
    let mut parsed_url = String::new();

    if url.starts_with("gemini://") {
        let parts: Vec<&str> = url.split("gemini://").collect();
        parsed_url = if parts.len() > 1 {
            let second_part = parts[1];
            (second_part.to_string().split('/').next().unwrap_or("")).to_string()
        } else {
            println!("{}: Failed to parse", "Error".red());
            std::process::exit(1);
        }
    } else {
        if let Some(index) = url.find('/') {
            parsed_url = (&url[..index]).to_string();
        }
    }

    let sanitized_domain: String = parsed_url
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
        .collect::<String>();

    let pathified_domain = Path::new(&sanitized_domain);
    if pathified_domain.exists() {
        println!(
            "{}: Path {} was previously recorded, Recording into previously generated archive",
            "LOG".yellow().bold(),
            parsed_url
        );
    } else {
        fs::create_dir_all(sanitized_domain.clone())?;
    }

    let utc: DateTime<Utc> = Utc::now();
    let formatted_utc = utc.format("%Y-%m-%dT%H%M%S%.f").to_string();
    let current_path: String = format!("{}/{}", sanitized_domain, formatted_utc);

    println!(
        "{}: To be written on: {}",
        "LOG".yellow().bold(),
        current_path.clone()
    );

    fs::create_dir(current_path.clone())?;
    Ok(current_path.clone())
}
