//! FHIR R4B Resource Tests
//!
//! Comprehensive tests for all major FHIR R4B resources.
//! R4B adds new resources and updates existing ones from R4.

use fhir_resources::r4b::bundle::{Bundle, BundleEntry, BundleLink};
use fhir_resources::r4b::citation::Citation;
use fhir_resources::r4b::evidence::Evidence;
use fhir_resources::r4b::evidence_report::EvidenceReport;
use fhir_resources::r4b::ingredient::Ingredient;
use fhir_resources::r4b::medication_knowledge::MedicationKnowledge;
use fhir_resources::r4b::observation::Observation;
use fhir_resources::r4b::organization::Organization;
use fhir_resources::r4b::patient::Patient;
use fhir_resources::r4b::practitioner::Practitioner;
use fhir_resources::r4b::types::{
    Address, CodeableConcept, Coding, ContactPoint, HumanName, Identifier, Reference,
};
use serde_json;

// =============================================================================
// Patient Resource Tests (R4B)
// =============================================================================

#[test]
fn test_r4b_patient_creation() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("r4b-patient-1".to_string()),
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
            value: Some("R4B-12345".to_string()),
            period: None,
            assigner: None,
        })]),
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            text: Some("Emily Carter".to_string()),
            family: Some("Carter".to_string()),
            given: Some(vec!["Emily".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some("email".to_string()),
            value: Some("emily.carter@example.com".to_string()),
            r#use: Some("home".to_string()),
            rank: Some(1),
            period: None,
        }]),
        gender: Some("female".to_string()),
        birth_date: Some("1985-07-15".to_string()),
        deceased: None,
        address: Some(vec![Address {
            id: None,
            extension: None,
            r#use: Some("home".to_string()),
            r#type: Some("physical".to_string()),
            text: Some("789 Oak Lane, Riverside".to_string()),
            line: Some(vec!["789 Oak Lane".to_string()]),
            city: Some("Riverside".to_string()),
            district: None,
            state: Some("NY".to_string()),
            postal_code: Some("10001".to_string()),
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
    assert_eq!(patient.id, Some("r4b-patient-1".to_string()));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.gender, Some("female".to_string()));
}

#[test]
fn test_r4b_patient_serialization() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("r4b-serialize".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(false),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: None,
            text: None,
            family: Some("Anderson".to_string()),
            given: Some(vec!["Michael".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("male".to_string()),
        birth_date: Some("1970-12-01".to_string()),
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
    assert!(json.contains("\"id\":\"r4b-serialize\""));
    assert!(json.contains("\"active\":false"));
}

// =============================================================================
// Bundle Resource Tests (R4B)
// =============================================================================

#[test]
fn test_r4b_bundle_creation() {
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("r4b-bundle-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "searchset".to_string(),
        timestamp: Some("2024-03-17T12:00:00Z".to_string()),
        total: Some(3),
        link: Some(vec![BundleLink {
            id: None,
            extension: None,
            modifier_extension: None,
            relation: "self".to_string(),
            url: "https://example.org/fhir/Patient?name=test".to_string(),
        }]),
        entry: Some(vec![BundleEntry {
            id: None,
            extension: None,
            modifier_extension: None,
            link: None,
            full_url: Some("https://example.org/fhir/Patient/p1".to_string()),
            resource: Some(serde_json::json!({
                "resourceType": "Patient",
                "id": "p1",
                "name": [{"family": "Test1"}]
            })),
            search: None,
            request: None,
            response: None,
        }]),
        signature: None,
    };

    assert_eq!(bundle.resource_type, "Bundle");
    assert_eq!(bundle.r#type, "searchset");
    assert_eq!(bundle.total, Some(3));
}

// =============================================================================
// Citation Resource Tests (New in R4B)
// =============================================================================

#[test]
fn test_r4b_citation_creation() {
    let citation = Citation {
        resource_type: "Citation".to_string(),
        id: Some("citation-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        url: Some("http://example.org/citations/1".to_string()),
        identifier: None,
        version: None,
        name: None,
        title: Some("Example Citation for Clinical Study".to_string()),
        status: "draft".to_string(),
        experimental: None,
        date: Some("2024-03-17".to_string()),
        publisher: Some("Example Publisher".to_string()),
        contact: None,
        description: Some("This is an example citation resource in R4B".to_string()),
        use_context: None,
        jurisdiction: None,
        purpose: None,
        copyright: None,
        approval_date: None,
        last_review_date: None,
        effective_period: None,
        author: None,
        editor: None,
        reviewer: None,
        endorser: None,
        summary: None,
        classification: None,
        note: None,
        current_state: None,
        status_date: None,
        related_artifact: None,
        cited_artifact: None,
    };

    assert_eq!(citation.resource_type, "Citation");
    assert_eq!(citation.status, "draft");
    assert!(citation.title.is_some());
}

// =============================================================================
// Evidence Report Tests (New in R4B)
// =============================================================================

#[test]
fn test_r4b_evidence_report_creation() {
    let evidence_report = EvidenceReport {
        resource_type: "EvidenceReport".to_string(),
        id: Some("evidence-report-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        url: Some("http://example.org/evidence-reports/1".to_string()),
        identifier: None,
        version: None,
        name: None,
        title: Some("Clinical Evidence Report".to_string()),
        status: "draft".to_string(),
        experimental: None,
        date: None,
        publisher: None,
        contact: None,
        description: None,
        use_context: None,
        jurisdiction: None,
        purpose: None,
        copyright: None,
        approval_date: None,
        last_review_date: None,
        effective_period: None,
        topic: None,
        author: None,
        editor: None,
        reviewer: None,
        endorser: None,
        related_artifact: None,
        related_identifier: None,
        cite_as: None,
        r#type: None,
        note: None,
        subject: None,
        section: None,
    };

    assert_eq!(evidence_report.resource_type, "EvidenceReport");
    assert_eq!(evidence_report.status, "draft");
}

// =============================================================================
// Ingredient Resource Tests (New in R4B)
// =============================================================================

#[test]
fn test_r4b_ingredient_creation() {
    let ingredient = Ingredient {
        resource_type: "Ingredient".to_string(),
        id: Some("ingredient-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        status: "active".to_string(),
        r#for: None,
        role: CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://hl7.org/fhir/ingredient-role".to_string()),
                version: None,
                code: Some("ActiveBase".to_string()),
                display: Some("Active Base".to_string()),
                user_selected: None,
            }]),
            text: Some("Active Base".to_string()),
        },
        function: None,
        group: None,
        allergen_indicator: None,
        comment: None,
        manufacturer: None,
        substance: None,
    };

    assert_eq!(ingredient.resource_type, "Ingredient");
    assert_eq!(ingredient.status, "active");
}

// =============================================================================
// Observation Resource Tests (R4B)
// =============================================================================

#[test]
fn test_r4b_observation_vital_signs() {
    let observation = Observation {
        resource_type: "Observation".to_string(),
        id: Some("obs-bp".to_string()),
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
            text: None,
        }]),
        code: CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://loinc.org".to_string()),
                version: None,
                code: Some("85354-9".to_string()),
                display: Some("Blood pressure".to_string()),
                user_selected: None,
            }]),
            text: Some("Blood pressure".to_string()),
        },
        subject: Some(Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/r4b-patient-1".to_string()),
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
}

// =============================================================================
// Practitioner and Organization Tests (R4B)
// =============================================================================

#[test]
fn test_r4b_practitioner_with_qualification() {
    let practitioner = Practitioner {
        resource_type: "Practitioner".to_string(),
        id: Some("prac-r4b".to_string()),
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
            r#use: Some("official".to_string()),
            text: Some("Dr. John Smith, MD".to_string()),
            family: Some("Smith".to_string()),
            given: Some(vec!["John".to_string()]),
            prefix: Some(vec!["Dr.".to_string()]),
            suffix: Some(vec!["MD".to_string()]),
            period: None,
        }]),
        telecom: None,
        address: None,
        gender: Some("male".to_string()),
        birth_date: None,
        photo: None,
        qualification: None,
        communication: None,
    };

    assert_eq!(practitioner.resource_type, "Practitioner");
    assert!(practitioner.active.unwrap());
}

#[test]
fn test_r4b_organization_healthcare_provider() {
    let organization = Organization {
        resource_type: "Organization".to_string(),
        id: Some("org-r4b".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
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
            text: None,
        }]),
        name: Some("City Medical Center".to_string()),
        alias: Some(vec!["CMC".to_string()]),
        telecom: None,
        address: None,
        part_of: None,
        contact: None,
        endpoint: None,
    };

    assert_eq!(organization.resource_type, "Organization");
    assert_eq!(organization.name, Some("City Medical Center".to_string()));
}

// =============================================================================
// Round-trip Tests
// =============================================================================

#[test]
fn test_r4b_patient_roundtrip() {
    let original = Patient {
        resource_type: "Patient".to_string(),
        id: Some("roundtrip-r4b".to_string()),
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
            family: Some("RoundtripTest".to_string()),
            given: Some(vec!["R4B".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("unknown".to_string()),
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
}

#[test]
fn test_r4b_bundle_roundtrip() {
    let original = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("bundle-roundtrip".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "collection".to_string(),
        timestamp: None,
        total: None,
        link: None,
        entry: None,
        signature: None,
    };

    let json = serde_json::to_string(&original).expect("Failed to serialize");
    let deserialized: Bundle = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(original.resource_type, deserialized.resource_type);
    assert_eq!(original.r#type, deserialized.r#type);
}
