use anyhow::Result;
use clap::Parser;
use log::info;
use std::io::{self, BufRead, BufReader};

use whippyunits_lsp_proxy::DisplayConfig;
use whippyunits_pretty::rustc_pretty::RustcPrettyPrinter;

/// Pretty-print rustc output with whippyunits type formatting
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose output mode
    #[arg(short, long)]
    verbose: bool,

    /// Disable Unicode symbols in output
    #[arg(short, long)]
    no_unicode: bool,

    /// Include raw type information
    #[arg(short = 'r', long)]
    include_raw: bool,

    /// Enable debug logging
    #[arg(short, long)]
    debug: bool,

    /// Input file (if not provided, reads from stdin)
    #[arg(short = 'f', long)]
    input: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    if args.debug {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    info!("🚀 WHIPPYUNITS PRETTY PRINTER STARTING");

    // Create display configuration
    let display_config = DisplayConfig {
        verbose: args.verbose,
        unicode: !args.no_unicode,
        include_raw: args.include_raw,
    };

    // Create the pretty printer
    let mut printer = RustcPrettyPrinter::with_config(display_config);

    // Process input
    if let Some(input_file) = args.input {
        // Read from file
        let content = std::fs::read_to_string(&input_file)?;
        let processed = printer.process_rustc_output(&content)?;
        print!("{}", processed);
    } else {
        // Read from stdin
        let stdin = io::stdin();
        let reader = BufReader::new(stdin.lock());

        for line in reader.lines() {
            let line = line?;
            let processed = printer.process_line(&line)?;
            println!("{}", processed);
        }
    }

    Ok(())
}
