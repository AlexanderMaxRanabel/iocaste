use std::{fs, io::prelude::*};

use colored::*;

pub async fn archive_page(current_path: String, html_body: String) -> anyhow::Result<()> {
    let html_path = format!("{}/{}", current_path, "main_output.gmi");
    let html = fs::File::create(html_path.clone());

    println!("{}: {}", "LOG".yellow().bold(), html_path.clone());

    html.expect("Cannot write to this file")
        .write_all(html_body.as_bytes())?;
    Ok(())
}
