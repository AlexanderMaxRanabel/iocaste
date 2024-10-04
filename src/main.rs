mod archive_gempage;
mod depth_gempage;
mod domain_utils;
mod fetch_gempage;
mod gemurl_parse;

use colored::*;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let url = args.get(2).cloned().unwrap_or_else(|| {
            println!("{}: No url has been provided", "Error".red());
            std::process::exit(1);
        });

        let depth_mode = args.get(4).cloned().unwrap_or_else(|| {
            println!("{}: No depth mode has been provided", "Error".red());
            std::process::exit(1);
        });

        let gem_body = fetch_gempage::mk_req(url.clone()).await?;
        let current_path = gemurl_parse::parse_and_create_dir(url.clone()).await?;

        match depth_mode.as_str() {
            "False" => {
                archive_gempage::archive_page(current_path.clone(), gem_body.clone(), "main_output.gmi".to_string()).await?;
            }

            "True" => {
                archive_gempage::archive_page(current_path.clone(), gem_body.clone(), "main_output.gmi".to_string()).await?;
                depth_gempage::depth_based(gem_body.clone(), current_path.clone(), url.clone()).await?;
            }

            _ => {
                println!("{}: Unknown depth mode", "Error".red());
            }
        }
    } else {
        println!("{}: Gemini archiver", "Iocaste".magenta());
        std::process::exit(1);
    }

    Ok(())
}
