//! Example: Creating and serializing a FHIR R5 Patient resource

use fhir_resources::r5::patient::Patient;
use fhir_resources::r5::types::{Address, ContactPoint, HumanName};
use serde_json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple R5 Patient
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("example-r5".to_string()),
        meta: None,
        implicit_rules: None,
        language: Some("en-US".to_string()),
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            text: Some("Peter James Chalmers".to_string()),
            family: Some("Chalmers".to_string()),
            given: Some(vec!["Peter".to_string(), "James".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some("phone".to_string()),
            value: Some("+1-555-123-4567".to_string()),
            r#use: Some("home".to_string()),
            rank: Some(1),
            period: None,
        }]),
        gender: Some("male".to_string()),
        birth_date: Some("1974-12-25".to_string()),
        deceased: None,
        address: Some(vec![Address {
            id: None,
            extension: None,
            r#use: Some("home".to_string()),
            r#type: Some("physical".to_string()),
            text: Some("534 Erewhon St, Pleasantville, Vic, 3999".to_string()),
            line: Some(vec!["534 Erewhon St".to_string()]),
            city: Some("Pleasantville".to_string()),
            district: None,
            state: Some("Vic".to_string()),
            postal_code: Some("3999".to_string()),
            country: Some("Australia".to_string()),
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
    let json = serde_json::to_string_pretty(&patient)?;

    println!("FHIR R5 Patient Resource:");
    println!("{}", json);

    // Deserialize back
    let _deserialized: Patient = serde_json::from_str(&json)?;

    println!("\n✓ Successfully created, serialized, and deserialized FHIR R5 Patient!");

    Ok(())
}
