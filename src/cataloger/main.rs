// This is free and unencumbered software released into the public domain.

use asimov_module::SysexitsError::{self, *};
use asimov_x_module::api::x::XClient;
use asimov_x_module::providers::x::extract_list_id_from_url;
use clap::{Parser, Subcommand};
use clientele::StandardOptions;
use std::error::Error;

#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Catalog X list members from a list URL.")]
    GetListMembers {
        url: String,

        #[clap(short, long)]
        limit: Option<usize>,

        #[clap(short, long, default_value = "jsonl")]
        output: String,
    },
}

pub fn main() -> Result<SysexitsError, Box<dyn Error>> {
    asimov_module::dotenv().ok();
    let args = asimov_module::args_os()?;
    let options = Options::parse_from(args);

    if options.flags.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        return Ok(EX_OK);
    }

    if options.flags.license {
        print!("{}", include_str!("../../UNLICENSE"));
        return Ok(EX_OK);
    }

    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    match options.command {
        Some(Commands::GetListMembers { url, limit, output }) => {
            let output_format = if output == "json" || output == "jsonl" {
                output
            } else {
                eprintln!(
                    "Warning: Invalid output format '{}'. Using 'jsonl' instead.",
                    output
                );
                "jsonl".to_string()
            };

            let list_id = extract_list_id_from_url(&url)
                .ok_or_else(|| anyhow::anyhow!("Invalid X list URL: {}", url))?;

            let client = XClient::new()?;
            let api_response = client.fetch_list_members(&list_id, limit)?;

            let filter = asimov_x_module::jq::x_list();
            let transformed = filter
                .filter_json(serde_json::to_value(api_response)?)
                .map_err(|e| anyhow::anyhow!("JQ filter error: {}", e))?;

            match output_format.as_str() {
                "jsonl" => {
                    if let Some(members) = transformed["members"]["items"].as_array() {
                        for member in members {
                            println!("{}", serde_json::to_string(member)?);
                        }
                    }
                },
                "json" => {
                    if cfg!(feature = "pretty") {
                        colored_json::write_colored_json(&transformed, &mut std::io::stdout())?;
                        println!();
                    } else {
                        println!("{}", serde_json::to_string(&transformed)?);
                    }
                },
                _ => {
                    if let Some(members) = transformed["members"]["items"].as_array() {
                        for member in members {
                            println!("{}", serde_json::to_string(member)?);
                        }
                    }
                },
            };
        },
        None => {
            eprintln!("No command specified. Use --help for usage information.");
            return Ok(EX_USAGE);
        },
    }

    Ok(EX_OK)
}
