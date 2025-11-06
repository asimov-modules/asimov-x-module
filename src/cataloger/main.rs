// This is free and unencumbered software released into the public domain.

use asimov_module::SysexitsError::{self, *};
use asimov_x_module::api::x::XClient;
use asimov_x_module::providers::x::extract_list_id_from_url;
use clap::Parser;
use clientele::StandardOptions;
use std::error::Error;

#[derive(Debug, Parser)]
struct Options {
    #[clap(flatten)]
    flags: StandardOptions,

    /// X list URL to catalog
    url: Option<String>,

    /// Limit the number of members to fetch
    #[clap(short, long)]
    limit: Option<usize>,

    /// Output format (json or jsonl)
    #[clap(short, long, default_value = "jsonl")]
    output: String,
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

    // Check if URL is provided
    let url = options.url.ok_or_else(|| {
        eprintln!("Error: URL is required. Use --help for usage information.");
        EX_USAGE
    })?;

    #[cfg(feature = "tracing")]
    asimov_module::init_tracing_subscriber(&options.flags).expect("failed to initialize logging");

    let output_format = if options.output == "json" || options.output == "jsonl" {
        options.output
    } else {
        eprintln!(
            "Warning: Invalid output format '{}'. Using 'jsonl' instead.",
            options.output
        );
        "jsonl".to_string()
    };

    let list_id = extract_list_id_from_url(&url)
        .ok_or_else(|| anyhow::anyhow!("Invalid X list URL: {}", url))?;

    let client = XClient::new()?;
    let api_response = client.fetch_list_members(&list_id, options.limit)?;

    let filter = asimov_x_module::jq::x_list();
    let transformed = filter
        .filter_json(serde_json::to_value(api_response)?)
        .map_err(|e| anyhow::anyhow!("JQ filter error: {}", e))?;

    match output_format.as_str() {
        "jsonl" => {
            if let Some(members) = transformed["members"]["items"].as_array() {
                for member in members {
                    println!("{}", member);
                }
            }
        },
        "json" => {
            println!("{}", transformed);
        },
        _ => {
            if let Some(members) = transformed["members"]["items"].as_array() {
                for member in members {
                    println!("{}", member);
                }
            }
        },
    };

    Ok(EX_OK)
}
