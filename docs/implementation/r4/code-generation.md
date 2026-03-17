# Code Generation Patterns

## Overview

This document defines the code generation templates and patterns used to produce Rust source code from FHIR schemas.

## File Organization

### Module Structure

```
crates/resources/src/r4/
├── mod.rs                    # Root module with declarations
├── types.rs                  # Complex types (HumanName, Address, etc.)
├── account.rs                # Account resource
├── patient.rs                # Patient resource
├── patient/                  # Patient BackboneElements
│   ├── mod.rs               # Patient nested module declarations
│   ├── contact.rs           # Patient.contact BackboneElement
│   └── communication.rs     # Patient.communication BackboneElement
├── observation.rs            # Observation resource
└── ...                       # Other resources
```

### mod.rs Template

```rust
//! FHIR R4 Resources and Types
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

pub mod types;

pub mod account;
pub mod observation;
pub mod patient;
// ... alphabetically sorted resources
```

## Struct Generation

### Basic Struct Template

```rust
/// {documentation}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {StructName} {
    {fields}
}
```

### Example: Patient Resource

```rust
/// Demographics and other administrative information about an individual or animal receiving care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// Metadata about the resource
    pub meta: Option<Meta>,
    
    /// A set of rules under which this content was created
    pub implicit_rules: Option<String>,
    
    /// Language of the resource content
    pub language: Option<String>,
    
    /// An identifier for this patient
    pub identifier: Option<Vec<Identifier>>,
    
    /// Whether this patient record is in active use
    pub active: Option<bool>,
    
    /// A name associated with the patient
    pub name: Option<Vec<HumanName>>,
    
    /// A contact detail for the individual
    pub telecom: Option<Vec<ContactPoint>>,
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}
```

### Field Template

```rust
{doc_comment}
{serde_attributes}
pub {field_name}: {field_type},
```

### Field with Serde Rename

```rust
/// {documentation}
#[serde(rename = "{original_name}")]
pub {field_name}: {field_type},
```

### Field with Reserved Keyword

```rust
/// {documentation}
#[serde(rename = "type")]
pub r#type: Option<String>,
```

## Enum Generation

### Choice Type Enum Template

```rust
/// Choice type for {ResourceName}.{fieldName}[x]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum {FieldName}Choice {
    {variants}
}
```

### Example: Deceased Choice Type

```rust
/// Choice type for Patient.deceased[x]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```

### Code Enum Template (ValueSet)

```rust
/// {documentation}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum {Name}Code {
    {variants}
}
```

### Example: Address Use Code

```rust
/// The use of an address: home | work | temp | old | billing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressUseCode {
    Home,
    Work,
    Temp,
    Old,
    Billing,
}
```

### Enum Variant with Custom Serialization

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentifierUseCode {
    #[serde(rename = "usual")]
    Usual,
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "temp")]
    Temp,
    #[serde(rename = "secondary")]
    Secondary,
    #[serde(rename = "old")]
    Old,
}
```

## BackboneElement Generation

### Inline BackboneElement Pattern

**File: `patient.rs`**
```rust
pub mod contact;
pub mod communication;

pub struct Patient {
    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<contact::PatientContact>>,
    
    /// Language communication preferences
    pub communication: Option<Vec<communication::PatientCommunication>>,
}
```

**File: `patient/mod.rs`**
```rust
pub mod contact;
pub mod communication;
```

**File: `patient/contact.rs`**
```rust
use serde::{Deserialize, Serialize};
use crate::r4::types::*;

/// A contact party (e.g. guardian, partner, friend) for the patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// The nature of the relationship between the patient and the contact person
    pub relationship: Option<Vec<CodeableConcept>>,
    
    /// A name associated with the contact person
    pub name: Option<HumanName>,
    
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
    
    /// Address for the contact person
    pub address: Option<Address>,
    
    /// male | female | other | unknown
    pub gender: Option<AdministrativeGenderCode>,
    
    /// Organization that is associated with the contact
    pub organization: Option<Reference<Organization>>,
    
    /// The period during which this contact person or organization is valid
    pub period: Option<Period>,
}
```

### Nested Module Creation Algorithm

1. Detect BackboneElement in resource definition
2. Extract element name (e.g., `Patient.contact` → `contact`)
3. Create module directory: `patient/`
4. Create module file: `patient/contact.rs`
5. Generate struct: `PatientContact`
6. Update parent: `pub mod contact;`
7. Reference in parent struct: `pub contact: Option<Vec<contact::PatientContact>>`

## Import Statements

### types.rs Imports

```rust
use serde::{Deserialize, Serialize};
```

### Resource File Imports

```rust
use serde::{Deserialize, Serialize};
use crate::r4::types::*;
```

### BackboneElement File Imports

```rust
use serde::{Deserialize, Serialize};
use crate::r4::types::*;
use super::super::{Resource, Reference};  // If needed
```

## Documentation Comments

### Doc Comment Template

```rust
/// {short_description}
///
/// {long_description_if_available}
```

### Example

```rust
/// An identifier for this patient
///
/// An identifier that applies to this person as a patient.
pub identifier: Option<Vec<Identifier>>,
```

### Multi-line Doc Comments

```rust
/// A contact party (e.g. guardian, partner, friend) for the patient
///
/// Contact covers all kinds of contact parties: family members, business contacts,
/// guardians, caregivers. Not applicable to register pedigree and family ties
/// beyond use for administrative purposes.
pub contact: Option<Vec<contact::PatientContact>>,
```

## Serde Annotations

### Struct Annotations

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {Name} { ... }
```

### Choice Type Annotations

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum {Name}Choice { ... }
```

### Field Annotations

#### Standard Field
```rust
pub field_name: Type,
```

#### Renamed Field
```rust
#[serde(rename = "originalName")]
pub field_name: Type,
```

#### Flattened Field (Choice Types)
```rust
#[serde(flatten)]
pub choice_field: Option<ChoiceEnum>,
```

#### Skip Serialization (PhantomData)
```rust
#[serde(skip)]
_phantom: std::marker::PhantomData<T>,
```

## Common Patterns

### Pattern: Optional Single Value

**FHIR**: min=0, max=1
```rust
pub field: Option<Type>,
```

### Pattern: Optional Array

**FHIR**: min=0, max=*
```rust
pub field: Option<Vec<Type>>,
```

### Pattern: Required Value

**FHIR**: min=1, max=1
```rust
pub field: Type,
```

### Pattern: Required Array

**FHIR**: min=1, max=*
```rust
pub field: Vec<Type>,
```

### Pattern: Choice Type

**FHIR**: deceased[x] → deceasedBoolean | deceasedDateTime
```rust
#[serde(flatten)]
pub deceased: Option<DeceasedChoice>,

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```

### Pattern: Reference

**FHIR**: Reference to Organization
```rust
pub managing_organization: Option<Reference<Organization>>,
```

### Pattern: Code with Binding

**FHIR**: code bound to ValueSet
```rust
pub gender: Option<AdministrativeGenderCode>,

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdministrativeGenderCode {
    Male,
    Female,
    Other,
    Unknown,
}
```

### Pattern: Reserved Keyword

**FHIR**: field named "type"
```rust
#[serde(rename = "type")]
pub r#type: Option<String>,
```

## Code Formatting

### Indentation

- 4 spaces per level (Rust standard)
- No tabs

### Line Length

- Target: 80-100 characters
- Max: 120 characters (before wrapping)

### Field Alignment

Align types vertically when possible:
```rust
pub id:         Option<String>,
pub meta:       Option<Meta>,
pub identifier: Option<Vec<Identifier>>,
```

### Blank Lines

- 1 blank line between struct fields (with doc comments)
- 2 blank lines between structs/enums
- No blank line after opening brace
- No blank line before closing brace

### Example

```rust
/// Patient resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    /// Logical id
    pub id: Option<String>,
    
    /// An identifier for this patient
    pub identifier: Option<Vec<Identifier>>,
}

/// Address type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// home | work | temp
    pub r#use: Option<AddressUseCode>,
}
```

## File Header

### Standard Header Template

```rust
//! FHIR R4 {ResourceName}
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};
use crate::r4::types::*;
```

### Example: patient.rs

```rust
//! FHIR R4 Patient Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};
use crate::r4::types::*;

pub mod contact;
pub mod communication;

// Structs follow...
```

## rustfmt Configuration

Generated code is formatted using rustfmt with standard settings:

```toml
# .rustfmt.toml (if needed)
edition = "2021"
max_width = 100
use_small_heuristics = "Default"
```

Invoke after generation:
```bash
rustfmt crates/resources/src/r4/*.rs
rustfmt crates/resources/src/r4/**/*.rs
```

## Generation Order

1. **types.rs** - Generate complex types first (no dependencies)
2. **Resources** - Generate in alphabetical order
3. **BackboneElements** - Generate as nested modules within resources
4. **mod.rs** - Generate last (needs list of all modules)

## Complete Example: Address Type

```rust
//! FHIR R4 Types
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};

/// An address expressed using postal conventions (as opposed to GPS or other location definition formats)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Unique id for inter-element referencing
    pub id: Option<String>,
    
    /// home | work | temp | old | billing - purpose of this address
    #[serde(rename = "use")]
    pub r#use: Option<AddressUseCode>,
    
    /// postal | physical | both
    #[serde(rename = "type")]
    pub r#type: Option<AddressTypeCode>,
    
    /// Text representation of the address
    pub text: Option<String>,
    
    /// Street name, number, direction & P.O. Box etc.
    pub line: Option<Vec<String>>,
    
    /// Name of city, town etc.
    pub city: Option<String>,
    
    /// Sub-unit of country (abbreviations ok)
    pub district: Option<String>,
    
    /// Sub-unit of country (abbreviations ok)
    pub state: Option<String>,
    
    /// Postal code for area
    pub postal_code: Option<String>,
    
    /// Country (e.g. can be ISO 3166 2 or 3 letter code)
    pub country: Option<String>,
    
    /// Time period when address was/is in use
    pub period: Option<Period>,
}

/// The use of an address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressUseCode {
    Home,
    Work,
    Temp,
    Old,
    Billing,
}

/// The type of an address (physical / postal)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressTypeCode {
    Postal,
    Physical,
    Both,
}
```
