use colored::*;

pub async fn create_link(domain: String, sublink: String) -> anyhow::Result<String> {
    let url: String;
    if sublink.starts_with("/") {
        url = format!("{}{}{}", "gemini://", domain, sublink);
    } else {
        url = format!("{}{}/{}", "gemini://", domain, sublink);
    }
    Ok(url)
}

pub async fn get_proper_domain(url: String) -> anyhow::Result<String> {
    let mut domain = String::new();

    if url.starts_with("gemini://") {
        let parts: Vec<&str> = url.split("gemini://").collect();
        domain = if parts.len() > 1 {
            let second_part = parts[1];
            (second_part.to_string().split('/').next().unwrap_or("")).to_string()
        } else {
            println!("{}: Failed to parse", "Error".red());
            std::process::exit(1);
        }
    }

    Ok(domain)
}

pub async fn get_path(sublink: String) -> anyhow::Result<String> {
    let start_index = match sublink.find("gemini://") {
        Some(index) => index + "gemini://".len(),
        None => 0,
    };
    let end_index = match sublink.find(".gmi") {
        Some(index) => index,
        None => 0,
    };
    let result = &sublink[start_index..end_index];

    Ok(result.to_string())
}

pub async fn extract_links(
    mut anchor_links: Vec<String>,
    gem_body: String,
    url: String,
) -> anyhow::Result<Vec<String>> {
    let vectorized_content: Vec<&str> = gem_body.lines().collect();

    for line in vectorized_content {
        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        if let Some(token_start) = tokens.get(0) {
            if token_start == "=>" && tokens.len() > 1 {
                let mut link = tokens.get(1).cloned().unwrap_or_else(|| {
                    println!(
                        "{}: No link has been found in tokens: {:?}",
                        colored::Colorize::red("Error"),
                        tokens
                    );
                    std::process::exit(1);
                });

                if !link.starts_with("gemini://") {
                    let domain = get_proper_domain(url.clone()).await?;
                    link = create_link(domain, url.clone()).await?;
                }

                anchor_links.push(link);
            }
        }
    }
    Ok(anchor_links)
}
