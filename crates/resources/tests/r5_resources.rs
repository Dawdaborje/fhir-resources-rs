//! FHIR R5 Resource Tests
//!
//! Comprehensive tests for all major FHIR R5 resources.
//! R5 is the latest published version with significant updates and new resources.

use fhir_resources::r5::bundle::{Bundle, BundleEntry, BundleLink};
use fhir_resources::r5::condition::Condition;
use fhir_resources::r5::device::Device;
use fhir_resources::r5::encounter::Encounter;
use fhir_resources::r5::genomic_study::GenomicStudy;
use fhir_resources::r5::imaging_study::ImagingStudy;
use fhir_resources::r5::inventory_item::InventoryItem;
use fhir_resources::r5::medication_request::MedicationRequest;
use fhir_resources::r5::observation::Observation;
use fhir_resources::r5::organization::Organization;
use fhir_resources::r5::patient::Patient;
use fhir_resources::r5::practitioner::Practitioner;
use fhir_resources::r5::procedure::Procedure;
use fhir_resources::r5::specimen::Specimen;
use fhir_resources::r5::transport::Transport;
use fhir_resources::r5::types::{
    Address, CodeableConcept, Coding, ContactPoint, HumanName, Identifier, Reference,
};
use serde_json;

// =============================================================================
// Patient Resource Tests (R5)
// =============================================================================

#[test]
fn test_r5_patient_creation() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("r5-patient-1".to_string()),
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
            value: Some("R5-MRN-98765".to_string()),
            period: None,
            assigner: None,
        })]),
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            text: Some("Alex Johnson".to_string()),
            family: Some("Johnson".to_string()),
            given: Some(vec!["Alex".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: Some(vec![ContactPoint {
            id: None,
            extension: None,
            system: Some("phone".to_string()),
            value: Some("+1-555-999-8888".to_string()),
            r#use: Some("mobile".to_string()),
            rank: Some(1),
            period: None,
        }]),
        gender: Some("other".to_string()),
        birth_date: Some("1992-11-30".to_string()),
        deceased: None,
        address: Some(vec![Address {
            id: None,
            extension: None,
            r#use: Some("home".to_string()),
            r#type: Some("both".to_string()),
            text: Some("321 Pine Street, Apt 5B, Downtown".to_string()),
            line: Some(vec!["321 Pine Street".to_string(), "Apt 5B".to_string()]),
            city: Some("Downtown".to_string()),
            district: None,
            state: Some("WA".to_string()),
            postal_code: Some("98101".to_string()),
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
    assert_eq!(patient.id, Some("r5-patient-1".to_string()));
    assert_eq!(patient.active, Some(true));
    assert_eq!(patient.gender, Some("other".to_string()));
}

#[test]
fn test_r5_patient_with_multiple_names() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("multi-name".to_string()),
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
                text: Some("Dr. Sarah Elizabeth Jones".to_string()),
                family: Some("Jones".to_string()),
                given: Some(vec!["Sarah".to_string(), "Elizabeth".to_string()]),
                prefix: Some(vec!["Dr.".to_string()]),
                suffix: None,
                period: None,
            },
            HumanName {
                id: None,
                extension: None,
                r#use: Some("nickname".to_string()),
                text: Some("Sally".to_string()),
                family: None,
                given: Some(vec!["Sally".to_string()]),
                prefix: None,
                suffix: None,
                period: None,
            },
        ]),
        telecom: None,
        gender: Some("female".to_string()),
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

    let names = patient.name.as_ref().unwrap();
    assert_eq!(names.len(), 2);
    assert_eq!(names[0].r#use, Some("official".to_string()));
    assert_eq!(names[1].r#use, Some("nickname".to_string()));
}

// =============================================================================
// Bundle Resource Tests (R5)
// =============================================================================

#[test]
fn test_r5_bundle_transaction() {
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("r5-transaction".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "transaction".to_string(),
        timestamp: Some("2024-03-17T15:30:00Z".to_string()),
        total: None,
        link: Some(vec![BundleLink {
            id: None,
            extension: None,
            modifier_extension: None,
            relation: "self".to_string(),
            url: "https://example.org/fhir/Bundle/r5-transaction".to_string(),
        }]),
        entry: Some(vec![BundleEntry {
            id: None,
            extension: None,
            modifier_extension: None,
            link: None,
            full_url: Some("urn:uuid:a1b2c3d4-e5f6-4a5b-8c9d-0e1f2a3b4c5d".to_string()),
            resource: Some(serde_json::json!({
                "resourceType": "Patient",
                "name": [{"family": "TransactionTest"}]
            })),
            search: None,
            request: None,
            response: None,
        }]),
        signature: None,
        issues: None,
    };

    assert_eq!(bundle.resource_type, "Bundle");
    assert_eq!(bundle.r#type, "transaction");
    assert!(bundle.entry.is_some());
}

#[test]
fn test_r5_bundle_searchset() {
    let bundle = Bundle {
        resource_type: "Bundle".to_string(),
        id: Some("search-results".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "searchset".to_string(),
        timestamp: Some("2024-03-17T16:00:00Z".to_string()),
        total: Some(5),
        link: Some(vec![
            BundleLink {
                id: None,
                extension: None,
                modifier_extension: None,
                relation: "self".to_string(),
                url: "https://example.org/fhir/Patient?name=Smith".to_string(),
            },
            BundleLink {
                id: None,
                extension: None,
                modifier_extension: None,
                relation: "next".to_string(),
                url: "https://example.org/fhir/Patient?name=Smith&page=2".to_string(),
            },
        ]),
        entry: None,
        signature: None,
        issues: None,
    };

    assert_eq!(bundle.r#type, "searchset");
    assert_eq!(bundle.total, Some(5));
    assert_eq!(bundle.link.as_ref().unwrap().len(), 2);
}

// =============================================================================
// Observation Resource Tests (R5)
// =============================================================================

#[test]
fn test_r5_observation_laboratory() {
    let observation = Observation {
        resource_type: "Observation".to_string(),
        id: Some("lab-result-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        instantiates: None,
        based_on: None,
        triggered_by: None,
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
                code: Some("laboratory".to_string()),
                display: Some("Laboratory".to_string()),
                user_selected: None,
            }]),
            text: Some("Laboratory".to_string()),
        }]),
        code: CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://loinc.org".to_string()),
                version: None,
                code: Some("2339-0".to_string()),
                display: Some("Glucose".to_string()),
                user_selected: None,
            }]),
            text: Some("Glucose".to_string()),
        },
        subject: Some(Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/r5-patient-1".to_string()),
            r#type: None,
            identifier: None,
            display: None,
        })),
        focus: None,
        encounter: None,
        effective: None,
        issued: Some("2024-03-17T10:00:00Z".to_string()),
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

    assert_eq!(observation.resource_type, "Observation");
    assert_eq!(observation.status, "final");
    assert!(observation.category.is_some());
}

// =============================================================================
// Condition Resource Tests (R5)
// =============================================================================

#[test]
fn test_r5_condition_diagnosis() {
    let condition = Condition {
        resource_type: "Condition".to_string(),
        id: Some("diagnosis-1".to_string()),
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
        category: Some(vec![CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some(
                    "http://terminology.hl7.org/CodeSystem/condition-category".to_string(),
                ),
                version: None,
                code: Some("encounter-diagnosis".to_string()),
                display: Some("Encounter Diagnosis".to_string()),
                user_selected: None,
            }]),
            text: None,
        }]),
        severity: None,
        code: Some(CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://snomed.info/sct".to_string()),
                version: None,
                code: Some("73211009".to_string()),
                display: Some("Diabetes mellitus".to_string()),
                user_selected: None,
            }]),
            text: Some("Diabetes mellitus".to_string()),
        }),
        body_site: None,
        subject: Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/r5-patient-1".to_string()),
            r#type: Some("Patient".to_string()),
            identifier: None,
            display: Some("Alex Johnson".to_string()),
        }),
        encounter: None,
        onset: None,
        abatement: None,
        recorded_date: Some("2024-03-17".to_string()),
        participant: None,
        stage: None,
        evidence: None,
        note: None,
    };

    assert_eq!(condition.resource_type, "Condition");
    assert!(condition.code.is_some());
    assert!(condition.clinical_status.is_some());
}

// =============================================================================
// Practitioner and Organization Tests (R5)
// =============================================================================

#[test]
fn test_r5_practitioner_complete() {
    let practitioner = Practitioner {
        resource_type: "Practitioner".to_string(),
        id: Some("prac-r5".to_string()),
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
            system: Some("http://hl7.org/fhir/sid/us-npi".to_string()),
            value: Some("9876543210".to_string()),
            period: None,
            assigner: None,
        })]),
        active: Some(true),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("official".to_string()),
            text: Some("Dr. Maria Garcia, MD, PhD".to_string()),
            family: Some("Garcia".to_string()),
            given: Some(vec!["Maria".to_string()]),
            prefix: Some(vec!["Dr.".to_string()]),
            suffix: Some(vec!["MD".to_string(), "PhD".to_string()]),
            period: None,
        }]),
        telecom: Some(vec![
            ContactPoint {
                id: None,
                extension: None,
                system: Some("email".to_string()),
                value: Some("maria.garcia@hospital.org".to_string()),
                r#use: Some("work".to_string()),
                rank: Some(1),
                period: None,
            },
            ContactPoint {
                id: None,
                extension: None,
                system: Some("phone".to_string()),
                value: Some("+1-555-111-2222".to_string()),
                r#use: Some("work".to_string()),
                rank: Some(2),
                period: None,
            },
        ]),
        gender: Some("female".to_string()),
        birth_date: Some("1978-06-15".to_string()),
        deceased: None,
        address: None,
        photo: None,
        qualification: None,
        communication: None,
    };

    assert_eq!(practitioner.resource_type, "Practitioner");
    assert_eq!(practitioner.active, Some(true));
    assert_eq!(practitioner.telecom.as_ref().unwrap().len(), 2);
}

#[test]
fn test_r5_organization_with_contacts() {
    let organization = Organization {
        resource_type: "Organization".to_string(),
        id: Some("org-r5".to_string()),
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
                code: Some("dept".to_string()),
                display: Some("Hospital Department".to_string()),
                user_selected: None,
            }]),
            text: None,
        }]),
        name: Some("Cardiology Department".to_string()),
        alias: None,
        description: Some("Specialized cardiology services".to_string()),
        contact: None,
        part_of: Some(Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Organization/main-hospital".to_string()),
            r#type: None,
            identifier: None,
            display: Some("Main Hospital".to_string()),
        })),
        endpoint: None,
        qualification: None,
    };

    assert_eq!(organization.resource_type, "Organization");
    assert_eq!(organization.name, Some("Cardiology Department".to_string()));
    assert!(organization.part_of.is_some());
}

// =============================================================================
// New R5 Resources Tests
// =============================================================================

#[test]
fn test_r5_inventory_item() {
    let inventory_item = InventoryItem {
        resource_type: "InventoryItem".to_string(),
        id: Some("inv-item-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        status: "active".to_string(),
        category: None,
        code: None,
        name: None,
        responsible_organization: None,
        description: None,
        inventory_status: None,
        base_unit: None,
        net_content: None,
        association: None,
        characteristic: None,
        instance: None,
        product_reference: None,
    };

    assert_eq!(inventory_item.resource_type, "InventoryItem");
    assert_eq!(inventory_item.status, "active");
}

#[test]
fn test_r5_transport() {
    let transport = Transport {
        resource_type: "Transport".to_string(),
        id: Some("transport-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        instantiates_canonical: None,
        instantiates_uri: None,
        based_on: None,
        group_identifier: None,
        part_of: None,
        status: "in-progress".to_string(),
        status_reason: None,
        intent: "order".to_string(),
        priority: None,
        code: None,
        description: Some("Transport medical specimen to lab".to_string()),
        focus: None,
        r#for: None,
        encounter: None,
        completion_time: None,
        authored_on: Some("2024-03-17T14:00:00Z".to_string()),
        last_modified: None,
        requester: None,
        requested_location: None,
        current_location: None,
        reason: None,
        history: None,
        restriction: None,
        input: None,
        output: None,
        note: None,
        owner: None,
        performer_type: None,
        location: None,
        insurance: None,
        relevant_history: None,
    };

    assert_eq!(transport.resource_type, "Transport");
    assert_eq!(transport.status, "in-progress");
    assert_eq!(transport.intent, "order");
}

#[test]
fn test_r5_genomic_study() {
    let genomic_study = GenomicStudy {
        resource_type: "GenomicStudy".to_string(),
        id: Some("genomic-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        status: "available".to_string(),
        r#type: None,
        subject: Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/r5-patient-1".to_string()),
            r#type: None,
            identifier: None,
            display: None,
        }),
        encounter: None,
        start_date: Some("2024-03-01".to_string()),
        based_on: None,
        referrer: None,
        interpreter: None,
        reason: None,
        instantiates_canonical: None,
        instantiates_uri: None,
        note: None,
        description: Some("Whole genome sequencing study".to_string()),
        analysis: None,
    };

    assert_eq!(genomic_study.resource_type, "GenomicStudy");
    assert_eq!(genomic_study.status, "available");
}

// =============================================================================
// Device and Specimen Tests (R5)
// =============================================================================

#[test]
fn test_r5_device() {
    let device = Device {
        resource_type: "Device".to_string(),
        id: Some("device-1".to_string()),
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
            r#use: None,
            r#type: None,
            system: Some("http://example.org/devices".to_string()),
            value: Some("DEVICE-12345".to_string()),
            period: None,
            assigner: None,
        })]),
        display_name: Some("Blood Pressure Monitor Model X".to_string()),
        definition: None,
        udi_carrier: None,
        status: Some("active".to_string()),
        availability_status: None,
        biological_source_event: None,
        manufacturer: Some("MedTech Inc".to_string()),
        manufacture_date: None,
        expiration_date: None,
        lot_number: None,
        serial_number: Some("SN-9876543210".to_string()),
        name: None,
        model_number: Some("BP-X2000".to_string()),
        part_number: None,
        category: None,
        r#type: None,
        version: None,
        conforms_to: None,
        property: None,
        mode: None,
        cycle: None,
        duration: None,
        owner: None,
        contact: None,
        location: None,
        url: None,
        endpoint: None,
        gateway: None,
        note: None,
        safety: None,
        parent: None,
    };

    assert_eq!(device.resource_type, "Device");
    assert_eq!(device.status, Some("active".to_string()));
    assert!(device.manufacturer.is_some());
}

#[test]
fn test_r5_specimen() {
    let specimen = Specimen {
        resource_type: "Specimen".to_string(),
        id: Some("specimen-1".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        accession_identifier: None,
        status: Some("available".to_string()),
        r#type: Some(CodeableConcept {
            id: None,
            extension: None,
            coding: Some(vec![Coding {
                id: None,
                extension: None,
                system: Some("http://snomed.info/sct".to_string()),
                version: None,
                code: Some("119297000".to_string()),
                display: Some("Blood specimen".to_string()),
                user_selected: None,
            }]),
            text: Some("Blood specimen".to_string()),
        }),
        subject: Some(Box::new(Reference {
            id: None,
            extension: None,
            reference: Some("Patient/r5-patient-1".to_string()),
            r#type: None,
            identifier: None,
            display: None,
        })),
        received_time: None,
        parent: None,
        request: None,
        combined: None,
        role: None,
        feature: None,
        collection: None,
        processing: None,
        container: None,
        condition: None,
        note: None,
    };

    assert_eq!(specimen.resource_type, "Specimen");
    assert_eq!(specimen.status, Some("available".to_string()));
}

// =============================================================================
// Serialization/Deserialization Tests (R5)
// =============================================================================

#[test]
fn test_r5_patient_serialization() {
    let patient = Patient {
        resource_type: "Patient".to_string(),
        id: Some("serialize-r5".to_string()),
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
            family: Some("Serialization".to_string()),
            given: Some(vec!["Test".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("male".to_string()),
        birth_date: Some("2000-01-01".to_string()),
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

    let json = serde_json::to_string_pretty(&patient).expect("Failed to serialize");
    assert!(json.contains("\"resourceType\": \"Patient\""));
    assert!(json.contains("\"id\": \"serialize-r5\""));
    assert!(json.contains("\"active\": true"));
}

#[test]
fn test_r5_bundle_deserialization() {
    let json_data = r#"{
        "resourceType": "Bundle",
        "type": "collection",
        "entry": [
            {
                "fullUrl": "https://example.org/Patient/1",
                "resource": {
                    "resourceType": "Patient",
                    "id": "1"
                }
            }
        ]
    }"#;

    let bundle: Bundle = serde_json::from_str(json_data).expect("Failed to deserialize");

    assert_eq!(bundle.resource_type, "Bundle");
    assert_eq!(bundle.r#type, "collection");
    assert!(bundle.entry.is_some());
}

// =============================================================================
// Round-trip Tests (R5)
// =============================================================================

#[test]
fn test_r5_patient_roundtrip() {
    let original = Patient {
        resource_type: "Patient".to_string(),
        id: Some("roundtrip-r5".to_string()),
        meta: None,
        implicit_rules: None,
        language: Some("fr-FR".to_string()),
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        active: Some(false),
        name: Some(vec![HumanName {
            id: None,
            extension: None,
            r#use: Some("maiden".to_string()),
            text: None,
            family: Some("RoundTrip".to_string()),
            given: Some(vec!["R5".to_string(), "Test".to_string()]),
            prefix: None,
            suffix: None,
            period: None,
        }]),
        telecom: None,
        gender: Some("female".to_string()),
        birth_date: Some("1995-05-05".to_string()),
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

    let json = serde_json::to_string(&original).expect("Failed to serialize");
    let deserialized: Patient = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(original.resource_type, deserialized.resource_type);
    assert_eq!(original.id, deserialized.id);
    assert_eq!(original.language, deserialized.language);
    assert_eq!(original.active, deserialized.active);
    assert_eq!(original.gender, deserialized.gender);
    assert_eq!(original.birth_date, deserialized.birth_date);
}

#[test]
fn test_r5_observation_roundtrip() {
    let original = Observation {
        resource_type: "Observation".to_string(),
        id: Some("obs-roundtrip".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        text: None,
        contained: None,
        extension: None,
        modifier_extension: None,
        identifier: None,
        instantiates: None,
        based_on: None,
        triggered_by: None,
        part_of: None,
        status: "preliminary".to_string(),
        category: None,
        code: CodeableConcept {
            id: None,
            extension: None,
            coding: None,
            text: Some("Test Code".to_string()),
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

    let json = serde_json::to_string(&original).expect("Failed to serialize");
    let deserialized: Observation = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(original.resource_type, deserialized.resource_type);
    assert_eq!(original.status, deserialized.status);
}
