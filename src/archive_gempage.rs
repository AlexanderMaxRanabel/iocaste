use std::{fs, io::prelude::*};

use colored::*;

pub async fn archive_page(current_path: String, gem_body: String, output_name: String) -> anyhow::Result<()> {
    let gemtext_path = format!("{}/{}", current_path, output_name);
    let gemtext = fs::File::create(gemtext_path.clone());

    println!("{}: {}", "LOG".yellow().bold(), gemtext_path.clone());

    gemtext
        .expect("Cannot write to this file")
        .write_all(gem_body.as_bytes())?;
    Ok(())
}
