use crate::domain_utils;
use crate::fetch_gempage;

use std::{fs, io::Write};

use colored::*;

pub async fn depth_based(gem_body: String, current_path: String, url: String) -> anyhow::Result<()> {
    let mut anchor_links: Vec<String> = Vec::new();
    anchor_links = domain_utils::extract_links(anchor_links, gem_body.clone(), url.clone()).await?;

    for sublink in anchor_links {
        let sublink_gem_body = fetch_gempage::mk_req(sublink.clone()).await?;

        let sublink_domain = domain_utils::get_proper_domain(url.clone()).await?;
        let sublink_path = domain_utils::get_path(sublink.clone()).await?;

        let sublink_sanitized_domain = sublink_domain
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric() || c == '-' || c == '.')
            .collect::<String>();

        let sublink_path_with_dashes = sublink_path.replace("/", "-");

        let sublink_gem_path = format!(
            "{}/{}-{}.gmi",
            current_path, sublink_sanitized_domain, sublink_path_with_dashes

        );

        let sublink_gem = fs::File::create(sublink_gem_path.clone());

        println!("{}: Writing: {} to {}", "LOG".yellow().bold(), sublink.clone(), sublink_gem_path.clone());

        sublink_gem
            .expect("Cannot write to this file")
            .write_all(sublink_gem_body.as_bytes())?;
    }
    
    Ok(())
}
