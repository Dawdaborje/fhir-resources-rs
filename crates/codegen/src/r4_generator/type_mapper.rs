//! Type Mapping Utilities
//!
//! Maps FHIR types to Rust types and handles cardinality.

/// Map FHIR primitive type to Rust type
pub fn map_primitive_type(fhir_type: &str) -> String {
    // Handle FHIR system URIs (e.g., http://hl7.org/fhirpath/System.String)
    if fhir_type.starts_with("http://hl7.org/fhirpath/System.") {
        let type_name = fhir_type.split('.').last().unwrap_or(fhir_type);
        return map_primitive_type(type_name);
    }

    match fhir_type {
        // Primitives
        "string" | "String" | "uri" | "url" | "canonical" | "oid" | "uuid" | "code" | "id"
        | "markdown" | "base64Binary" | "instant" | "Instant" | "date" | "Date" | "dateTime"
        | "DateTime" | "time" | "Time" | "xhtml" => "String".to_string(),

        "boolean" | "Boolean" => "bool".to_string(),
        "integer" | "Integer" | "positiveInt" | "PositiveInt" => "i32".to_string(),
        "unsignedInt" | "UnsignedInt" => "u32".to_string(),
        "decimal" | "Decimal" => "f64".to_string(),

        // Polymorphic resource type - use serde_json::Value as placeholder
        // TODO: Generate proper Resource enum with all resource types
        "Resource" => "serde_json::Value".to_string(),

        // Complex types - use as-is
        _ => fhir_type.to_string(),
    }
}

/// Apply cardinality wrapping to a Rust type
pub fn apply_cardinality(rust_type: &str, min: u32, max: &str) -> String {
    match (min, max) {
        (0, "1") => format!("Option<{}>", rust_type),
        (0, "*") => format!("Option<Vec<{}>>", rust_type),
        (1, "1") => rust_type.to_string(),
        (1, "*") => format!("Vec<{}>", rust_type),
        _ => {
            // Handle specific numeric max values
            if max == "*" || max.parse::<u32>().unwrap_or(0) > 1 {
                if min == 0 {
                    format!("Option<Vec<{}>>", rust_type)
                } else {
                    format!("Vec<{}>", rust_type)
                }
            } else {
                if min == 0 {
                    format!("Option<{}>", rust_type)
                } else {
                    rust_type.to_string()
                }
            }
        }
    }
}

/// Convert FHIR field name to Rust field name (camelCase to snake_case)
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_lower = false;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 && prev_lower {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
            prev_lower = ch.is_lowercase();
        }
    }

    result
}

/// Convert FHIR type name to Rust type name (already PascalCase)
pub fn to_pascal_case(s: &str) -> String {
    s.to_string()
}

/// Check if a string is a Rust reserved keyword
pub fn is_reserved_keyword(s: &str) -> bool {
    matches!(
        s,
        "type"
            | "use"
            | "match"
            | "ref"
            | "box"
            | "fn"
            | "impl"
            | "trait"
            | "struct"
            | "enum"
            | "mod"
            | "pub"
            | "crate"
            | "self"
            | "Self"
            | "super"
            | "async"
            | "await"
            | "break"
            | "const"
            | "continue"
            | "dyn"
            | "else"
            | "extern"
            | "false"
            | "for"
            | "if"
            | "in"
            | "let"
            | "loop"
            | "move"
            | "mut"
            | "return"
            | "static"
            | "true"
            | "where"
            | "while"
            | "abstract"
            | "become"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

/// Escape Rust field name if it's a reserved keyword
pub fn escape_keyword(s: &str) -> String {
    if is_reserved_keyword(s) {
        format!("r#{}", s)
    } else {
        s.to_string()
    }
}

/// Detect if an element path ends with [x] (choice type)
pub fn is_choice_type(path: &str) -> bool {
    path.ends_with("[x]")
}

/// Extract base name from choice type path (e.g., "Patient.deceased[x]" -> "deceased")
pub fn choice_type_base_name(path: &str) -> String {
    if let Some(pos) = path.rfind('.') {
        let field = &path[pos + 1..];
        field.trim_end_matches("[x]").to_string()
    } else {
        path.trim_end_matches("[x]").to_string()
    }
}

/// Generate enum name for choice type (e.g., "deceased" -> "DeceasedChoice")
pub fn choice_enum_name(base_name: &str) -> String {
    let pascal = if base_name.chars().next().unwrap().is_lowercase() {
        let mut chars = base_name.chars();
        let first = chars.next().unwrap().to_uppercase().collect::<String>();
        first + chars.as_str()
    } else {
        base_name.to_string()
    };
    format!("{}Choice", pascal)
}

/// Parse enum variants from ValueSet binding description
pub fn parse_enum_variants(description: &str) -> Vec<String> {
    description
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            // Convert to PascalCase, handling kebab-case
            s.split('-')
                .map(|part| {
                    let mut chars = part.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                })
                .collect::<String>()
        })
        .collect()
}

/// Extract resource name from target profile URL
pub fn extract_resource_from_profile(profile: &str) -> Option<String> {
    profile.split('/').last().map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_primitive_type() {
        assert_eq!(map_primitive_type("string"), "String");
        assert_eq!(map_primitive_type("boolean"), "bool");
        assert_eq!(map_primitive_type("integer"), "i32");
        assert_eq!(map_primitive_type("HumanName"), "HumanName");
    }

    #[test]
    fn test_apply_cardinality() {
        assert_eq!(apply_cardinality("String", 0, "1"), "Option<String>");
        assert_eq!(apply_cardinality("String", 0, "*"), "Option<Vec<String>>");
        assert_eq!(apply_cardinality("String", 1, "1"), "String");
        assert_eq!(apply_cardinality("String", 1, "*"), "Vec<String>");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("birthDate"), "birth_date");
        assert_eq!(
            to_snake_case("managingOrganization"),
            "managing_organization"
        );
        assert_eq!(to_snake_case("id"), "id");
    }

    #[test]
    fn test_escape_keyword() {
        assert_eq!(escape_keyword("type"), "r#type");
        assert_eq!(escape_keyword("use"), "r#use");
        assert_eq!(escape_keyword("name"), "name");
    }

    #[test]
    fn test_is_choice_type() {
        assert!(is_choice_type("Patient.deceased[x]"));
        assert!(!is_choice_type("Patient.name"));
    }

    #[test]
    fn test_choice_type_base_name() {
        assert_eq!(choice_type_base_name("Patient.deceased[x]"), "deceased");
    }

    #[test]
    fn test_choice_enum_name() {
        assert_eq!(choice_enum_name("deceased"), "DeceasedChoice");
        assert_eq!(choice_enum_name("value"), "ValueChoice");
    }

    #[test]
    fn test_parse_enum_variants() {
        let variants = parse_enum_variants("home | work | temp | old");
        assert_eq!(variants, vec!["Home", "Work", "Temp", "Old"]);

        let variants = parse_enum_variants("entered-in-error | on-hold");
        assert_eq!(variants, vec!["EnteredInError", "OnHold"]);
    }
}
