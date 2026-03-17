//! Code Generation
//!
//! Generates Rust source code from FHIR structure definitions.
//! This module is version-agnostic and can be used for any FHIR version.

use super::type_mapper::*;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

/// Element definition (shared structure across FHIR versions)
#[derive(Debug, Deserialize, Clone)]
pub struct ElementDefinition {
    pub id: String,
    pub path: String,

    #[serde(default)]
    pub min: u32,

    #[serde(default = "default_max")]
    pub max: String,

    #[serde(rename = "type", default)]
    pub element_type: Vec<TypeRef>,

    #[serde(default)]
    pub short: Option<String>,

    #[serde(default)]
    pub definition: Option<String>,

    #[serde(default)]
    pub binding: Option<Binding>,

    #[serde(rename = "contentReference", default)]
    pub content_reference: Option<String>,
}

fn default_max() -> String {
    "1".to_string()
}

/// Type reference
#[derive(Debug, Deserialize, Clone)]
pub struct TypeRef {
    pub code: String,

    #[serde(rename = "targetProfile", default)]
    pub target_profile: Vec<String>,
}

/// ValueSet binding
#[derive(Debug, Deserialize, Clone)]
pub struct Binding {
    pub strength: String,

    #[serde(rename = "valueSet", default)]
    pub value_set: Option<String>,
}

/// Snapshot of element definitions
#[derive(Debug, Deserialize, Clone)]
pub struct Snapshot {
    pub element: Vec<ElementDefinition>,
}

/// FHIR StructureDefinition
#[derive(Debug, Deserialize, Clone)]
pub struct StructureDefinition {
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    pub id: String,
    pub name: String,

    #[serde(default)]
    pub kind: Option<String>,

    #[serde(rename = "type")]
    pub structure_type: Option<String>,

    pub snapshot: Option<Snapshot>,

    #[serde(default)]
    pub description: Option<String>,
}

/// Sanitize a string for use in doc comments
fn sanitize_doc_comment(text: &str) -> String {
    // Replace newlines and multiple spaces with single space
    let cleaned = text
        .lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join(" ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // Truncate if too long (keep reasonable line length)
    if cleaned.len() > 200 {
        format!("{}...", &cleaned[..197])
    } else {
        cleaned
    }
}

/// Generate types.rs content from type definitions
pub fn generate_types(definitions: &[StructureDefinition], version: &str) -> Result<String> {
    let mut output = String::new();

    // File header
    output.push_str(&format!(
        "//! FHIR {} Complex Types\n",
        version.to_uppercase()
    ));
    output.push_str("//!\n");
    output.push_str("//! Auto-generated from FHIR schema definitions.\n");
    output.push_str("//! Do not modify directly.\n\n");
    output.push_str("#![allow(non_camel_case_types)]\n\n");
    output.push_str("use serde::{Deserialize, Serialize};\n\n");

    // Generate each type
    for def in definitions {
        if let Some(snapshot) = &def.snapshot {
            let type_code = generate_struct(
                &def.name,
                &def.description
                    .as_ref()
                    .or(snapshot.element.get(0).and_then(|e| e.short.as_ref())),
                &snapshot.element,
                &def.name,
            );
            output.push_str(&type_code);
            output.push_str("\n\n");
        }
    }

    Ok(output)
}

/// Detect BackboneElements in a resource
fn find_backbone_elements(elements: &[ElementDefinition], base_path: &str) -> Vec<String> {
    let mut backbones = Vec::new();

    for element in elements {
        // Check if this is a BackboneElement type
        if element.element_type.len() == 1 && element.element_type[0].code == "BackboneElement" {
            // Only include if it's a direct child or nested within base_path
            if element.path.starts_with(&format!("{}.", base_path)) || element.path == base_path {
                backbones.push(element.path.clone());
            }
        }
    }

    backbones
}

/// Generate nested BackboneElement struct
fn generate_backbone_struct(
    backbone_path: &str,
    elements: &[ElementDefinition],
    _resource_name: &str,
) -> String {
    // Create struct name from path: Bundle.entry -> BundleEntry
    let parts: Vec<&str> = backbone_path.split('.').collect();
    let struct_name = parts
        .iter()
        .map(|p| to_pascal_case(p))
        .collect::<Vec<_>>()
        .join("");

    // Get description from the BackboneElement itself
    let doc = elements
        .iter()
        .find(|e| e.path == backbone_path)
        .and_then(|e| e.short.as_ref());

    generate_struct(&struct_name, &doc, elements, backbone_path)
}

/// Generate resource files from resource definitions
pub fn generate_resources(
    definitions: &[StructureDefinition],
    version: &str,
) -> Result<HashMap<String, String>> {
    let mut files = HashMap::new();

    for def in definitions {
        if def.kind.as_deref() == Some("resource") {
            if let Some(snapshot) = &def.snapshot {
                // Find all BackboneElements in this resource
                let backbone_paths = find_backbone_elements(&snapshot.element, &def.name);

                // Generate main resource file
                let mut file_content = String::new();

                // File header
                file_content.push_str(&format!(
                    "//! FHIR {} {} Resource\n",
                    version.to_uppercase(),
                    def.name
                ));
                file_content.push_str("//!\n");
                file_content.push_str("//! Auto-generated from FHIR schema definitions.\n");
                file_content.push_str("//! Do not modify directly.\n\n");
                file_content.push_str("use serde::{Deserialize, Serialize};\n");
                file_content.push_str(&format!("use crate::{}::types::*;\n\n", version));

                // Generate BackboneElement structs inline in the same file
                for backbone_path in &backbone_paths {
                    let backbone_code =
                        generate_backbone_struct(backbone_path, &snapshot.element, &def.name);
                    file_content.push_str(&backbone_code);
                    file_content.push_str("\n\n");
                }

                // Generate main struct
                let struct_code = generate_struct_with_resource_type(
                    &def.name,
                    &def.description
                        .as_ref()
                        .or(snapshot.element.get(0).and_then(|e| e.short.as_ref())),
                    &snapshot.element,
                    &def.name,
                    &def.name, // Pass resource name as resourceType
                );
                file_content.push_str(&struct_code);

                // Convert resource name to file name (PascalCase -> snake_case)
                let file_name = to_snake_case(&def.name);
                files.insert(file_name, file_content);
            }
        }
    }

    Ok(files)
}

/// Generate a struct from element definitions
fn generate_struct(
    name: &str,
    doc: &Option<&String>,
    elements: &[ElementDefinition],
    base_path: &str,
) -> String {
    generate_struct_internal(name, doc, elements, base_path, None)
}

/// Generate a struct from element definitions with optional resource type
fn generate_struct_with_resource_type(
    name: &str,
    doc: &Option<&String>,
    elements: &[ElementDefinition],
    base_path: &str,
    resource_type: &str,
) -> String {
    generate_struct_internal(name, doc, elements, base_path, Some(resource_type))
}

/// Internal struct generation with optional resource type field
fn generate_struct_internal(
    name: &str,
    doc: &Option<&String>,
    elements: &[ElementDefinition],
    base_path: &str,
    resource_type: Option<&str>,
) -> String {
    let mut output = String::new();

    // Doc comment
    if let Some(description) = doc {
        let sanitized = sanitize_doc_comment(description);
        if !sanitized.is_empty() {
            output.push_str(&format!("/// {}\n", sanitized));
        }
    }

    // Struct definition
    output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
    output.push_str("#[serde(rename_all = \"camelCase\")]\n");
    output.push_str(&format!("pub struct {} {{\n", name));

    // Add resourceType field for resources
    if let Some(_rt) = resource_type {
        output.push_str("    /// The type of resource\n");
        output.push_str("    #[serde(rename = \"resourceType\")]\n");
        output.push_str(&format!("    pub resource_type: String,\n\n"));
    }

    // Generate fields - only direct children of the base path
    let base_depth = base_path.chars().filter(|&c| c == '.').count();
    let mut generated_fields = std::collections::HashSet::new();

    for element in elements {
        // Skip the base element itself
        if element.path == base_path {
            continue;
        }

        // Only process immediate children
        if !element.path.starts_with(&format!("{}.", base_path)) {
            continue;
        }

        // Calculate depth
        let element_depth = element.path.chars().filter(|&c| c == '.').count();
        if element_depth != base_depth + 1 {
            continue;
        }

        // Extract field name
        let field_name = element
            .path
            .split('.')
            .last()
            .unwrap()
            .trim_end_matches("[x]");

        // Skip if already generated (for choice types)
        if !generated_fields.insert(field_name.to_string()) {
            continue;
        }

        // Generate field
        let field_code = generate_field(element, base_path);
        output.push_str(&field_code);
    }

    output.push_str("}\n");

    output
}

/// Generate a single field
fn generate_field(element: &ElementDefinition, _base_path: &str) -> String {
    let mut output = String::new();

    // Extract field name from path
    let field_name_orig = element
        .path
        .split('.')
        .last()
        .unwrap()
        .trim_end_matches("[x]");

    let field_name = to_snake_case(field_name_orig);
    let field_name_escaped = escape_keyword(&field_name);

    // Doc comment
    if let Some(short) = &element.short {
        let sanitized = sanitize_doc_comment(short);
        if !sanitized.is_empty() {
            output.push_str(&format!("    /// {}\n", sanitized));
        }
    }

    // Serde rename if needed (but NOT for choice types, as they get specific renames)
    let is_choice_type = element.element_type.len() > 1;
    if !is_choice_type && (field_name != field_name_orig || field_name_escaped.starts_with("r#")) {
        output.push_str(&format!("    #[serde(rename = \"{}\")]\n", field_name_orig));
    }

    // Determine type
    let rust_type = if let Some(content_ref) = &element.content_reference {
        // Content reference (e.g., "#Bundle.link") - extract referenced type name
        let ref_path = content_ref.trim_start_matches('#');
        let parts: Vec<&str> = ref_path.split('.').collect();
        let type_name = parts
            .iter()
            .map(|p| to_pascal_case(p))
            .collect::<Vec<_>>()
            .join("");

        apply_cardinality(&type_name, element.min, &element.max)
    } else if element.element_type.is_empty() {
        // No type and no content reference - use fallback
        apply_cardinality("serde_json::Value", element.min, &element.max)
    } else if element.element_type.len() == 1 {
        // Single type
        let fhir_type = &element.element_type[0].code;

        // Handle BackboneElement
        let mut rust_base = if fhir_type == "BackboneElement" {
            // Generate struct name from path
            let parts: Vec<&str> = element.path.split('.').collect();
            parts
                .iter()
                .map(|p| to_pascal_case(p))
                .collect::<Vec<_>>()
                .join("")
        } else {
            map_primitive_type(fhir_type)
        };

        // Wrap recursive types in Box to prevent infinite size
        if rust_base == "Reference" || rust_base == "Identifier" {
            rust_base = format!("Box<{}>", rust_base);
        }

        apply_cardinality(&rust_base, element.min, &element.max)
    } else {
        // Choice type - generate multiple fields, one for each type
        // For example, Observation.instantiates[x] with types [canonical, Reference]
        // generates: instantiates_canonical and instantiates_reference

        for type_ref in &element.element_type {
            let fhir_type = &type_ref.code;
            let mut rust_base = map_primitive_type(fhir_type);

            // Wrap recursive types in Box
            if rust_base == "Reference" || rust_base == "Identifier" {
                rust_base = format!("Box<{}>", rust_base);
            }

            let rust_type_for_choice = apply_cardinality(&rust_base, element.min, &element.max);

            // Create field name by appending the type: instantiates_canonical, instantiates_reference
            let type_suffix = to_pascal_case(fhir_type);
            let choice_field_name = format!("{}_{}", field_name, to_snake_case(&type_suffix));
            let choice_field_name_escaped = escape_keyword(&choice_field_name);

            // Serde rename for choice fields (e.g., instantiatesCanonical)
            let json_field_name = format!("{}{}", field_name_orig, type_suffix);
            output.push_str(&format!("    #[serde(rename = \"{}\")]\n", json_field_name));
            output.push_str(&format!(
                "    pub {}: {},\n",
                choice_field_name_escaped, rust_type_for_choice
            ));
            output.push('\n');
        }

        return output;
    };

    output.push_str(&format!("    pub {}: {},\n", field_name_escaped, rust_type));
    output.push('\n');

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_field() {
        let element = ElementDefinition {
            id: "Patient.active".to_string(),
            path: "Patient.active".to_string(),
            min: 0,
            max: "1".to_string(),
            element_type: vec![TypeRef {
                code: "boolean".to_string(),
                target_profile: vec![],
            }],
            short: Some("Whether this patient record is in active use".to_string()),
            definition: None,
            binding: None,
            content_reference: None,
        };

        let field = generate_field(&element, "Patient");
        assert!(field.contains("pub active: Option<bool>"));
        assert!(field.contains("/// Whether this patient record is in active use"));
    }
}
