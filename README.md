# FHIR Resources Rust Library

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/Status-Personal%20Project-red.svg)]()

> **IMPORTANT DISCLAIMER**  
> This is a **personal project** and is **NOT ready for production use**. This library is being developed for educational purposes and personal healthcare applications. Do not use this in production healthcare systems without thorough testing and validation.

## Overview

A Rust library for working with FHIR (Fast Healthcare Interoperability Resources) data structures. This library provides type-safe implementations of common FHIR resources and data types used in healthcare applications.

## Project Status

- **Status**: Personal Development Project
- **Production Ready**:  No - Not suitable for production use
- **Testing**: Comprehensive test suite included
- **Documentation**: Basic documentation available
- **FHIR Compliance**: Partial - Not fully FHIR compliant

## Features

### Supported FHIR Versions

- **R4**: FHIR version 4.0.1 (full support)
- **R4B**: FHIR version 4.3.0 (full support)
- **R5**: FHIR version 5.0.0 (full support)

### Implemented Resources

#### Common Resources (R4, R4B, R5)
- **Patient**: Complete patient demographics and identifiers
- **Practitioner**: Healthcare provider information
- **Organization**: Healthcare organizations
- **Bundle**: Resource bundles for data exchange
- **Observation**: Clinical observations and measurements
- **Condition**: Health conditions and diagnoses
- **Procedure**: Medical procedures performed
- **MedicationRequest**: Medication orders and prescriptions
- **Encounter**: Healthcare encounters and visits
- **DiagnosticReport**: Diagnostic test results

#### R5-Specific Resources
- **Device**: Medical devices and equipment
- **Specimen**: Biological specimens
- **ImagingStudy**: Medical imaging studies
- **InventoryItem**: Healthcare inventory management
- **Transport**: Resource transport logistics
- **GenomicStudy**: Genomic research and analysis

### Data Types

- **HumanName**: Structured human names with international support
- **Identifier**: Healthcare identifiers (must be boxed in vectors)
- **Period**: Time periods for healthcare events
- **ContactPoint**: Contact information (phone, email, etc.)
- **Address**: Complete address structure with international support
- **CodeableConcept**: Coded concepts with coding arrays
- **Coding**: Individual coding entries
- **Reference**: Resource references between FHIR resources

### Technical Features

- **Type Safety**: Strongly typed Rust implementations with proper memory management
- **Serialization**: JSON serialization/deserialization support with FHIR camelCase
- **Multi-Version**: Support for multiple FHIR versions in a single library
- **Testing**: Comprehensive test suite covering all versions
- **Documentation**: Detailed usage examples and architecture documentation
- **Memory Optimization**: Box wrapping for large optional collections
- **FHIR Compliance**: Proper field naming and structure adherence

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fhir-resources-rs = "0.1.0"
```

## Quick Start

### Creating a FHIR R4 Patient

```rust
use fhir_resources::r4::patient::Patient;
use fhir_resources::r4::types::{HumanName, Identifier, ContactPoint, Address};

// Create a patient with identifiers
let patient = Patient {
    resource_type: "Patient".to_string(),
    id: Some("patient-123".to_string()),
    meta: None,
    implicit_rules: None,
    language: Some("en-US".to_string()),
    text: None,
    contained: None,
    extension: None,
    modifier_extension: None,
    // Important: Identifiers must be Box-wrapped in vectors
    identifier: Some(vec![Box::new(Identifier {
        id: None,
        extension: None,
        r#use: Some("official".to_string()),
        r#type: None,
        system: Some("http://example.org/mrn".to_string()),
        value: Some("MRN-12345".to_string()),
        period: None,
        assigner: None,
    })]),
    active: Some(true),
    name: Some(vec![HumanName {
        id: None,
        extension: None,
        r#use: Some("official".to_string()),
        text: Some("John Doe".to_string()),
        family: Some("Doe".to_string()),
        given: Some(vec!["John".to_string()]),
        prefix: None,
        suffix: None,
        period: None,
    }]),
    telecom: Some(vec![ContactPoint {
        id: None,
        extension: None,
        system: Some("phone".to_string()),
        value: Some("+1-555-555-5555".to_string()),
        r#use: Some("home".to_string()),
        rank: Some(1),
        period: None,
    }]),
    gender: Some("male".to_string()),
    birth_date: Some("1980-01-01".to_string()),
    deceased: None,
    address: Some(vec![Address {
        id: None,
        extension: None,
        r#use: Some("home".to_string()),
        r#type: Some("physical".to_string()),
        text: Some("123 Main St, Anytown, USA".to_string()),
        line: Some(vec!["123 Main St".to_string()]),
        city: Some("Anytown".to_string()),
        district: None,
        state: Some("CA".to_string()),
        postal_code: Some("12345".to_string()),
        country: Some("USA".to_string()),
        period: None,
    }]),
    marital_status: None,
    multiple_birth: None,
    photo: None,
    contact: None,
    communication: None,
    general_practitioner: None,
    managing_organization: None,
    link: None,
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&patient).unwrap();
println!("{}", json);

// Deserialize from JSON
let patient_from_json: Patient = serde_json::from_str(&json).unwrap();
```

### Using FHIR R5

```rust
use fhir_resources::r5::patient::Patient;

// The structure is similar across versions
let r5_patient_json = r#"{
    "resourceType": "Patient",
    "id": "r5-patient",
    "active": true,
    "gender": "female"
}"#;

let patient: Patient = serde_json::from_str(r5_patient_json).unwrap();
assert_eq!(patient.resource_type, "Patient");
```

### Important: Identifier Field Boxing

**Note**: The `identifier` field uses `Option<Vec<Box<Identifier>>>` for memory optimization. When creating identifiers, each `Identifier` must be wrapped in `Box::new()`:

```rust
// Correct ✅
identifier: Some(vec![Box::new(Identifier { /* ... */ })])

// Incorrect ❌
identifier: Some(vec![Identifier { /* ... */ }])
```

## Running Tests

```bash
# Run all tests in the workspace
cargo test

# Run tests for specific version
cargo test --test r4_resources
cargo test --test r4b_resources
cargo test --test r5_resources

# Run cross-version integration tests
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture

# Run a specific test
cargo test test_r4_patient_creation -- --nocapture
```

## Project Structure

```
fhir-resources-rs/
├── crates/
│   ├── codegen/          # Code generation from FHIR schemas
│   │   └── src/
│   │       ├── r4_generator/
│   │       ├── r4b_generator/
│   │       └── r5_generator/
│   ├── resources/        # Generated FHIR resource types
│   │   ├── src/
│   │   │   ├── r4/      # FHIR R4 resources
│   │   │   ├── r4b/     # FHIR R4B resources
│   │   │   ├── r5/      # FHIR R5 resources
│   │   │   └── r6/      # FHIR R6 (upcoming)
│   │   └── tests/       # Comprehensive test suite
│   └── types/           # Common types and utilities
├── docs/                # Documentation
│   └── implementation/  # Version-specific docs
├── examples/            # Usage examples
└── schemas/            # FHIR JSON schemas
    ├── r4/
    ├── r4b/
    └── r5/
```


## Contributing

This is a personal project, but suggestions and feedback are welcome. Please note that this is not intended for production use.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Disclaimer

This library is provided "as is" without warranty of any kind. It is not intended for use in production healthcare systems. Always validate healthcare data according to your organization's requirements and applicable regulations.

---

**⚠️ Remember**: This is a personal development project and should not be used in production healthcare systems without proper validation and testing.
