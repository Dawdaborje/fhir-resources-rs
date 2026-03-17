# FHIR R4 Code Generation Examples

## Overview

This document provides real-world examples of how FHIR schema definitions are transformed into Rust code.

## Example 1: Simple Resource - Account

### Input FHIR Schema

```json
{
  "resourceType": "StructureDefinition",
  "id": "Account",
  "name": "Account",
  "kind": "resource",
  "type": "Account",
  "snapshot": {
    "element": [
      {
        "id": "Account",
        "path": "Account",
        "short": "Tracks balance, charges, for patient or cost center",
        "definition": "A financial tool for tracking value accrued for a particular purpose.",
        "min": 0,
        "max": "*"
      },
      {
        "id": "Account.id",
        "path": "Account.id",
        "short": "Logical id of this artifact",
        "min": 0,
        "max": "1",
        "type": [{"code": "id"}]
      },
      {
        "id": "Account.identifier",
        "path": "Account.identifier",
        "short": "Account number",
        "min": 0,
        "max": "*",
        "type": [{"code": "Identifier"}]
      },
      {
        "id": "Account.status",
        "path": "Account.status",
        "short": "active | inactive | entered-in-error | on-hold | unknown",
        "min": 1,
        "max": "1",
        "type": [{"code": "code"}],
        "binding": {
          "strength": "required",
          "valueSet": "http://hl7.org/fhir/ValueSet/account-status",
          "description": "active | inactive | entered-in-error | on-hold | unknown"
        }
      },
      {
        "id": "Account.name",
        "path": "Account.name",
        "short": "Human-readable label",
        "min": 0,
        "max": "1",
        "type": [{"code": "string"}]
      }
    ]
  }
}
```

### Generated Rust Code

**File: `crates/resources/src/r4/account.rs`**

```rust
//! FHIR R4 Account Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};
use crate::r4::types::*;

/// Tracks balance, charges, for patient or cost center
///
/// A financial tool for tracking value accrued for a particular purpose.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// Account number
    pub identifier: Option<Vec<Identifier>>,
    
    /// active | inactive | entered-in-error | on-hold | unknown
    pub status: AccountStatusCode,
    
    /// Human-readable label
    pub name: Option<String>,
}

/// Account status codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountStatusCode {
    Active,
    Inactive,
    EnteredInError,
    OnHold,
    Unknown,
}
```

## Example 2: Choice Type - Patient.deceased[x]

### Input FHIR Schema

```json
{
  "id": "Patient.deceased[x]",
  "path": "Patient.deceased",
  "short": "Indicates if the individual is deceased or not",
  "min": 0,
  "max": "1",
  "type": [
    {"code": "boolean"},
    {"code": "dateTime"}
  ]
}
```

### Generated Rust Code

```rust
/// Indicates if the individual is deceased or not
#[serde(flatten)]
pub deceased: Option<DeceasedChoice>,

/// Choice type for Patient.deceased[x]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```

### Usage Example

```rust
// Deceased = true
let patient = Patient {
    deceased: Some(DeceasedChoice::Boolean(true)),
    ..Default::default()
};

// Deceased at specific date
let patient = Patient {
    deceased: Some(DeceasedChoice::DateTime("2020-01-15T10:30:00Z".to_string())),
    ..Default::default()
};

// Not deceased (or unknown)
let patient = Patient {
    deceased: None,
    ..Default::default()
};
```

## Example 3: BackboneElement - Patient.contact

### Input FHIR Schema

```json
{
  "id": "Patient.contact",
  "path": "Patient.contact",
  "short": "A contact party (e.g. guardian, partner, friend) for the patient",
  "min": 0,
  "max": "*",
  "type": [{"code": "BackboneElement"}]
},
{
  "id": "Patient.contact.relationship",
  "path": "Patient.contact.relationship",
  "short": "The kind of relationship",
  "min": 0,
  "max": "*",
  "type": [{"code": "CodeableConcept"}],
  "binding": {
    "valueSet": "http://hl7.org/fhir/ValueSet/patient-contactrelationship"
  }
},
{
  "id": "Patient.contact.name",
  "path": "Patient.contact.name",
  "short": "A name associated with the contact person",
  "min": 0,
  "max": "1",
  "type": [{"code": "HumanName"}]
},
{
  "id": "Patient.contact.telecom",
  "path": "Patient.contact.telecom",
  "short": "A contact detail for the person",
  "min": 0,
  "max": "*",
  "type": [{"code": "ContactPoint"}]
}
```

### Generated Rust Code

**File: `crates/resources/src/r4/patient.rs`**

```rust
pub mod contact;

pub struct Patient {
    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<contact::PatientContact>>,
}
```

**File: `crates/resources/src/r4/patient/mod.rs`**

```rust
pub mod contact;
```

**File: `crates/resources/src/r4/patient/contact.rs`**

```rust
//! FHIR R4 Patient Contact (BackboneElement)
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};
use crate::r4::types::*;

/// A contact party (e.g. guardian, partner, friend) for the patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// The kind of relationship
    pub relationship: Option<Vec<CodeableConcept>>,
    
    /// A name associated with the contact person
    pub name: Option<HumanName>,
    
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
}
```

### Usage Example

```rust
use crate::r4::{Patient, patient::contact::PatientContact};

let patient = Patient {
    contact: Some(vec![
        PatientContact {
            relationship: Some(vec![
                CodeableConcept {
                    text: Some("Emergency Contact".to_string()),
                    ..Default::default()
                }
            ]),
            name: Some(HumanName {
                family: Some("Doe".to_string()),
                given: Some(vec!["Jane".to_string()]),
                ..Default::default()
            }),
            telecom: Some(vec![
                ContactPoint {
                    system: Some(ContactPointSystemCode::Phone),
                    value: Some("555-1234".to_string()),
                    ..Default::default()
                }
            ]),
        }
    ]),
    ..Default::default()
};
```

## Example 4: Complex Type - HumanName

### Input FHIR Schema

```json
{
  "resourceType": "StructureDefinition",
  "id": "HumanName",
  "type": "HumanName",
  "snapshot": {
    "element": [
      {
        "id": "HumanName.use",
        "path": "HumanName.use",
        "short": "usual | official | temp | nickname | anonymous | old | maiden",
        "min": 0,
        "max": "1",
        "type": [{"code": "code"}],
        "binding": {
          "strength": "required",
          "description": "usual | official | temp | nickname | anonymous | old | maiden"
        }
      },
      {
        "id": "HumanName.text",
        "path": "HumanName.text",
        "short": "Text representation of the full name",
        "min": 0,
        "max": "1",
        "type": [{"code": "string"}]
      },
      {
        "id": "HumanName.family",
        "path": "HumanName.family",
        "short": "Family name (often called 'Surname')",
        "min": 0,
        "max": "1",
        "type": [{"code": "string"}]
      },
      {
        "id": "HumanName.given",
        "path": "HumanName.given",
        "short": "Given names (not always 'first'). Includes middle names",
        "min": 0,
        "max": "*",
        "type": [{"code": "string"}]
      },
      {
        "id": "HumanName.prefix",
        "path": "HumanName.prefix",
        "short": "Parts that come before the name",
        "min": 0,
        "max": "*",
        "type": [{"code": "string"}]
      },
      {
        "id": "HumanName.suffix",
        "path": "HumanName.suffix",
        "short": "Parts that come after the name",
        "min": 0,
        "max": "*",
        "type": [{"code": "string"}]
      },
      {
        "id": "HumanName.period",
        "path": "HumanName.period",
        "short": "Time period when name was/is in use",
        "min": 0,
        "max": "1",
        "type": [{"code": "Period"}]
      }
    ]
  }
}
```

### Generated Rust Code

**File: `crates/resources/src/r4/types.rs`**

```rust
/// A human's name with the ability to identify parts and usage information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    /// usual | official | temp | nickname | anonymous | old | maiden
    #[serde(rename = "use")]
    pub r#use: Option<HumanNameUseCode>,
    
    /// Text representation of the full name
    pub text: Option<String>,
    
    /// Family name (often called 'Surname')
    pub family: Option<String>,
    
    /// Given names (not always 'first'). Includes middle names
    pub given: Option<Vec<String>>,
    
    /// Parts that come before the name
    pub prefix: Option<Vec<String>>,
    
    /// Parts that come after the name
    pub suffix: Option<Vec<String>>,
    
    /// Time period when name was/is in use
    pub period: Option<Period>,
}

/// The use of a human name
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HumanNameUseCode {
    Usual,
    Official,
    Temp,
    Nickname,
    Anonymous,
    Old,
    Maiden,
}
```

### Usage Example

```rust
let name = HumanName {
    r#use: Some(HumanNameUseCode::Official),
    text: Some("Dr. John Q. Public Jr.".to_string()),
    family: Some("Public".to_string()),
    given: Some(vec!["John".to_string(), "Quincy".to_string()]),
    prefix: Some(vec!["Dr.".to_string()]),
    suffix: Some(vec!["Jr.".to_string()]),
    period: Some(Period {
        start: Some("2000-01-01".to_string()),
        end: None,
    }),
};
```

## Example 5: Reference Type - Patient.managingOrganization

### Input FHIR Schema

```json
{
  "id": "Patient.managingOrganization",
  "path": "Patient.managingOrganization",
  "short": "Organization that is the custodian of the patient record",
  "min": 0,
  "max": "1",
  "type": [{
    "code": "Reference",
    "targetProfile": ["http://hl7.org/fhir/StructureDefinition/Organization"]
  }]
}
```

### Generated Rust Code

```rust
/// Organization that is the custodian of the patient record
pub managing_organization: Option<Reference<Organization>>,
```

### Reference Type Definition

**File: `crates/types/src/lib.rs`**

```rust
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A reference from one resource to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference<T> {
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<String>,
    
    /// Type the reference refers to (e.g. "Patient")
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    
    /// Logical reference, when literal reference is not known
    pub identifier: Option<Identifier>,
    
    /// Text alternative for the resource
    pub display: Option<String>,
    
    /// Phantom data to bind the generic type
    #[serde(skip)]
    _phantom: PhantomData<T>,
}

impl<T> Default for Reference<T> {
    fn default() -> Self {
        Self {
            reference: None,
            r#type: None,
            identifier: None,
            display: None,
            _phantom: PhantomData,
        }
    }
}
```

### Usage Example

```rust
let patient = Patient {
    managing_organization: Some(Reference {
        reference: Some("Organization/123".to_string()),
        display: Some("Acme Healthcare".to_string()),
        ..Default::default()
    }),
    ..Default::default()
};
```

## Example 6: Cardinality Variations

### Optional Single Value (min=0, max=1)

```json
{
  "id": "Patient.active",
  "path": "Patient.active",
  "min": 0,
  "max": "1",
  "type": [{"code": "boolean"}]
}
```

```rust
pub active: Option<bool>,
```

### Optional Array (min=0, max=*)

```json
{
  "id": "Patient.identifier",
  "path": "Patient.identifier",
  "min": 0,
  "max": "*",
  "type": [{"code": "Identifier"}]
}
```

```rust
pub identifier: Option<Vec<Identifier>>,
```

### Required Single Value (min=1, max=1)

```json
{
  "id": "Account.status",
  "path": "Account.status",
  "min": 1,
  "max": "1",
  "type": [{"code": "code"}]
}
```

```rust
pub status: AccountStatusCode,
```

### Required Array (min=1, max=*)

```json
{
  "id": "Bundle.entry",
  "path": "Bundle.entry",
  "min": 1,
  "max": "*",
  "type": [{"code": "BackboneElement"}]
}
```

```rust
pub entry: Vec<BundleEntry>,
```

## Example 7: Reserved Keyword Handling

### Input FHIR Schema

```json
{
  "id": "Address.type",
  "path": "Address.type",
  "short": "postal | physical | both",
  "min": 0,
  "max": "1",
  "type": [{"code": "code"}]
}
```

### Generated Rust Code

```rust
/// postal | physical | both
#[serde(rename = "type")]
pub r#type: Option<AddressTypeCode>,
```

## Example 8: Complete Resource - Observation (Simplified)

### Generated File Structure

```
crates/resources/src/r4/
├── observation.rs
└── observation/
    ├── mod.rs
    ├── component.rs
    └── reference_range.rs
```

### observation.rs

```rust
//! FHIR R4 Observation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};
use crate::r4::types::*;

pub mod component;
pub mod reference_range;

/// Measurements and simple assertions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// Business Identifier for observation
    pub identifier: Option<Vec<Identifier>>,
    
    /// registered | preliminary | final | amended +
    pub status: ObservationStatusCode,
    
    /// Classification of type of observation
    pub category: Option<Vec<CodeableConcept>>,
    
    /// Type of observation (code / type)
    pub code: CodeableConcept,
    
    /// Who and/or what the observation is about
    pub subject: Option<Reference<Patient>>,
    
    /// Actual result
    #[serde(flatten)]
    pub value: Option<ValueChoice>,
    
    /// Component results
    pub component: Option<Vec<component::ObservationComponent>>,
}

/// Observation status codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObservationStatusCode {
    Registered,
    Preliminary,
    Final,
    Amended,
    Corrected,
    Cancelled,
    #[serde(rename = "entered-in-error")]
    EnteredInError,
    Unknown,
}

/// Choice type for Observation.value[x]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueChoice {
    Quantity(Quantity),
    CodeableConcept(CodeableConcept),
    String(String),
    Boolean(bool),
    Integer(i32),
    Range(Range),
    Ratio(Ratio),
    DateTime(String),
    Period(Period),
}
```

## Summary

These examples demonstrate:

1. **Simple resources** with primitive fields
2. **Choice types** using enums with #[serde(untagged)]
3. **BackboneElements** as nested modules
4. **Complex types** with enums for coded values
5. **Reference types** with generic parameters
6. **Cardinality variations** using Option and Vec
7. **Reserved keywords** with r# prefix
8. **Complete resources** with all patterns combined

The code generator produces idiomatic, type-safe Rust code that preserves FHIR semantics while providing excellent IDE support and compile-time validation.
