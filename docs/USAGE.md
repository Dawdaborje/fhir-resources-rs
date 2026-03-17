# FHIR Code Generator - Usage Guide

## Overview

This tool generates type-safe Rust structs from FHIR (Fast Healthcare Interoperability Resources) JSON schema definitions. It currently supports FHIR R4 with planned support for R4B, R5, and R6.

## Prerequisites

- Rust 1.70 or later
- `rustfmt` (for code formatting)
- FHIR schema files in `schemas/<version>/` directory

## Quick Start

### Running the Generator

From the workspace root directory:

```bash
cargo run --bin codegen -- r4
```

This will:
1. Parse FHIR R4 schemas from `schemas/r4/`
2. Generate Rust types in `crates/resources/src/r4/`
3. Format the generated code with rustfmt

### Expected Output

```
Generating FHIR r4 types...
  Parsing FHIR R4 schemas...
    Parsed 149 resources and 63 types
  Generating Rust code...
    Generated 148 resource files
  Writing files...
  Formatting code with rustfmt...
    Code formatted successfully
Code generation complete!
```

## Command-Line Interface

### Syntax

```bash
cargo run --bin codegen -- <version>
```

### Arguments

- `<version>` - FHIR version to generate (required)
  - `r4` - FHIR R4 (currently supported)
  - `r4b` - FHIR R4B (not yet implemented)
  - `r5` - FHIR R5 (not yet implemented)
  - `r6` - FHIR R6 (not yet implemented)

### Examples

Generate FHIR R4 types:
```bash
cargo run --bin codegen -- r4
```

Run from a different directory:
```bash
cargo run --manifest-path /path/to/fhir-resources-rs/Cargo.toml --bin codegen -- r4
```

## Generated Code Structure

The generator creates the following file structure:

```
crates/resources/src/r4/
├── mod.rs                          # Module declarations
├── types.rs                        # Complex types (HumanName, Address, etc.)
├── account.rs                      # Account resource
├── patient.rs                      # Patient resource
├── observation.rs                  # Observation resource
└── ...                             # All other FHIR resources
```

### Module Organization

- **mod.rs** - Contains `pub mod` declarations for all generated modules
- **types.rs** - FHIR data types (HumanName, Address, Period, Identifier, etc.)
- **resource files** - One file per FHIR resource type (Patient, Observation, etc.)

## Generated Code Features

### Type Safety

All generated structs include:
- Proper Rust naming conventions (snake_case fields, PascalCase types)
- Serde serialization/deserialization annotations
- Comprehensive documentation from FHIR schemas

### Cardinality Mapping

FHIR cardinality is mapped to Rust types:

| FHIR (min, max) | Rust Type | Description |
|-----------------|-----------|-------------|
| 0, 1 | `Option<T>` | Optional single value |
| 0, * | `Option<Vec<T>>` | Optional array |
| 1, 1 | `T` | Required single value |
| 1, * | `Vec<T>` | Required array |

### Example Generated Code

Patient resource:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// An identifier for this patient
    pub identifier: Option<Vec<Box<Identifier>>>,
    
    /// Whether this patient's record is in active use
    pub active: Option<bool>,
    
    /// A name associated with the patient
    pub name: Option<Vec<HumanName>>,
}
```

## Using Generated Types

### Add Dependency

In your `Cargo.toml`:

```toml
[dependencies]
fhir-resources = { path = "crates/resources" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Import Types

```rust
use fhir_resources::r4::{Patient, HumanName};
use serde_json;
```

### Deserialize FHIR JSON

```rust
let json_data = r#"{
    "resourceType": "Patient",
    "id": "example",
    "active": true,
    "name": [{
        "family": "Doe",
        "given": ["John"]
    }]
}"#;

let patient: Patient = serde_json::from_str(json_data)?;
println!("Patient name: {:?}", patient.name);
```

### Serialize to FHIR JSON

```rust
let patient = Patient {
    id: Some("example".to_string()),
    active: Some(true),
    name: Some(vec![HumanName {
        family: Some("Doe".to_string()),
        given: Some(vec!["John".to_string()]),
        ..Default::default()
    }]),
    ..Default::default()
};

let json = serde_json::to_string_pretty(&patient)?;
println!("{}", json);
```

## Troubleshooting

### Error: No such file or directory (schemas/r4/profiles-resources.json)

**Cause**: Running the generator from the wrong directory.

**Solution**: Always run from the workspace root:
```bash
cd /path/to/fhir-resources-rs
cargo run --bin codegen -- r4
```

### Warning: rustfmt failed

**Cause**: rustfmt is not installed or not in PATH.

**Solution**: Install rustfmt:
```bash
rustup component add rustfmt
```

Note: The generator will continue even if rustfmt fails, but code may be unformatted.

### Compilation Warnings: type should have upper camel case name

**Cause**: Some FHIR primitive types have lowercase names (e.g., `date`, `uuid`).

**Solution**: These are warnings only and don't affect functionality. They match FHIR naming conventions.

## Development Workflow

### Typical Usage Pattern

1. **Generate types** after schema updates:
   ```bash
   cargo run --bin codegen -- r4
   ```

2. **Build resources crate** to verify generated code:
   ```bash
   cargo build -p fhir-resources
   ```

3. **Use in your application**:
   ```rust
   use fhir_resources::r4::Patient;
   ```

### Regenerating Code

The generator overwrites existing files in the output directory. To regenerate:

```bash
# Clean old generated files (optional)
rm -rf crates/resources/src/r4/*.rs

# Regenerate
cargo run --bin codegen -- r4
```

## Schema Files

### Required Files

For R4 generation, these files must exist in `schemas/r4/`:
- `profiles-resources.json` - Resource definitions (Patient, Observation, etc.)
- `profiles-types.json` - Complex type definitions (HumanName, Address, etc.)

### Schema Sources

FHIR schemas can be downloaded from:
- FHIR R4: https://www.hl7.org/fhir/R4/
- Official downloads: https://www.hl7.org/fhir/downloads.html

## Advanced Usage

### Custom Output Directory

Modify `crates/codegen/src/r4_generator/mod.rs`:

```rust
let output_dir = Path::new("your/custom/path");
```

### Adding Support for Other Versions

To add R4B support:

1. Create `crates/codegen/src/r4b_generator/` module
2. Copy and adapt code from `r4_generator/`
3. Update `main.rs` dispatcher:
   ```rust
   match version.as_str() {
       "r4" => r4_generator::generate()?,
       "r4b" => r4b_generator::generate()?,
       // ...
   }
   ```

### Customizing Type Mappings

Edit `crates/codegen/src/r4_generator/type_mapper.rs`:

```rust
pub fn map_primitive_type(fhir_type: &str) -> String {
    match fhir_type {
        "date" => "chrono::NaiveDate".to_string(),  // Custom mapping
        // ...
    }
}
```

## Known Limitations

1. **Choice types** - Currently mapped to `serde_json::Value`, not type-safe enums
2. **ValueSet enums** - Coded values use `String` instead of generated enums
3. **BackboneElements** - Nested types aren't separated into submodules yet
4. **Resource polymorphism** - `Resource` type uses `serde_json::Value` placeholder

These limitations will be addressed in future releases.

## Performance

Typical generation times on modern hardware:
- Parsing: < 1 second
- Code generation: < 1 second
- rustfmt: 2-3 seconds
- Total: ~5 seconds

Generated code size:
- ~150 files
- ~15,000 lines of Rust code
- ~1 MB total

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review implementation documentation in `docs/implementation/r4/`
3. Examine generated code for specific resource types

## Version Compatibility

| FHIR Version | Status | Schema Date |
|--------------|--------|-------------|
| R4 (4.0.1) | Supported | 2019-10-30 |
| R4B | Planned | TBD |
| R5 | Planned | TBD |
| R6 | Planned | TBD |
