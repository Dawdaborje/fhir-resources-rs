# FHIR R5 Code Generation Patterns

## Overview

This document defines the code generation templates and patterns used to produce Rust source code from FHIR R5 schemas. R5 generation reuses the common code generation modules developed for R4B, with version-specific adjustments as needed.

## File Organization

### Module Structure

```
crates/resources/src/r5/
├── mod.rs                    # Root module with declarations
├── types.rs                  # Complex types (HumanName, Address, CodeableConcept, etc.)
├── account.rs                # Account resource
├── patient.rs                # Patient resource
├── patient/                  # Patient BackboneElements
│   ├── mod.rs               # Patient nested module declarations
│   ├── contact.rs           # Patient.contact BackboneElement
│   └── communication.rs     # Patient.communication BackboneElement
├── observation.rs            # Observation resource
├── bundle.rs                 # Bundle resource
└── ...                       # Other R5 resources
```

### mod.rs Template

```rust
//! FHIR R5 Resources and Types
//!
//! Auto-generated from FHIR R5 (5.0.0) schema definitions.
//! Do not modify directly.
//!
//! Generated: [timestamp]

pub mod types;

pub mod account;
pub mod activity_definition;
pub mod adverse_event;
pub mod allergy_intolerance;
// ... alphabetically sorted resource modules
pub mod patient;
pub mod observation;
// ... more resources
```

**Generation Logic**:
1. Collect all resource names from StructureDefinitions
2. Convert to snake_case (Patient → patient)
3. Sort alphabetically
4. Generate `pub mod {name};` for each

---

## Struct Generation

### Basic Struct Template

All generated structs follow this pattern:

```rust
/// {short description from FHIR schema}
///
/// {definition from FHIR schema}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {StructName} {
    {fields}
}
```

### Resource Example: Patient

```rust
/// Demographics and other administrative information about an individual
/// receiving care or other health-related services.
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
    
    /// Text summary of the resource, for human interpretation
    pub text: Option<Narrative>,
    
    /// An identifier for this patient
    pub identifier: Option<Vec<Identifier>>,
    
    /// Whether this patient record is in active use
    pub active: Option<bool>,
    
    /// A name associated with the individual
    pub name: Option<Vec<HumanName>>,
    
    /// A contact detail for the individual
    pub telecom: Option<Vec<ContactPoint>>,
    
    /// male | female | other | unknown
    pub gender: Option<String>,
    
    /// The date of birth for the individual
    pub birth_date: Option<String>,
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
    
    /// An address for the individual
    pub address: Option<Vec<Address>>,
    
    /// A contact party (e.g. guardian, partner, friend) for the patient
    pub contact: Option<Vec<PatientContact>>,
}
```

### Complex Type Example: HumanName

```rust
/// A name, normally of a human, that can be used for other living entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,
    
    /// usual | official | temp | nickname | anonymous | old | maiden
    pub r#use: Option<String>,
    
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
```

---

## Field Generation

### Field Template

Each field is generated from an Element in the StructureDefinition:

```rust
/// {element.short}
{serde_attributes}
pub {field_name}: {field_type},
```

### Field Name Conversion

FHIR paths use camelCase or specific naming; convert to Rust snake_case:

| FHIR Path | Rust Field Name | Notes |
|-----------|-----------------|-------|
| `Patient.birthDate` | `birth_date` | camelCase → snake_case |
| `HumanName.use` | `r#use` | Rust keyword escaped |
| `Patient.identifier` | `identifier` | Already snake_case |
| `Meta.versionId` | `version_id` | camelCase → snake_case |

**Conversion Logic**:
1. Extract last segment of path after `.`
2. Convert camelCase to snake_case
3. If Rust keyword (use, type, etc.), prefix with `r#`

### Field Type Derivation

Derived from `element.type[].code` and cardinality (`min`, `max`):

```rust
fn derive_field_type(element: &Element) -> String {
    let base_type = map_fhir_type(&element.type[0].code);
    apply_cardinality(base_type, element.min, &element.max)
}
```

See [type-mapping.md](./type-mapping.md) for detailed mapping rules.

### Serde Attributes

#### Default: camelCase Rename

All structs use:
```rust
#[serde(rename_all = "camelCase")]
```

This handles:
- `birthDate` ↔ `birth_date`
- `versionId` ↔ `version_id`

#### Keyword Escape

For Rust keywords, no serde rename needed due to `rename_all`:
```rust
/// usage | official | temp | nickname | anonymous | old | maiden
pub r#use: Option<String>,
```

#### Choice Types (Flatten)

Choice type fields use `#[serde(flatten)]`:
```rust
/// Indicates if the individual is deceased or not
#[serde(flatten)]
pub deceased: Option<DeceasedChoice>,
```

See [Choice Types](#choice-types) section below.

---

## Choice Types

### Overview

FHIR allows polymorphic fields with multiple possible types, denoted by `[x]` suffix:
- `Patient.deceased[x]` can be `deceasedBoolean` or `deceasedDateTime`
- `Observation.value[x]` can be `valueQuantity`, `valueString`, etc.

### Detection Logic

An element is a choice type if:
1. Path contains `[x]` suffix, OR
2. Multiple sibling elements share the same base path with different type suffixes

Example from schema:
```json
{
  "id": "Patient.deceased[x]",
  "path": "Patient.deceased",
  "type": [
    {"code": "boolean"},
    {"code": "dateTime"}
  ]
}
```

### Generation Pattern

For each choice type, generate an enum:

```rust
/// Choice type for Patient.deceased field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```

### Enum Naming Convention

| FHIR Element | Enum Name | Variants |
|--------------|-----------|----------|
| `deceased[x]` | `DeceasedChoice` | `Boolean`, `DateTime` |
| `value[x]` | `ValueChoice` | `Quantity`, `String`, `CodeableConcept`, etc. |
| `multipleBirth[x]` | `MultipleBirthChoice` | `Boolean`, `Integer` |

**Rules**:
1. Remove `[x]` suffix from element name
2. Convert to PascalCase
3. Append "Choice"
4. Each variant is PascalCase of type name

### Serde Configuration

```rust
#[serde(untagged)]  // No discriminator field, type inferred from value
#[serde(flatten)]   // On parent struct field
```

**Why untagged?**
FHIR JSON doesn't use a type tag:
```json
{
  "deceasedBoolean": true
}
// NOT:
{
  "deceased": { "type": "boolean", "value": true }
}
```

### Flattening in Parent Struct

```rust
pub struct Patient {
    // ... other fields
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}
```

**Why flatten?**
FHIR uses separate JSON keys for each variant:
```json
{
  "resourceType": "Patient",
  "id": "example",
  "deceasedBoolean": true
}
```

Flattening allows the enum to deserialize from the parent object's keys.

### Complex Choice Types

Some R5 choice types have many variants:

```rust
/// Choice type for Observation.value field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObservationValueChoice {
    Quantity(Quantity),
    CodeableConcept(CodeableConcept),
    String(String),
    Boolean(bool),
    Integer(i32),
    Range(Range),
    Ratio(Ratio),
    SampledData(SampledData),
    Time(String),
    DateTime(String),
    Period(Period),
    Attachment(Attachment),
    Reference(Reference),
}
```

**Challenge**: Deserialization ordering matters with `#[serde(untagged)]`. Place more specific types before more general ones.

---

## BackboneElement Generation

### Overview

FHIR resources often contain nested complex elements defined inline (BackboneElements). These are generated as separate structs.

### Example: Patient.contact

From schema:
```json
{
  "id": "Patient.contact",
  "path": "Patient.contact",
  "min": 0,
  "max": "*",
  "type": [{"code": "BackboneElement"}]
}
```

Generated code:

**patient.rs**:
```rust
pub struct Patient {
    // ...
    pub contact: Option<Vec<PatientContact>>,
}
```

**patient/contact.rs**:
```rust
/// A contact party (e.g. guardian, partner, friend) for the patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,
    
    /// The kind of relationship
    pub relationship: Option<Vec<CodeableConcept>>,
    
    /// A name associated with the contact person
    pub name: Option<HumanName>,
    
    /// A contact detail for the person
    pub telecom: Option<Vec<ContactPoint>>,
    
    /// Address for the contact person
    pub address: Option<Address>,
    
    /// male | female | other | unknown
    pub gender: Option<String>,
    
    /// Organization that is associated with the contact
    pub organization: Option<Reference>,
    
    /// The period during which this contact person or organization is valid
    pub period: Option<Period>,
}
```

**patient/mod.rs**:
```rust
//! Patient nested modules

pub mod contact;
pub mod communication;

pub use contact::PatientContact;
pub use communication::PatientCommunication;
```

### Nested BackboneElement Naming

| FHIR Path | Struct Name | File Location |
|-----------|-------------|---------------|
| `Patient.contact` | `PatientContact` | `patient/contact.rs` |
| `Patient.communication` | `PatientCommunication` | `patient/communication.rs` |
| `Bundle.entry` | `BundleEntry` | `bundle/entry.rs` |
| `Observation.component` | `ObservationComponent` | `observation/component.rs` |

**Naming Convention**:
- ParentResourceName + BackboneElementName (both PascalCase)

---

## Enums from ValueSets

### Strong Bindings

When an element has a required binding to a ValueSet with a small set of codes, generate an enum:

```rust
/// Administrative gender codes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AdministrativeGender {
    Male,
    Female,
    Other,
    Unknown,
}
```

**Usage**:
```rust
pub struct Patient {
    // ...
    pub gender: Option<AdministrativeGender>,
}
```

**Challenge**: Determining which ValueSets to generate enums for.

**Current Approach**: Use String for all coded fields, as ValueSet expansion is complex. Future enhancement could generate enums for common ValueSets.

---

## Documentation Generation

### Struct Documentation

Generated from StructureDefinition:

```rust
/// {definition}
///
/// {additional notes if present}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patient { ... }
```

### Field Documentation

Generated from element.short:

```rust
/// An identifier for this patient
pub identifier: Option<Vec<Identifier>>,
```

If element.short is missing, use element.definition (truncated if too long).

### Module-Level Documentation

**types.rs**:
```rust
//! FHIR R5 Complex Types
//!
//! This module contains complex data types used across FHIR R5 resources.
//! Auto-generated from profiles-types.json.
```

**resource files** (patient.rs):
```rust
//! Patient Resource
//!
//! Demographics and other administrative information about an individual
//! receiving care or other health-related services.
//!
//! Auto-generated from FHIR R5 (5.0.0) schema definitions.
```

---

## Formatting and Style

### Code Formatting

After generation, run rustfmt:

```rust
crate::common::writer::format_code(output_dir)?;
```

This ensures:
- Consistent indentation (4 spaces)
- Line length < 100 characters (where possible)
- Proper spacing around operators

### Import Organization

Each generated file includes necessary imports:

```rust
use serde::{Deserialize, Serialize};

// For types.rs, no additional imports needed

// For resources, import from types module:
use super::types::*;
```

### Derive Macros

All generated structs have:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
```

**Why these traits?**
- `Debug` - Essential for development and debugging
- `Clone` - FHIR resources are often cloned when processing
- `Serialize` - Convert Rust → JSON
- `Deserialize` - Convert JSON → Rust

### Visibility

All generated structs and fields are `pub`:

```rust
pub struct Patient {
    pub id: Option<String>,
    // ...
}
```

This allows users of the library to construct and access FHIR resources freely.

---

## Special Cases

### Recursive Types

Some FHIR types are recursive (e.g., Extension can contain Extension):

```rust
pub struct Extension {
    pub url: String,
    pub value: Option<ValueChoice>,
    pub extension: Option<Vec<Box<Extension>>>,  // Recursive
}
```

**Issue**: Direct recursion causes infinite size.

**Solution**: Wrap recursive field in `Box<T>` (heap allocation).

**Note**: Current generator may not handle this automatically. Manual fix required, or enhance generator to detect recursive types.

### Reserved Keywords

Escape Rust keywords with `r#` prefix:

| FHIR | Rust |
|------|------|
| `use` | `r#use` |
| `type` | `r#type` |
| `match` | `r#match` |
| `mod` | `r#mod` |

Serde `rename_all = "camelCase"` handles serialization correctly.

### Empty Structs

Some FHIR complex types may have no fields in the snapshot. Generate anyway:

```rust
/// Placeholder for empty FHIR type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyType {}
```

### Extensions

FHIR extension mechanism is complex. For now, represent as:

```rust
pub struct Extension {
    pub url: String,
    pub value: Option<ExtensionValueChoice>,  // Large choice type
}
```

---

## Generated Code Statistics (Estimated)

For FHIR R5:
- **Resources**: ~150 resource types
- **Complex Types**: ~100 types in types.rs
- **BackboneElements**: ~300 nested structs across resources
- **Choice Type Enums**: ~50 enums
- **Total Lines of Code**: ~100,000 lines (formatted)
- **Total Files**: ~200 files

---

## Validation

After generation:

1. **Compile Check**:
   ```bash
   cargo check --package fhir-resources
   ```

2. **Format Verification**:
   ```bash
   cargo fmt -- --check
   ```

3. **Clippy Lints**:
   ```bash
   cargo clippy --package fhir-resources
   ```

4. **Test Deserialization**:
   ```rust
   #[test]
   fn test_patient_deserialize() {
       let json = include_str!("../../test-data/r5/patient-example.json");
       let _patient: r5::Patient = serde_json::from_str(json).unwrap();
   }
   ```

---

## Future Enhancements

1. **ValueSet Enum Generation**: Generate enums for common required ValueSets
2. **Validation Methods**: Add `validate()` methods to check cardinality, invariants
3. **Builder Pattern**: Generate builder APIs for constructing resources
4. **FHIRPath Support**: Generate methods for FHIRPath expression evaluation
5. **Reference Type Safety**: Generic `Reference<T>` to ensure type-safe references
6. **Narrative Generation**: Auto-generate human-readable text summaries
7. **Recursive Type Detection**: Automatically detect and box recursive fields

---

## Example Generated Files

### types.rs (excerpt)

```rust
//! FHIR R5 Complex Types
//!
//! Auto-generated from FHIR R5 (5.0.0) profiles-types.json

use serde::{Deserialize, Serialize};

/// A name, normally of a human, that can be used for other living entities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    pub id: Option<String>,
    pub r#use: Option<String>,
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
    pub period: Option<Period>,
}

/// A physical or postal address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub id: Option<String>,
    pub r#use: Option<String>,
    pub r#type: Option<String>,
    pub text: Option<String>,
    pub line: Option<Vec<String>>,
    pub city: Option<String>,
    pub district: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub period: Option<Period>,
}

// ... more types
```

### patient.rs (excerpt)

```rust
//! Patient Resource
//!
//! Demographics and other administrative information

use serde::{Deserialize, Serialize};
use super::types::*;

pub mod contact;
pub mod communication;

pub use contact::PatientContact;
pub use communication::PatientCommunication;

/// Demographics and other administrative information about an individual
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub id: Option<String>,
    pub identifier: Option<Vec<Identifier>>,
    pub active: Option<bool>,
    pub name: Option<Vec<HumanName>>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
    
    pub address: Option<Vec<Address>>,
    pub contact: Option<Vec<PatientContact>>,
}

/// Choice type for Patient.deceased field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```
