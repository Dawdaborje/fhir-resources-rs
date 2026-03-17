//! FHIR JSON Schema Parser
//!
//! Parses FHIR Bundle JSON schemas into intermediate representation.

use crate::common::codegen::StructureDefinition;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// FHIR Bundle container
#[derive(Debug, Deserialize)]
pub struct FhirBundle {
    #[allow(dead_code)]
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    #[allow(dead_code)]
    #[serde(rename = "type")]
    pub bundle_type: String,

    pub entry: Vec<BundleEntry>,
}

/// Bundle entry containing a resource
#[derive(Debug, Deserialize)]
pub struct BundleEntry {
    #[serde(rename = "fullUrl")]
    pub full_url: String,

    pub resource: serde_json::Value, // Use Value to handle different resource types
}

/// Parse a FHIR Bundle from a JSON file
pub fn parse_bundle(path: &Path) -> Result<Vec<StructureDefinition>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let bundle: FhirBundle = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))?;

    // Extract StructureDefinitions from bundle
    let mut definitions = Vec::new();

    for entry in bundle.entry {
        // Check if this is a StructureDefinition
        if let Some(resource_type) = entry.resource.get("resourceType").and_then(|v| v.as_str()) {
            if resource_type == "StructureDefinition" {
                // Try to deserialize as StructureDefinition
                match serde_json::from_value::<StructureDefinition>(entry.resource) {
                    Ok(def) => definitions.push(def),
                    Err(e) => {
                        // Log and skip entries that can't be parsed
                        eprintln!("Warning: Failed to parse StructureDefinition: {}", e);
                    }
                }
            }
        }
    }

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use crate::common::codegen::ElementDefinition;

    use super::*;

    #[test]
    fn test_parse_element_definition() {
        let json = r#"{
            "id": "Patient.identifier",
            "path": "Patient.identifier",
            "min": 0,
            "max": "*",
            "type": [{"code": "Identifier"}],
            "short": "An identifier for this patient"
        }"#;

        let element: ElementDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(element.id, "Patient.identifier");
        assert_eq!(element.min, 0);
        assert_eq!(element.max, "*");
        assert_eq!(element.element_type.len(), 1);
        assert_eq!(element.element_type[0].code, "Identifier");
    }
}
