# FHIR R5 Examples

## Overview

This document provides practical examples of using the generated FHIR R5 Rust types. Examples include creating resources, serializing to JSON, deserializing from JSON, and working with complex types.

---

## Basic Resource Creation

### Creating a Simple Patient

```rust
use fhir_resources::r5::{Patient, HumanName};

fn create_simple_patient() -> Patient {
    Patient {
        id: Some("example".to_string()),
        meta: None,
        implicit_rules: None,
        language: Some("en-US".to_string()),
        text: None,
        identifier: None,
        active: Some(true),
        name: Some(vec![
            HumanName {
                id: None,
                r#use: Some("official".to_string()),
                text: Some("Peter James Chalmers".to_string()),
                family: Some("Chalmers".to_string()),
                given: Some(vec!["Peter".to_string(), "James".to_string()]),
                prefix: None,
                suffix: None,
                period: None,
            }
        ]),
        telecom: None,
        gender: Some("male".to_string()),
        birth_date: Some("1974-12-25".to_string()),
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
    }
}
```

### Creating a Patient with Address

```rust
use fhir_resources::r5::{Patient, HumanName, Address, Period};

fn create_patient_with_address() -> Patient {
    Patient {
        id: Some("example-address".to_string()),
        active: Some(true),
        name: Some(vec![
            HumanName {
                r#use: Some("official".to_string()),
                family: Some("Doe".to_string()),
                given: Some(vec!["Jane".to_string()]),
                ..Default::default()
            }
        ]),
        address: Some(vec![
            Address {
                id: None,
                r#use: Some("home".to_string()),
                r#type: Some("physical".to_string()),
                text: Some("534 Erewhon St, Pleasantville, Vic, 3999".to_string()),
                line: Some(vec!["534 Erewhon St".to_string()]),
                city: Some("Pleasantville".to_string()),
                district: None,
                state: Some("Vic".to_string()),
                postal_code: Some("3999".to_string()),
                country: Some("Australia".to_string()),
                period: Some(Period {
                    id: None,
                    start: Some("2020-01-01".to_string()),
                    end: None,
                }),
            }
        ]),
        ..Default::default()
    }
}
```

**Note**: Requires implementing `Default` trait for resources, or manually specifying all fields.

---

## Working with Choice Types

### Patient with Deceased Boolean

```rust
use fhir_resources::r5::{Patient, DeceasedChoice};

fn create_deceased_patient() -> Patient {
    Patient {
        id: Some("deceased-example".to_string()),
        active: Some(false),
        deceased: Some(DeceasedChoice::Boolean(true)),
        ..Default::default()
    }
}
```

### Patient with Deceased DateTime

```rust
use fhir_resources::r5::{Patient, DeceasedChoice};

fn create_deceased_patient_with_date() -> Patient {
    Patient {
        id: Some("deceased-date-example".to_string()),
        active: Some(false),
        deceased: Some(DeceasedChoice::DateTime("2020-05-15T10:30:00Z".to_string())),
        ..Default::default()
    }
}
```

### Observation with Value Quantity

```rust
use fhir_resources::r5::{Observation, ObservationValueChoice, Quantity};

fn create_observation_with_value() -> Observation {
    Observation {
        id: Some("body-weight".to_string()),
        status: "final".to_string(),
        code: /* CodeableConcept for body weight */,
        subject: Some(/* Reference to Patient */),
        value: Some(ObservationValueChoice::Quantity(Quantity {
            id: None,
            value: Some(85.5),
            comparator: None,
            unit: Some("kg".to_string()),
            system: Some("http://unitsofmeasure.org".to_string()),
            code: Some("kg".to_string()),
        })),
        ..Default::default()
    }
}
```

---

## Serialization to JSON

### Serialize Patient to JSON

```rust
use fhir_resources::r5::Patient;
use serde_json;

fn serialize_patient_to_json(patient: &Patient) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(patient)
}

fn main() {
    let patient = create_simple_patient();
    
    match serialize_patient_to_json(&patient) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Serialization error: {}", e),
    }
}
```

**Output**:
```json
{
  "id": "example",
  "language": "en-US",
  "active": true,
  "name": [
    {
      "use": "official",
      "text": "Peter James Chalmers",
      "family": "Chalmers",
      "given": ["Peter", "James"]
    }
  ],
  "gender": "male",
  "birthDate": "1974-12-25"
}
```

**Note**: Fields with `None` values are omitted from JSON (serde default behavior).

---

## Deserialization from JSON

### Deserialize Patient from JSON String

```rust
use fhir_resources::r5::Patient;
use serde_json;

fn deserialize_patient_from_json(json: &str) -> Result<Patient, serde_json::Error> {
    serde_json::from_str(json)
}

fn main() {
    let json = r#"
    {
      "resourceType": "Patient",
      "id": "example",
      "active": true,
      "name": [
        {
          "use": "official",
          "family": "Chalmers",
          "given": ["Peter", "James"]
        }
      ],
      "gender": "male",
      "birthDate": "1974-12-25"
    }
    "#;
    
    match deserialize_patient_from_json(json) {
        Ok(patient) => {
            println!("Patient ID: {:?}", patient.id);
            println!("Active: {:?}", patient.active);
            println!("Gender: {:?}", patient.gender);
        }
        Err(e) => eprintln!("Deserialization error: {}", e),
    }
}
```

### Deserialize from File

```rust
use fhir_resources::r5::Patient;
use std::fs;

fn load_patient_from_file(path: &str) -> Result<Patient, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let patient = serde_json::from_str(&json)?;
    Ok(patient)
}

fn main() {
    match load_patient_from_file("patient-example.json") {
        Ok(patient) => println!("Loaded patient: {:?}", patient.id),
        Err(e) => eprintln!("Error loading patient: {}", e),
    }
}
```

---

## Working with Complex Types

### Creating Identifiers

```rust
use fhir_resources::r5::{Identifier, CodeableConcept, Coding, Period};

fn create_medical_record_number() -> Identifier {
    Identifier {
        id: None,
        r#use: Some("official".to_string()),
        r#type: Some(CodeableConcept {
            id: None,
            coding: Some(vec![
                Coding {
                    id: None,
                    system: Some("http://terminology.hl7.org/CodeSystem/v2-0203".to_string()),
                    version: None,
                    code: Some("MR".to_string()),
                    display: Some("Medical record number".to_string()),
                    user_selected: None,
                }
            ]),
            text: Some("Medical Record Number".to_string()),
        }),
        system: Some("http://hospital.example.org".to_string()),
        value: Some("12345".to_string()),
        period: Some(Period {
            id: None,
            start: Some("2020-01-01".to_string()),
            end: None,
        }),
        assigner: None,
    }
}

fn create_patient_with_identifier() -> Patient {
    Patient {
        id: Some("example".to_string()),
        identifier: Some(vec![create_medical_record_number()]),
        ..Default::default()
    }
}
```

### Creating Contact Points

```rust
use fhir_resources::r5::{ContactPoint, Period};

fn create_contact_points() -> Vec<ContactPoint> {
    vec![
        ContactPoint {
            id: None,
            system: Some("phone".to_string()),
            value: Some("+1-555-123-4567".to_string()),
            r#use: Some("home".to_string()),
            rank: Some(1),
            period: Some(Period {
                id: None,
                start: Some("2020-01-01".to_string()),
                end: None,
            }),
        },
        ContactPoint {
            id: None,
            system: Some("email".to_string()),
            value: Some("patient@example.com".to_string()),
            r#use: Some("work".to_string()),
            rank: Some(2),
            period: None,
        },
    ]
}
```

---

## Working with References

### Creating References to Other Resources

```rust
use fhir_resources::r5::{Observation, Reference, CodeableConcept};

fn create_observation_with_subject() -> Observation {
    Observation {
        id: Some("obs-example".to_string()),
        status: "final".to_string(),
        code: /* CodeableConcept */,
        subject: Some(Reference {
            id: None,
            reference: Some("Patient/example".to_string()),
            r#type: Some("Patient".to_string()),
            identifier: None,
            display: Some("Peter James Chalmers".to_string()),
        }),
        performer: Some(vec![
            Reference {
                id: None,
                reference: Some("Practitioner/f201".to_string()),
                r#type: Some("Practitioner".to_string()),
                identifier: None,
                display: Some("Dr. Adam Careful".to_string()),
            }
        ]),
        ..Default::default()
    }
}
```

### Using Identifier References

```rust
use fhir_resources::r5::{Reference, Identifier};

fn create_logical_reference() -> Reference {
    Reference {
        id: None,
        reference: None,  // No literal reference
        r#type: Some("Patient".to_string()),
        identifier: Some(Identifier {
            id: None,
            r#use: None,
            r#type: None,
            system: Some("http://hospital.example.org".to_string()),
            value: Some("12345".to_string()),
            period: None,
            assigner: None,
        }),
        display: Some("Patient with MRN 12345".to_string()),
    }
}
```

---

## Working with BackboneElements

### Patient with Contact Person

```rust
use fhir_resources::r5::{Patient, PatientContact, HumanName, ContactPoint, Address, Reference, CodeableConcept};

fn create_patient_with_contact() -> Patient {
    Patient {
        id: Some("example".to_string()),
        name: Some(vec![
            HumanName {
                family: Some("Doe".to_string()),
                given: Some(vec!["John".to_string()]),
                ..Default::default()
            }
        ]),
        contact: Some(vec![
            PatientContact {
                id: None,
                extension: None,
                modifier_extension: None,
                relationship: Some(vec![
                    CodeableConcept {
                        id: None,
                        coding: None,
                        text: Some("Emergency Contact".to_string()),
                    }
                ]),
                name: Some(HumanName {
                    family: Some("Doe".to_string()),
                    given: Some(vec!["Jane".to_string()]),
                    ..Default::default()
                }),
                telecom: Some(vec![
                    ContactPoint {
                        system: Some("phone".to_string()),
                        value: Some("+1-555-987-6543".to_string()),
                        r#use: Some("mobile".to_string()),
                        ..Default::default()
                    }
                ]),
                address: None,
                gender: Some("female".to_string()),
                organization: None,
                period: None,
            }
        ]),
        ..Default::default()
    }
}
```

---

## Working with Bundles

### Creating a Bundle with Multiple Resources

```rust
use fhir_resources::r5::{Bundle, BundleEntry, Patient, Observation};
use serde_json::Value;

fn create_search_results_bundle(patients: Vec<Patient>) -> Bundle {
    Bundle {
        id: Some("bundle-example".to_string()),
        meta: None,
        implicit_rules: None,
        language: None,
        identifier: None,
        r#type: "searchset".to_string(),
        timestamp: Some("2023-11-15T10:30:00Z".to_string()),
        total: Some(patients.len() as u32),
        link: None,
        entry: Some(
            patients.into_iter().map(|patient| {
                BundleEntry {
                    id: None,
                    link: None,
                    full_url: Some(format!("http://example.org/fhir/Patient/{}", 
                                          patient.id.clone().unwrap_or_default())),
                    resource: Some(serde_json::to_value(patient).unwrap()),
                    search: None,
                    request: None,
                    response: None,
                }
            }).collect()
        ),
        signature: None,
    }
}
```

**Note**: Bundle.entry.resource is polymorphic. Current implementation may use `serde_json::Value`. Future enhancement could use a Resource enum.

### Creating a Transaction Bundle

```rust
use fhir_resources::r5::{Bundle, BundleEntry, BundleEntryRequest};

fn create_transaction_bundle() -> Bundle {
    Bundle {
        id: Some("transaction-example".to_string()),
        r#type: "transaction".to_string(),
        entry: Some(vec![
            BundleEntry {
                id: None,
                full_url: Some("urn:uuid:61ebe359-bfdc-4613-8bf2-c5e300945f0a".to_string()),
                resource: Some(/* Patient resource as Value */),
                request: Some(BundleEntryRequest {
                    id: None,
                    method: "POST".to_string(),
                    url: "Patient".to_string(),
                    if_none_match: None,
                    if_modified_since: None,
                    if_match: None,
                    if_none_exist: None,
                }),
                ..Default::default()
            }
        ]),
        ..Default::default()
    }
}
```

---

## Error Handling

### Handling Deserialization Errors

```rust
use fhir_resources::r5::Patient;
use serde_json;

fn safe_deserialize_patient(json: &str) -> Result<Patient, String> {
    serde_json::from_str(json)
        .map_err(|e| format!("Failed to deserialize Patient: {}", e))
}

fn main() {
    let valid_json = r#"{"resourceType": "Patient", "id": "123", "active": true}"#;
    let invalid_json = r#"{"resourceType": "Patient", "active": "not a boolean"}"#;
    
    match safe_deserialize_patient(valid_json) {
        Ok(patient) => println!("Valid patient: {:?}", patient.id),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    match safe_deserialize_patient(invalid_json) {
        Ok(patient) => println!("Valid patient: {:?}", patient.id),
        Err(e) => eprintln!("Error: {}", e),  // Will print error about boolean type
    }
}
```

### Validating Required Fields

```rust
use fhir_resources::r5::Observation;

fn validate_observation(obs: &Observation) -> Result<(), String> {
    // Check required fields (min=1)
    if obs.status.is_empty() {
        return Err("Observation.status is required".to_string());
    }
    
    // Note: code is also required but the type system doesn't enforce it
    // if there are Option fields that should be required
    
    Ok(())
}
```

---

## Advanced Examples

### Working with Extensions

```rust
use fhir_resources::r5::{Patient, Extension};

fn create_patient_with_extension() -> Patient {
    Patient {
        id: Some("example".to_string()),
        extension: Some(vec![
            Extension {
                id: None,
                url: "http://example.org/fhir/StructureDefinition/patient-importance".to_string(),
                value: Some(/* ExtensionValueChoice::Code */),
                extension: None,  // No nested extensions
            }
        ]),
        ..Default::default()
    }
}
```

### Cloning and Modifying Resources

```rust
use fhir_resources::r5::Patient;

fn update_patient_status(mut patient: Patient) -> Patient {
    patient.active = Some(false);
    patient
}

fn main() {
    let original = create_simple_patient();
    let updated = update_patient_status(original.clone());
    
    println!("Original active: {:?}", original.active);
    println!("Updated active: {:?}", updated.active);
}
```

---

## Testing Examples

### Unit Test for Resource Creation

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use fhir_resources::r5::Patient;
    
    #[test]
    fn test_create_patient() {
        let patient = create_simple_patient();
        
        assert_eq!(patient.id, Some("example".to_string()));
        assert_eq!(patient.active, Some(true));
        assert_eq!(patient.gender, Some("male".to_string()));
        assert!(patient.name.is_some());
        
        let names = patient.name.unwrap();
        assert_eq!(names.len(), 1);
        assert_eq!(names[0].family, Some("Chalmers".to_string()));
    }
    
    #[test]
    fn test_patient_serialization_round_trip() {
        let original = create_simple_patient();
        
        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();
        
        // Deserialize back
        let deserialized: Patient = serde_json::from_str(&json).unwrap();
        
        // Compare fields
        assert_eq!(original.id, deserialized.id);
        assert_eq!(original.active, deserialized.active);
        assert_eq!(original.gender, deserialized.gender);
    }
}
```

### Integration Test with Real FHIR Data

```rust
#[test]
fn test_deserialize_real_fhir_patient() {
    let json = include_str!("../test-data/r5/patient-example.json");
    
    let result: Result<Patient, _> = serde_json::from_str(json);
    
    assert!(result.is_ok(), "Failed to deserialize: {:?}", result.err());
    
    let patient = result.unwrap();
    assert!(patient.id.is_some());
}
```

---

## Pattern Matching on Choice Types

### Handling Different Value Types

```rust
use fhir_resources::r5::{Observation, ObservationValueChoice};

fn display_observation_value(obs: &Observation) {
    match &obs.value {
        Some(ObservationValueChoice::Quantity(q)) => {
            println!("Quantity: {:?} {:?}", q.value, q.unit);
        }
        Some(ObservationValueChoice::String(s)) => {
            println!("String value: {}", s);
        }
        Some(ObservationValueChoice::CodeableConcept(cc)) => {
            println!("Coded value: {:?}", cc.text);
        }
        Some(ObservationValueChoice::Boolean(b)) => {
            println!("Boolean value: {}", b);
        }
        None => {
            println!("No value present");
        }
        _ => {
            println!("Other value type");
        }
    }
}
```

### Creating Different Choice Type Variants

```rust
use fhir_resources::r5::{Patient, DeceasedChoice};

fn create_living_patient() -> Patient {
    Patient {
        deceased: Some(DeceasedChoice::Boolean(false)),
        ..Default::default()
    }
}

fn create_deceased_patient(date: &str) -> Patient {
    Patient {
        deceased: Some(DeceasedChoice::DateTime(date.to_string())),
        ..Default::default()
    }
}
```

---

## Best Practices

### 1. Use Type Aliases for Clarity

```rust
type PatientId = String;
type FhirDateTime = String;

fn get_patient_by_id(id: PatientId) -> Option<Patient> {
    // Implementation
}
```

### 2. Builder Pattern (Manual Implementation)

```rust
pub struct PatientBuilder {
    patient: Patient,
}

impl PatientBuilder {
    pub fn new() -> Self {
        Self {
            patient: Patient::default(),
        }
    }
    
    pub fn id(mut self, id: &str) -> Self {
        self.patient.id = Some(id.to_string());
        self
    }
    
    pub fn active(mut self, active: bool) -> Self {
        self.patient.active = Some(active);
        self
    }
    
    pub fn gender(mut self, gender: &str) -> Self {
        self.patient.gender = Some(gender.to_string());
        self
    }
    
    pub fn build(self) -> Patient {
        self.patient
    }
}

// Usage:
let patient = PatientBuilder::new()
    .id("example")
    .active(true)
    .gender("male")
    .build();
```

### 3. Validation Helper Functions

```rust
fn is_valid_fhir_id(id: &str) -> bool {
    id.len() <= 64 && id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '.')
}

fn validate_patient(patient: &Patient) -> Result<(), String> {
    if let Some(id) = &patient.id {
        if !is_valid_fhir_id(id) {
            return Err(format!("Invalid ID format: {}", id));
        }
    }
    Ok(())
}
```

### 4. Resource Collections

```rust
use std::collections::HashMap;

struct PatientRegistry {
    patients: HashMap<String, Patient>,
}

impl PatientRegistry {
    pub fn new() -> Self {
        Self {
            patients: HashMap::new(),
        }
    }
    
    pub fn add(&mut self, patient: Patient) -> Result<(), String> {
        let id = patient.id.clone()
            .ok_or("Patient must have an ID")?;
        
        self.patients.insert(id, patient);
        Ok(())
    }
    
    pub fn get(&self, id: &str) -> Option<&Patient> {
        self.patients.get(id)
    }
}
```

---

## Complete Example: FHIR Client

```rust
use fhir_resources::r5::{Patient, Bundle};
use reqwest;
use serde_json;

pub struct FhirClient {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl FhirClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }
    
    pub fn get_patient(&self, id: &str) -> Result<Patient, Box<dyn std::error::Error>> {
        let url = format!("{}/Patient/{}", self.base_url, id);
        let response = self.client.get(&url).send()?;
        let patient = response.json::<Patient>()?;
        Ok(patient)
    }
    
    pub fn search_patients(&self, params: &[(&str, &str)]) -> Result<Bundle, Box<dyn std::error::Error>> {
        let url = format!("{}/Patient", self.base_url);
        let response = self.client.get(&url).query(params).send()?;
        let bundle = response.json::<Bundle>()?;
        Ok(bundle)
    }
    
    pub fn create_patient(&self, patient: &Patient) -> Result<Patient, Box<dyn std::error::Error>> {
        let url = format!("{}/Patient", self.base_url);
        let response = self.client.post(&url).json(patient).send()?;
        let created = response.json::<Patient>()?;
        Ok(created)
    }
}

// Usage:
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FhirClient::new("https://hapi.fhir.org/baseR5");
    
    // Search for patients
    let results = client.search_patients(&[("family", "Smith")])?;
    println!("Found {} patients", results.total.unwrap_or(0));
    
    // Get specific patient
    let patient = client.get_patient("example")?;
    println!("Patient: {:?}", patient.name);
    
    Ok(())
}
```

---

## Summary

These examples demonstrate:
- Creating FHIR R5 resources
- Working with complex types (HumanName, Address, etc.)
- Handling choice types (deceased[x], value[x])
- Serialization and deserialization
- References between resources
- BackboneElements (Patient.contact)
- Bundles for collections
- Error handling
- Testing strategies
- Best practices and patterns

For more examples, see the FHIR R5 specification examples at https://hl7.org/fhir/R5/
