# FHIR R4 Type Mapping Rules

## Overview

This document defines the rules for mapping FHIR types to Rust types. The mapping preserves FHIR semantics while providing idiomatic Rust APIs.

## Primitive Type Mapping

### FHIR Primitives → Rust Types

| FHIR Type | Rust Type | Notes |
|-----------|-----------|-------|
| `string` | `String` | UTF-8 string |
| `boolean` | `bool` | true/false |
| `integer` | `i32` | Signed 32-bit integer |
| `unsignedInt` | `u32` | Unsigned 32-bit integer |
| `positiveInt` | `u32` | Positive integer (validated at runtime) |
| `decimal` | `f64` | 64-bit floating point |
| `uri` | `String` | URI string |
| `url` | `String` | URL string |
| `canonical` | `String` | Canonical URL |
| `oid` | `String` | OID string |
| `uuid` | `String` | UUID string |
| `code` | `String` | Code from ValueSet (or enum if bound) |
| `id` | `String` | Resource ID |
| `markdown` | `String` | Markdown text |
| `base64Binary` | `String` | Base64-encoded binary |
| `instant` | `String` | ISO 8601 instant |
| `date` | `String` | YYYY-MM-DD |
| `dateTime` | `String` | ISO 8601 date-time |
| `time` | `String` | HH:MM:SS |
| `xhtml` | `String` | XHTML narrative |

**Rationale**: FHIR primitives are represented as strings in JSON, so we use String for most types. Future enhancement could add newtype wrappers for validation (e.g., `struct FhirDate(String)`).

## Complex Type Mapping

### FHIR Complex Types → Rust Structs

Complex types from `profiles-types.json` are generated as Rust structs:

| FHIR Type | Rust Type | Location |
|-----------|-----------|----------|
| `Element` | `Element` | `r4/types.rs` |
| `BackboneElement` | `BackboneElement` | `r4/types.rs` |
| `HumanName` | `HumanName` | `r4/types.rs` |
| `Address` | `Address` | `r4/types.rs` |
| `ContactPoint` | `ContactPoint` | `r4/types.rs` |
| `Period` | `Period` | `r4/types.rs` |
| `Identifier` | `Identifier` | `r4/types.rs` |
| `CodeableConcept` | `CodeableConcept` | `r4/types.rs` |
| `Coding` | `Coding` | `r4/types.rs` |
| `Quantity` | `Quantity` | `r4/types.rs` |
| `Range` | `Range` | `r4/types.rs` |
| `Ratio` | `Ratio` | `r4/types.rs` |
| `Attachment` | `Attachment` | `r4/types.rs` |
| `Reference` | `Reference<T>` | `crates/types/src/lib.rs` |

**Example**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    pub r#use: Option<HumanNameUseCode>,
    pub text: Option<String>,
    pub family: Option<String>,
    pub given: Option<Vec<String>>,
    pub prefix: Option<Vec<String>>,
    pub suffix: Option<Vec<String>>,
    pub period: Option<Period>,
}
```

## Cardinality Mapping

FHIR uses `min` and `max` to define cardinality. Map to Rust Option/Vec:

### Mapping Rules

| min | max | FHIR Cardinality | Rust Type | Rust Expression |
|-----|-----|------------------|-----------|-----------------|
| 0 | 1 | Optional single | `Option<T>` | `field: Option<T>` |
| 0 | * | Optional multiple | `Option<Vec<T>>` | `field: Option<Vec<T>>` |
| 1 | 1 | Required single | `T` | `field: T` |
| 1 | * | Required multiple | `Vec<T>` | `field: Vec<T>` |
| n | m | Constrained | `Vec<T>` | `field: Vec<T>` (validate at runtime) |

### Examples

**Patient.identifier** (min=0, max=*):
```json
{
  "id": "Patient.identifier",
  "path": "Patient.identifier",
  "min": 0,
  "max": "*",
  "type": [{"code": "Identifier"}]
}
```
→ `pub identifier: Option<Vec<Identifier>>`

**Patient.active** (min=0, max=1):
```json
{
  "id": "Patient.active",
  "path": "Patient.active",
  "min": 0,
  "max": "1",
  "type": [{"code": "boolean"}]
}
```
→ `pub active: Option<bool>`

**Bundle.entry** (min=0, max=*):
```json
{
  "id": "Bundle.entry",
  "path": "Bundle.entry",
  "min": 0,
  "max": "*",
  "type": [{"code": "BackboneElement"}]
}
```
→ `pub entry: Option<Vec<BundleEntry>>`

### Rationale

- `Option<Vec<T>>` vs `Vec<T>`: FHIR distinguishes between "absent" (null) and "empty array" ([]). Use Option to preserve this distinction.
- Required fields are rare in FHIR R4, most are optional.

## Choice Types

FHIR allows elements to have multiple types, indicated by `[x]` in the path.

### Detection

Choice types have:
1. Multiple child elements with same base path
2. Different type suffixes (e.g., `deceasedBoolean`, `deceasedDateTime`)
3. Parent path ends with `[x]` (e.g., `Patient.deceased[x]`)

### Mapping Strategy

Generate an enum with variants for each type:

**FHIR Schema**:
```json
{
  "id": "Patient.deceased[x]",
  "path": "Patient.deceased",
  "min": 0,
  "max": "1",
  "type": [
    {"code": "boolean"},
    {"code": "dateTime"}
  ]
}
```

**Rust Code**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}

pub struct Patient {
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}
```

### Naming Convention

- Element name: Remove `[x]` suffix → `deceased`
- Enum name: PascalCase + "Choice" → `DeceasedChoice`
- Variant names: PascalCase type name → `Boolean`, `DateTime`

### Common Choice Types

| FHIR Element | Types | Rust Enum |
|--------------|-------|-----------|
| `value[x]` | boolean, integer, string, dateTime, Quantity, etc. | `ValueChoice` |
| `deceased[x]` | boolean, dateTime | `DeceasedChoice` |
| `multipleBirth[x]` | boolean, integer | `MultipleBirthChoice` |
| `onset[x]` | dateTime, Age, Period, Range | `OnsetChoice` |

### Serde Annotations

- `#[serde(untagged)]` - Deserialize based on value type, not tag
- `#[serde(flatten)]` - Flatten choice field into parent struct

## Reference Types

FHIR references point to other resources. Use generic `Reference<T>` type:

**FHIR Schema**:
```json
{
  "id": "Patient.managingOrganization",
  "path": "Patient.managingOrganization",
  "type": [{
    "code": "Reference",
    "targetProfile": ["http://hl7.org/fhir/StructureDefinition/Organization"]
  }]
}
```

**Rust Code**:
```rust
pub struct Patient {
    pub managing_organization: Option<Reference<Organization>>,
}
```

**Generic Reference Type** (in `crates/types/src/lib.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference<T> {
    pub reference: Option<String>,
    pub r#type: Option<String>,
    pub identifier: Option<Identifier>,
    pub display: Option<String>,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<T>,
}
```

### Multiple Target Profiles

If multiple targets, use enum or `Reference<Resource>`:

```json
{
  "type": [{
    "code": "Reference",
    "targetProfile": [
      "http://hl7.org/fhir/StructureDefinition/Practitioner",
      "http://hl7.org/fhir/StructureDefinition/Organization"
    ]
  }]
}
```

→ `pub performer: Option<Reference<Resource>>` (or union type)

## BackboneElement Types

BackboneElements are inline complex types defined within a resource.

### Detection

Elements with `type: BackboneElement` that are nested within a resource.

### Mapping Strategy

Generate separate struct with name based on parent path:

**FHIR Schema**:
```json
{
  "id": "Patient.contact",
  "path": "Patient.contact",
  "type": [{"code": "BackboneElement"}],
  "snapshot": {
    "element": [
      {
        "id": "Patient.contact.relationship",
        "path": "Patient.contact.relationship",
        "type": [{"code": "CodeableConcept"}]
      },
      {
        "id": "Patient.contact.name",
        "path": "Patient.contact.name",
        "type": [{"code": "HumanName"}]
      }
    ]
  }
}
```

**Rust Code** (separate file `patient/contact.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    pub relationship: Option<Vec<CodeableConcept>>,
    pub name: Option<HumanName>,
    pub telecom: Option<Vec<ContactPoint>>,
    pub address: Option<Address>,
    pub organization: Option<Reference<Organization>>,
}
```

**Parent Resource** (`patient.rs`):
```rust
pub struct Patient {
    pub contact: Option<Vec<contact::PatientContact>>,
}
```

### Naming Convention

- Struct name: `{ParentResource}{NestedElementName}` → `PatientContact`
- Module name: `{parent_resource}/{nested_element}` → `patient/contact.rs`

## ValueSet Bindings (Code Enums)

FHIR codes can be bound to ValueSets. Generate enums for strong typing:

### Detection

Elements with `type: code` and a `binding`:

```json
{
  "id": "Address.use",
  "path": "Address.use",
  "type": [{"code": "code"}],
  "binding": {
    "strength": "required",
    "valueSet": "http://hl7.org/fhir/ValueSet/address-use",
    "description": "home | work | temp | old | billing"
  }
}
```

### Mapping Strategy

Parse `binding.description` to extract enum variants:

**Rust Code**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AddressUseCode {
    Home,
    Work,
    Temp,
    Old,
    Billing,
}

pub struct Address {
    pub r#use: Option<AddressUseCode>,
}
```

### Parsing Algorithm

1. Extract `binding.description`
2. Split by `|`
3. Trim whitespace
4. Convert to PascalCase for variant names
5. Add `#[serde(rename = "original")]` if needed

### Binding Strength

| Strength | Strategy |
|----------|----------|
| `required` | Generate enum |
| `extensible` | Generate enum + Unknown variant |
| `preferred` | Use `String` (too loose for enum) |
| `example` | Use `String` |

## Reserved Keywords

Rust keywords must be escaped with `r#`:

| FHIR Name | Rust Name | Serde Rename |
|-----------|-----------|--------------|
| `type` | `r#type` | `#[serde(rename = "type")]` |
| `use` | `r#use` | `#[serde(rename = "use")]` |
| `match` | `r#match` | `#[serde(rename = "match")]` |
| `ref` | `r#ref` | `#[serde(rename = "ref")]` |

**Example**:
```rust
pub struct Address {
    #[serde(rename = "use")]
    pub r#use: Option<AddressUseCode>,
    
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}
```

## Naming Conventions

### Field Names

Convert FHIR camelCase to Rust snake_case:

| FHIR Name | Rust Name |
|-----------|-----------|
| `resourceType` | `resource_type` |
| `managingOrganization` | `managing_organization` |
| `birthDate` | `birth_date` |

Use `#[serde(rename_all = "camelCase")]` on struct to auto-convert.

### Type Names

Keep PascalCase:

| FHIR Name | Rust Name |
|-----------|-----------|
| `Patient` | `Patient` |
| `HumanName` | `HumanName` |
| `CodeableConcept` | `CodeableConcept` |

### Module Names

Convert to snake_case:

| Resource | Module |
|----------|--------|
| `Patient` | `patient.rs` |
| `BodyStructure` | `body_structure.rs` |
| `MedicationRequest` | `medication_request.rs` |

## Special Cases

### Arrays

FHIR arrays preserve order:
- `Patient.name` → first name is "official", rest may not be
- `Address.line` → order matters for address labels
- `HumanName.given` → first is primary, rest are middle names

Use `Vec<T>` to preserve order.

### Extensions

FHIR extensions are handled as:
```rust
pub struct Element {
    pub id: Option<String>,
    pub extension: Option<Vec<Extension>>,
}
```

Every element can have extensions. BackboneElements also have `modifierExtension`.

### Polymorphic Resources

Some fields accept multiple resource types (e.g., `Bundle.entry.resource`):
```rust
pub enum Resource {
    Patient(Patient),
    Observation(Observation),
    // ... all resource types
}
```

Generate large enum with all possible resource types.

## Type Mapping Summary

```
FHIR Schema
    ↓
Element Definition
    ↓
[Primitive Mapping]
    ↓
[Cardinality Wrapping]
    ↓
[Choice Type Detection]
    ↓
[Reference Type Handling]
    ↓
[BackboneElement Extraction]
    ↓
[ValueSet Enum Generation]
    ↓
Rust Type
```
