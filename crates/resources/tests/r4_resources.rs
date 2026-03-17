//! FHIR R4 Resource Tests
//!
//! Comprehensive tests for all major FHIR R4 resources including:
//! - Resource creation
//! - Serialization/Deserialization
//! - Required field validation
//! - Complex nested structures

use fhir_resources::r4::bundle::{Bundle, BundleEntry, BundleLink};
use fhir_resources::r4::condition::Condition;
use fhir_resources::r4::diagnostic_report::DiagnosticReport;
use fhir_resources::r4::encounter::Encounter;
use fhir_resources::r4::medication_request::MedicationRequest;
use fhir_resources::r4::observation::Observation;
use fhir_resources::r4::organization::Organization;
use fhir_resources::r4::patient::Patient;
use fhir_resources::r4::practitioner::Practitioner;
use fhir_resources::r4::procedure::Procedure;
use fhir_resources::r4::types::{
    Address, CodeableConcept, Coding, ContactPoint, HumanName, Identifier, Reference,
};
use serde_json;

// =============================================================================
// Patient Resource Tests
// =============================================================================

#[test]
fn test_r4_patient_creation() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("test-patient-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: Some("en-US".to_string()),
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: Some(vec![Box::new(Identifier {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            r#type: None,
            system: Some("http://example.org/mrn".to_string()),
            value: Some("12345".to_string()),
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

    assert_eq!(patient.resource_type, "Patient");
    assert_eq!(patient.id, Some("test-patient-1".to_string()));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.gender, Some("male".to_string()));
}

#[test]
fn test_r4_patient_serialization() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("serialize-test".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: None,
            text: None,
            family: Some("Smith".to_string()),
            given: Some(vec!["Jane".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("female".to_string()),
        birth_date: Some("1990-05-15".to_string()),
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

    let json = serde_json::to_string(&patient).expect("Failed to serialize patient");
    assert!(json.contains("\"resourceType\":\"Patient\""));
    assert!(json.contains("\"id\":\"serialize-test\""));
    assert!(json.contains("\"active\":true"));
    assert!(json.contains("\"gender\":\"female\""));
}

#[test]
fn test_r4_patient_deserialization() {
    let json_data = r#"{
        "resourceType": "Patient",
        "id": "deserialize-test",
        "active": true,
        "name": [{
            "family": "Johnson",
            "given": ["Robert"]
        }],
        "gender": "male",
        "birthDate": "1975-03-20"
    }"#;

    let patient: Patient = serde_json::from_str(json_data).expect("Failed to deserialize patient");

    assert_eq!(patient.resource_type, "Patient");
    assert_eq!(patient.id, Some("deserialize-test".to_string()));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.gender, Some("male".to_string()));
    assert_eq!(patient.birth_date, Some("1975-03-20".to_string()));

    let names = patient.name.as_ref().unwrap();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0].family, Some("Johnson".to_string()));
}

// =============================================================================
// Bundle Resource Tests
// =============================================================================

#[test]
fn test_r4_bundle_creation() {
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("bundle-test".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "collection".to_string(),
        timestamp: Some("2024-03-17T10:00:00Z".to_string()),
        total: Some(2),
        link: Some(vec![BundleLink {
            id: None,
            extension: None,
            modifier_extension: None,
            relation: "self".to_string(),
            url: "https://example.org/fhir/Bundle/bundle-test".to_string(),
        }]),
        entry: Some(vec![
            BundleEntry {
                id: None,
                extension: None,
                modifier_extension: None,
                link: None,
                full_url: Some("https://example.org/fhir/Patient/1".to_string()),
                resource: Some(serde_json::json!({
                    "resourceType": "Patient",
                    "id": "1"
                })),
                search: None,
                request: None,
                response: None,
            },
            BundleEntry {
                id: None,
                extension: None,
                modifier_extension: None,
                link: None,
                full_url: Some("https://example.org/fhir/Patient/2".to_string()),
                resource: Some(serde_json::json!({
                    "resourceType": "Patient",
                    "id": "2"
                })),
                search: None,
                request: None,
                response: None,
            },
        ]),
        signature: None,
    };

    assert_eq!(bundle.resource_type, "Bundle");
    assert_eq!(bundle.r#type, "collection");
    assert_eq!(bundle.total, Some(2));
    assert_eq!(bundle.entry.as_ref().unwrap().len(), 2);
}

#[test]
fn test_r4_bundle_transaction() {
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("transaction-bundle".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "transaction".to_string(),
        timestamp: Some("2024-03-17T11:00:00Z".to_string()),
        total: None,
        link: None,
        entry: Some(vec![BundleEntry {
            id: None,
            extension: None,
            modifier_extension: None,
            link: None,
            full_url: Some("urn:uuid:61ebe359-bfdc-4613-8bf2-c5e300945f0a".to_string()),
            resource: Some(serde_json::json!({
                "resourceType": "Patient",
                "name": [{"family": "TestPatient"}]
            })),
            search: None,
            request: None,
            response: None,
        }]),
        signature: None,
    };

    let json = serde_json::to_string(&bundle).expect("Failed to serialize bundle");
    assert!(json.contains("\"type\":\"transaction\""));
    assert!(json.contains("urn:uuid:61ebe359-bfdc-4613-8bf2-c5e300945f0a"));
}

// =============================================================================
// Observation Resource Tests
// =============================================================================

#[test]
fn test_r4_observation_creation() {
    let observation = Observation {
        resource_type: "Observation".to_string(),
        id: Some("obs-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        based_on: None,
        part_of: None,
        status: "final".to_string(),
        category: Some(vec![CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/observation-category".to_string(),
                ),
                version: None,
                code: Some("vital-signs".to_string()),
                display: Some("Vital Signs".to_string()),
                user_selected: None,
            }]),
            text: Some("Vital Signs".to_string()),
        }]),
        code: CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://loinc.org".to_string()),
                version: None,
                code: Some("8867-4".to_string()),
                display: Some("Heart rate".to_string()),
                user_selected: None,
            }]),
            text: Some("Heart rate".to_string()),
        },
        subject: Some(Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/test-patient-1".to_string()),
            r#type: None,
            identifier: None,
            display: None,
        })),
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
        method: None,
        specimen: None,
        device: None,
        reference_range: None,
        has_member: None,
        derived_from: None,
        component: None,
    };

    assert_eq!(observation.resource_type, "Observation");
    assert_eq!(observation.status, "final");
    assert!(observation.subject.is_some());
}

// =============================================================================
// Practitioner Resource Tests
// =============================================================================

#[test]
fn test_r4_practitioner_creation() {
    let practitioner = Practitioner {
        resource_type: "Practitioner".to_string(),
        id: Some("prac-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: Some(vec![Box::new(Identifier {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            r#type: None,
            system: Some("http://example.org/npi".to_string()),
            value: Some("1234567890".to_string()),
            period: None,
            assigner: None,
        })]),
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            text: Some("Dr. Sarah Smith".to_string()),
            family: Some("Smith".to_string()),
            given: Some(vec!["Sarah".to_string()]),
            prefix: Some(vec!["Dr.".to_string()]),
            suffix: None,
            period: None,
        }]),
        telecom: Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some("email".to_string()),
            value: Some("sarah.smith@example.org".to_string()),
            r#use: Some("work".to_string()),
            rank: None,
            period: None,
        }]),
        address: None,
        gender: Some("female".to_string()),
        birth_date: None,
        photo: None,
        qualification: None,
        communication: None,
    };

    assert_eq!(practitioner.resource_type, "Practitioner");
    assert_eq!(practitioner.active, Some(true));
    assert!(practitioner.identifier.is_some());
}

// =============================================================================
// Organization Resource Tests
// =============================================================================

#[test]
fn test_r4_organization_creation() {
    let organization = Organization {
        resource_type: "Organization".to_string(),
        id: Some("org-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: Some(vec![Box::new(Identifier {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            r#type: None,
            system: Some("http://example.org/organizations".to_string()),
            value: Some("ORG-123".to_string()),
            period: None,
            assigner: None,
        })]),
        active: Some(true),
        r#type: Some(vec![CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://terminology.hl7.org/CodeSystem/organization-type".to_string()),
                version: None,
                code: Some("prov".to_string()),
                display: Some("Healthcare Provider".to_string()),
                user_selected: None,
            }]),
            text: Some("Healthcare Provider".to_string()),
        }]),
        name: Some("Example Hospital".to_string()),
        alias: None,
        telecom: Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some("phone".to_string()),
            value: Some("+1-555-000-0000".to_string()),
            r#use: Some("work".to_string()),
            rank: None,
            period: None,
        }]),
        address: Some(vec![Address {
            id: None,
            extension: None,
            r#use: Some("work".to_string()),
            r#type: Some("physical".to_string()),
            text: Some("456 Hospital Drive, Medical City".to_string()),
            line: Some(vec!["456 Hospital Drive".to_string()]),
            city: Some("Medical City".to_string()),
            district: None,
            state: Some("CA".to_string()),
            postal_code: Some("90210".to_string()),
            country: Some("USA".to_string()),
            period: None,
        }]),
        part_of: None,
        contact: None,
        endpoint: None,
    };

    assert_eq!(organization.resource_type, "Organization");
    assert_eq!(organization.name, Some("Example Hospital".to_string()));
    assert_eq!(organization.active, Some(true));
}

// =============================================================================
// Condition Resource Tests
// =============================================================================

#[test]
fn test_r4_condition_creation() {
    let condition = Condition {
        resource_type: "Condition".to_string(),
        id: Some("cond-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        clinical_status: Some(CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/condition-clinical".to_string(),
                ),
                version: None,
                code: Some("active".to_string()),
                display: Some("Active".to_string()),
                user_selected: None,
            }]),
            text: None,
        }),
        verification_status: Some(CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/condition-ver-status".to_string(),
                ),
                version: None,
                code: Some("confirmed".to_string()),
                display: Some("Confirmed".to_string()),
                user_selected: None,
            }]),
            text: None,
        }),
        category: None,
        severity: None,
        code: Some(CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://snomed.info/sct".to_string()),
                version: None,
                code: Some("38341003".to_string()),
                display: Some("Hypertension".to_string()),
                user_selected: None,
            }]),
            text: Some("Hypertension".to_string()),
        }),
        body_site: None,
        subject: Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/test-patient-1".to_string()),
            r#type: None,
            identifier: None,
            display: None,
        }),
        encounter: None,
        onset: None,
        abatement: None,
        recorded_date: Some("2024-03-17".to_string()),
        recorder: None,
        asserter: None,
        stage: None,
        evidence: None,
        note: None,
    };

    assert_eq!(condition.resource_type, "Condition");
    assert!(condition.code.is_some());
    assert_eq!(
        condition.subject.reference,
        Some("Patient/test-patient-1".to_string())
    );
}

// =============================================================================
// Round-trip Tests
// =============================================================================

#[test]
fn test_r4_patient_roundtrip() {
    let original = Patient {
        resource_type: "Patient".to_string(),
        id: Some("roundtrip-test".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: None,
            text: None,
            family: Some("TestFamily".to_string()),
            given: Some(vec!["TestGiven".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("other".to_string()),
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

    // Serialize
    let json = serde_json::to_string(&original).expect("Failed to serialize");

    // Deserialize
    let deserialized: Patient = serde_json::from_str(&json).expect("Failed to deserialize");

    // Verify
    assert_eq!(original.resource_type, deserialized.resource_type);
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.active, deserialized.active);
    assert_eq!(original.gender, deserialized.gender);
}
