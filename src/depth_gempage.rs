use crate::domain_utils;
use crate::fetch_gempage;
use crate::archive_gempage;

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
            "{}-{}.gmi",
            sublink_sanitized_domain, sublink_path_with_dashes

        );

        let _ = archive_gempage::archive_page(current_path.clone(), sublink_gem_body, sublink_gem_path).await?;
    }
    
    Ok(())
}
