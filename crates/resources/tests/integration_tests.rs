//! Integration Tests
//!
//! Cross-version and integration tests for common FHIR patterns.
//! These tests verify consistency across R4, R4B, and R5 implementations.

use serde_json;

// =============================================================================
// Cross-Version Patient Tests
// =============================================================================

#[test]
fn test_cross_version_patient_basic_structure() {
    // R4 Patient
    let r4_json = r#"{
        "resourceType": "Patient",
        "id": "cross-test",
        "active": true,
        "gender": "male"
    }"#;
    
    let r4_patient: fhir_resources::r4::patient::Patient = 
        serde_json::from_str(r4_json).expect("R4 Patient deserialization failed");
    
    assert_eq!(r4_patient.resource_type, "Patient");
    assert_eq!(r4_patient.id, Some("cross-test".to_string()));
    
    // R4B Patient
    let r4b_patient: fhir_resources::r4b::patient::Patient = 
        serde_json::from_str(r4_json).expect("R4B Patient deserialization failed");
    
    assert_eq!(r4b_patient.resource_type, "Patient");
    assert_eq!(r4b_patient.id, Some("cross-test".to_string()));
    
    // R5 Patient
    let r5_patient: fhir_resources::r5::patient::Patient = 
        serde_json::from_str(r4_json).expect("R5 Patient deserialization failed");
    
    assert_eq!(r5_patient.resource_type, "Patient");
    assert_eq!(r5_patient.id, Some("cross-test".to_string()));
}

#[test]
fn test_cross_version_patient_with_name() {
    let json = r#"{
        "resourceType": "Patient",
        "id": "name-test",
        "name": [{
            "family": "Cross",
            "given": ["Version", "Test"]
        }],
        "gender": "female",
        "birthDate": "1985-01-15"
    }"#;
    
    // All versions should handle this
    let r4: fhir_resources::r4::patient::Patient = serde_json::from_str(json).unwrap();
    let r4b: fhir_resources::r4b::patient::Patient = serde_json::from_str(json).unwrap();
    let r5: fhir_resources::r5::patient::Patient = serde_json::from_str(json).unwrap();
    
    assert_eq!(r4.name.as_ref().unwrap()[0].family, Some("Cross".to_string()));
    assert_eq!(r4b.name.as_ref().unwrap()[0].family, Some("Cross".to_string()));
    assert_eq!(r5.name.as_ref().unwrap()[0].family, Some("Cross".to_string()));
}

// =============================================================================
// Cross-Version Bundle Tests
// =============================================================================

#[test]
fn test_cross_version_bundle_empty() {
    let json = r#"{
        "resourceType": "Bundle",
        "type": "collection"
    }"#;
    
    let r4: fhir_resources::r4::bundle::Bundle = serde_json::from_str(json).unwrap();
    let r4b: fhir_resources::r4b::bundle::Bundle = serde_json::from_str(json).unwrap();
    let r5: fhir_resources::r5::bundle::Bundle = serde_json::from_str(json).unwrap();
    
    assert_eq!(r4.r#type, "collection");
    assert_eq!(r4b.r#type, "collection");
    assert_eq!(r5.r#type, "collection");
}

#[test]
fn test_cross_version_bundle_with_links() {
    let json = r#"{
        "resourceType": "Bundle",
        "type": "searchset",
        "link": [{
            "relation": "self",
            "url": "https://example.org/fhir/Patient"
        }]
    }"#;
    
    let r4: fhir_resources::r4::bundle::Bundle = serde_json::from_str(json).unwrap();
    let r4b: fhir_resources::r4b::bundle::Bundle = serde_json::from_str(json).unwrap();
    let r5: fhir_resources::r5::bundle::Bundle = serde_json::from_str(json).unwrap();
    
    assert!(r4.link.is_some());
    assert!(r4b.link.is_some());
    assert!(r5.link.is_some());
    
    assert_eq!(r4.link.unwrap()[0].relation, "self");
    assert_eq!(r4b.link.unwrap()[0].relation, "self");
    assert_eq!(r5.link.unwrap()[0].relation, "self");
}

// =============================================================================
// Cross-Version Observation Tests
// =============================================================================

#[test]
fn test_cross_version_observation_basic() {
    let json = r#"{
        "resourceType": "Observation",
        "status": "final",
        "code": {
            "text": "Heart Rate"
        }
    }"#;
    
    let r4: fhir_resources::r4::observation::Observation = serde_json::from_str(json).unwrap();
    let r4b: fhir_resources::r4b::observation::Observation = serde_json::from_str(json).unwrap();
    let r5: fhir_resources::r5::observation::Observation = serde_json::from_str(json).unwrap();
    
    assert_eq!(r4.status, "final");
    assert_eq!(r4b.status, "final");
    assert_eq!(r5.status, "final");
    
    assert_eq!(r4.code.text, Some("Heart Rate".to_string()));
    assert_eq!(r4b.code.text, Some("Heart Rate".to_string()));
    assert_eq!(r5.code.text, Some("Heart Rate".to_string()));
}

// =============================================================================
// Common Pattern Tests
// =============================================================================

#[test]
fn test_patient_minimal_required_fields() {
    use fhir_resources::r5::patient::Patient;
    
    // Test that we can create a minimal patient with only required fields
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: None,
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: None,
        name: None,
        telecom: None,
        gender: None,
        birth_date: None,
        deceased: None,
        address: None,
        marital_status: None,
        multiple_birth: None,
        photo: None,
        contact: None,
        communication: None,
        general_practitioner: None,
        managing_organization: None,
        link: None,
    };
    
    let json = serde_json::to_string(&patient).expect("Failed to serialize minimal patient");
    assert!(json.contains("\"resourceType\":\"Patient\""));
}

#[test]
fn test_bundle_types() {
    use fhir_resources::r5::bundle::Bundle;
    
    let types = vec![
        "document",
        "message",
        "transaction",
        "transaction-response",
        "batch",
        "batch-response",
        "history",
        "searchset",
        "collection",
    ];
    
    for bundle_type in types {
        let bundle = Bundle {
            resource_type: "Bundle".to_string(),
            id: None,
            meta: None,
            implicit_rules: None,
            language: None,
            identifier: None,
            r#type: bundle_type.to_string(),
            timestamp: None,
            total: None,
            link: None,
            entry: None,
            signature: None,
            issues: None,
        };
        
        let json = serde_json::to_string(&bundle).expect("Failed to serialize bundle");
        let deserialized: Bundle = serde_json::from_str(&json).expect("Failed to deserialize bundle");
        
        assert_eq!(deserialized.r#type, bundle_type);
    }
}

#[test]
fn test_observation_statuses() {
    use fhir_resources::r5::observation::Observation;
    use fhir_resources::r5::types::CodeableConcept;
    
    let statuses = vec![
        "registered",
        "preliminary",
        "final",
        "amended",
        "corrected",
        "cancelled",
        "entered-in-error",
        "unknown",
    ];
    
    for status in statuses {
        let obs = Observation {
            resource_type: "Observation".to_string(),
            id: None,
            meta: None,
            implicit_rules: None,
            language: None,
            text: None,
            contained: None,
            extension: None,
            modifier_extension: None,
            identifier: None,
            instantiates_canonical: None,
            instantiates_reference: None,
            based_on: None,
            triggered_by: None,
            part_of: None,
            status: status.to_string(),
            category: None,
            code: CodeableConcept {
                id: None,
                extension: None,
                coding: None,
                text: Some("Test".to_string()),
            },
            subject: None,
            focus: None,
            encounter: None,
            effective: None,
            issued: None,
            performer: None,
            value: None,
            data_absent_reason: None,
            interpretation: None,
            note: None,
            body_site: None,
            body_structure: None,
            method: None,
            specimen: None,
            device: None,
            reference_range: None,
            has_member: None,
            derived_from: None,
            component: None,
        };
        
        let json = serde_json::to_string(&obs).expect("Failed to serialize");
        let deserialized: Observation = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(deserialized.status, status);
    }
}

// =============================================================================
// Reference and Identifier Tests
// =============================================================================

#[test]
fn test_reference_patterns() {
    use fhir_resources::r5::types::Reference;
    
    // Test different reference patterns
    let refs = vec![
        ("Patient/123", None, None),
        ("Practitioner/456", Some("Practitioner".to_string()), None),
        ("Organization/789", None, Some("Main Hospital".to_string())),
        ("https://example.org/fhir/Patient/abc", None, None),
        ("urn:uuid:53fefa32-fcbb-4ff8-8a92-55ee120877b7", None, None),
    ];
    
    for (reference, type_val, display) in refs {
        let ref_obj = Reference {
            id: None,
            extension: None,
            reference: Some(reference.to_string()),
            r#type: type_val,
            identifier: None,
            display: display,
        };
        
        let json = serde_json::to_string(&ref_obj).expect("Failed to serialize reference");
        let deserialized: Reference = serde_json::from_str(&json).expect("Failed to deserialize reference");
        
        assert_eq!(deserialized.reference, Some(reference.to_string()));
    }
}

#[test]
fn test_identifier_system_value() {
    use fhir_resources::r5::types::Identifier;
    
    let identifier = Identifier {
        id: None,
        extension: None,
        r#use: Some("official".to_string()),
        r#type: None,
        system: Some("http://example.org/identifiers".to_string()),
        value: Some("12345".to_string()),
        period: None,
        assigner: None,
    };
    
    let json = serde_json::to_string(&identifier).expect("Failed to serialize");
    let deserialized: Identifier = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(deserialized.system, Some("http://example.org/identifiers".to_string()));
    assert_eq!(deserialized.value, Some("12345".to_string()));
}

// =============================================================================
// CodeableConcept and Coding Tests
// =============================================================================

#[test]
fn test_codeable_concept_with_coding() {
    use fhir_resources::r5::types::{CodeableConcept, Coding};
    
    let concept = CodeableConcept {
        id: None,
        extension: None,
        coding: Some(vec![
            Coding {
                id: None,
                extension: None,
                system: Some("http://snomed.info/sct".to_string()),
                version: None,
                code: Some("38341003".to_string()),
                display: Some("Hypertension".to_string()),
                user_selected: None,
            },
            Coding {
                id: None,
                extension: None,
                system: Some("http://hl7.org/fhir/sid/icd-10".to_string()),
                version: None,
                code: Some("I10".to_string()),
                display: Some("Essential (primary) hypertension".to_string()),
                user_selected: None,
            },
        ]),
        text: Some("High Blood Pressure".to_string()),
    };
    
    let json = serde_json::to_string(&concept).expect("Failed to serialize");
    let deserialized: CodeableConcept = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(deserialized.coding.as_ref().unwrap().len(), 2);
    assert_eq!(deserialized.text, Some("High Blood Pressure".to_string()));
}

// =============================================================================
// Complex Nested Structure Tests
// =============================================================================

#[test]
fn test_patient_with_contact() {
    use fhir_resources::r5::patient::Patient;
    use fhir_resources::r5::types::{HumanName, ContactPoint, Address};
    
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("complex-patient".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(true),
        name: Some(vec![
            HumanName {
                id: None,
                extension: None,
                r#use: Some("official".to_string()),
                text: None,
                family: Some("Smith".to_string()),
                given: Some(vec!["John".to_string(), "William".to_string()]),
                prefix: None,
                suffix: None,
                period: None,
            },
        ]),
        telecom: Some(vec![
            ContactPoint {
                id: None,
                extension: None,
                system: Some("phone".to_string()),
                value: Some("+1-555-123-4567".to_string()),
                r#use: Some("home".to_string()),
                rank: Some(1),
                period: None,
            },
            ContactPoint {
                id: None,
                extension: None,
                system: Some("email".to_string()),
                value: Some("john.smith@example.com".to_string()),
                r#use: Some("home".to_string()),
                rank: Some(2),
                period: None,
            },
        ]),
        gender: Some("male".to_string()),
        birth_date: Some("1980-05-15".to_string()),
        deceased: None,
        address: Some(vec![
            Address {
                id: None,
                extension: None,
                r#use: Some("home".to_string()),
                r#type: Some("physical".to_string()),
                text: Some("123 Main Street, Apartment 4B, Springfield, IL 62701".to_string()),
                line: Some(vec!["123 Main Street".to_string(), "Apartment 4B".to_string()]),
                city: Some("Springfield".to_string()),
                district: None,
                state: Some("IL".to_string()),
                postal_code: Some("62701".to_string()),
                country: Some("USA".to_string()),
                period: None,
            },
        ]),
        marital_status: None,
        multiple_birth: None,
        photo: None,
        contact: None,
        communication: None,
        general_practitioner: None,
        managing_organization: None,
        link: None,
    };
    
    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&patient).expect("Failed to serialize");
    let deserialized: Patient = serde_json::from_str(&json).expect("Failed to deserialize");
    
    // Verify all complex structures
    assert_eq!(deserialized.name.as_ref().unwrap()[0].given.as_ref().unwrap().len(), 2);
    assert_eq!(deserialized.telecom.as_ref().unwrap().len(), 2);
    assert_eq!(deserialized.address.as_ref().unwrap()[0].line.as_ref().unwrap().len(), 2);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_arrays() {
    use fhir_resources::r5::patient::Patient;
    
    let json = r#"{
        "resourceType": "Patient",
        "name": [],
        "telecom": [],
        "address": []
    }"#;
    
    let patient: Patient = serde_json::from_str(json).expect("Failed to deserialize");
    
    // Empty arrays should deserialize as Some(vec![])
    assert!(patient.name.is_some());
    assert_eq!(patient.name.unwrap().len(), 0);
}

#[test]
fn test_null_vs_missing_fields() {
    use fhir_resources::r5::patient::Patient;
    
    let json_with_null = r#"{
        "resourceType": "Patient",
        "active": null
    }"#;
    
    let json_without = r#"{
        "resourceType": "Patient"
    }"#;
    
    let patient_null: Patient = serde_json::from_str(json_with_null).expect("Failed with null");
    let patient_missing: Patient = serde_json::from_str(json_without).expect("Failed without field");
    
    // Both should result in None for the active field
    assert_eq!(patient_null.active, None);
    assert_eq!(patient_missing.active, None);
}

#[test]
fn test_very_long_strings() {
    use fhir_resources::r5::patient::Patient;
    use fhir_resources::r5::types::HumanName;
    
    let very_long_name = "A".repeat(1000);
    
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("long-string-test".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: None,
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: None,
            text: Some(very_long_name.clone()),
            family: None,
            given: None,
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: None,
        birth_date: None,
        deceased: None,
        address: None,
        marital_status: None,
        multiple_birth: None,
        photo: None,
        contact: None,
        communication: None,
        general_practitioner: None,
        managing_organization: None,
        link: None,
    };
    
    let json = serde_json::to_string(&patient).expect("Failed to serialize");
    let deserialized: Patient = serde_json::from_str(&json).expect("Failed to deserialize");
    
    assert_eq!(
        deserialized.name.as_ref().unwrap()[0].text.as_ref().unwrap().len(),
        1000
    );
}
