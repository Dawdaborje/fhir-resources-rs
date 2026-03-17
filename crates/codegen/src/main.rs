//! FHIR Code Generator
//!
//! Generates Rust types from FHIR schema definitions.
//!
//! Usage: cargo run --bin codegen -- <version>
//!   where <version> is one of: r4, r4b, r5, r6

use anyhow::{bail, Result};
use std::env;

mod common;
mod r4_generator;
mod r4b_generator;
mod r5_generator;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        bail!(
            "Usage: {} <version>\n  Supported versions: r4, r4b, r5, r6",
            args[0]
        );
    }

    let version = &args[1];

    println!("Generating FHIR {} types...", version);

    match version.as_str() {
        "r4" => r4_generator::generate()?,
        "r4b" => r4b_generator::generate()?,
        "r5" => r5_generator::generate()?,
        "r6" => bail!("FHIR R6 is not yet supported"),
        _ => bail!(
            "Unknown FHIR version: {}. Supported versions: r4, r4b, r5, r6",
            version
        ),
    }

    println!("✓ Code generation complete!");

    Ok(())
}
