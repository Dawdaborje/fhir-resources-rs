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
        "integer64" | "Integer64" => "i64".to_string(), // R5 addition
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

/// Convert FHIR type name to Rust type name (PascalCase)
pub fn to_pascal_case(s: &str) -> String {
    // If already starts with uppercase, assume it's already PascalCase
    if s.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
        return s.to_string();
    }
    
    // Otherwise, capitalize first letter
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
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
