# FHIR R5 Type Mapping Rules

## Overview

This document defines the rules for mapping FHIR R5 types to Rust types. The mapping preserves FHIR semantics while providing idiomatic Rust APIs. R5 introduces some new types compared to R4, which are documented here.

## Primitive Type Mapping

### FHIR Primitives → Rust Types

| FHIR Type | Rust Type | Format/Constraints | Notes |
|-----------|-----------|-------------------|-------|
| `string` | `String` | UTF-8 | General string |
| `boolean` | `bool` | true/false | Boolean value |
| `integer` | `i32` | Signed 32-bit | -2^31 to 2^31-1 |
| `unsignedInt` | `u32` | Unsigned 32-bit | 0 to 2^32-1 |
| `positiveInt` | `u32` | > 0 | Positive integers only |
| `integer64` | `i64` | Signed 64-bit | **New in R5** |
| `decimal` | `f64` | 64-bit float | Arbitrary precision in FHIR |
| `uri` | `String` | URI format | Uniform Resource Identifier |
| `url` | `String` | URL format | Uniform Resource Locator |
| `canonical` | `String` | Canonical URL | Reference to FHIR definition |
| `oid` | `String` | urn:oid:[0-9.]+ | Object Identifier |
| `uuid` | `String` | urn:uuid:... | Universally Unique Identifier |
| `code` | `String` | [^\\s]+ | Code from terminology |
| `id` | `String` | [A-Za-z0-9\\-\\.]{1,64} | Logical ID |
| `markdown` | `String` | CommonMark | Markdown formatted text |
| `base64Binary` | `String` | Base64 encoded | Binary data |
| `instant` | `String` | YYYY-MM-DDThh:mm:ss.sss+zz:zz | Instant in time |
| `date` | `String` | YYYY, YYYY-MM, YYYY-MM-DD | Date |
| `dateTime` | `String` | YYYY-MM-DDThh:mm:ss+zz:zz | Date and time |
| `time` | `String` | hh:mm:ss | Time of day |
| `xhtml` | `String` | XHTML fragment | Narrative content |

**R5 New Types**:
- `integer64` - 64-bit integer support for large values
- More stringent validation rules for some existing primitives

**Rationale**: 
- FHIR primitives are represented as strings in JSON
- Use native Rust types (bool, i32, f64) where FHIR JSON uses native JSON types
- String for most others to preserve exact format
- Future enhancement: newtype wrappers for validation (e.g., `struct FhirDate(String)`)

**Validation**:
Current implementation does **not** validate formats (e.g., date format, regex patterns). Validation is runtime responsibility.

---

## Complex Type Mapping

### FHIR Complex Types → Rust Structs

All complex types from `profiles-types.json` are generated as Rust structs in `r5/types.rs`:

| FHIR Type | Rust Type | Description |
|-----------|-----------|-------------|
| `Element` | `Element` | Base element with id and extensions |
| `BackboneElement` | `BackboneElement` | Base for nested elements within resources |
| `HumanName` | `HumanName` | Person's name |
| `Address` | `Address` | Physical or postal address |
| `ContactPoint` | `ContactPoint` | Phone, email, etc. |
| `Period` | `Period` | Time range with start and end |
| `Identifier` | `Identifier` | Unique identifier |
| `CodeableConcept` | `CodeableConcept` | Coded concept with text |
| `Coding` | `Coding` | Code defined by a terminology system |
| `Quantity` | `Quantity` | Measured or measurable amount |
| `Range` | `Range` | Set of values bounded by low and high |
| `Ratio` | `Ratio` | Relationship between two Quantity values |
| `Attachment` | `Attachment` | Binary content with metadata |
| `Annotation` | `Annotation` | Text note with author |
| `Signature` | `Signature` | Digital signature |
| `SampledData` | `SampledData` | Sampled data (e.g., waveform) |
| `Timing` | `Timing` | Schedule for recurring events |
| `Duration` | `Duration` | Length of time |
| `Age` | `Age` | Duration measured in years, months, etc. |
| `Money` | `Money` | Monetary amount |
| `Count` | `Count` | Countable quantity |
| `Distance` | `Distance` | Measured distance |
| `Reference` | `Reference` | Reference to another resource |
| `Meta` | `Meta` | Resource metadata |
| `Narrative` | `Narrative` | Human-readable text summary |
| `Extension` | `Extension` | Extension mechanism |

**R5 additions** (compared to R4):
- Some new complex types introduced in R5
- Existing types may have new fields

**Example Usage**:
```rust
pub struct Patient {
    pub name: Option<Vec<HumanName>>,
    pub address: Option<Vec<Address>>,
    pub telecom: Option<Vec<ContactPoint>>,
}
```

---

## Cardinality Mapping

FHIR uses `min` and `max` to define cardinality. Map to Rust `Option`/`Vec`:

### Mapping Rules

| min | max | FHIR Meaning | Rust Type | Example |
|-----|-----|--------------|-----------|---------|
| 0 | 1 | Optional single value | `Option<T>` | `active: Option<bool>` |
| 0 | * | Optional array | `Option<Vec<T>>` | `identifier: Option<Vec<Identifier>>` |
| 1 | 1 | Required single value | `T` | `status: String` |
| 1 | * | Required array (non-empty) | `Vec<T>` | `entry: Vec<BundleEntry>` |
| n | m | Constrained cardinality | `Vec<T>` | Runtime validation needed |

### Detailed Examples

**Patient.identifier** (min=0, max=*):
```json
{
  "path": "Patient.identifier",
  "min": 0,
  "max": "*",
  "type": [{"code": "Identifier"}]
}
```
→ **Rust**: `pub identifier: Option<Vec<Identifier>>`

**Patient.active** (min=0, max=1):
```json
{
  "path": "Patient.active",
  "min": 0,
  "max": "1",
  "type": [{"code": "boolean"}]
}
```
→ **Rust**: `pub active: Option<bool>`

**Bundle.type** (min=1, max=1):
```json
{
  "path": "Bundle.type",
  "min": 1,
  "max": "1",
  "type": [{"code": "code"}]
}
```
→ **Rust**: `pub bundle_type: String` (required, no Option)

**Bundle.entry** (min=0, max=*):
```json
{
  "path": "Bundle.entry",
  "min": 0,
  "max": "*",
  "type": [{"code": "BackboneElement"}]
}
```
→ **Rust**: `pub entry: Option<Vec<BundleEntry>>`

### Rationale

**Why `Option<Vec<T>>` instead of `Vec<T>` for min=0, max=*?**

FHIR distinguishes between:
- **Absent**: Field not present in JSON → `None`
- **Empty**: Field present with empty array → `Some(vec![])`

Example JSON:
```json
{
  "resourceType": "Patient",
  "id": "example"
  // identifier field is absent
}
```
→ `identifier: None`

```json
{
  "resourceType": "Patient",
  "id": "example",
  "identifier": []
  // identifier field is present but empty
}
```
→ `identifier: Some(vec![])`

This distinction can be semantically important in FHIR.

**Why `Vec<T>` for min=1, max=*?**

The field is required, so it's always present. No need for Option.

---

## Choice Types

FHIR allows elements to have multiple possible types, indicated by `[x]` suffix in the element path.

### Detection

A choice type is identified by:
1. Element path ends with `[x]` (e.g., `Patient.deceased[x]`)
2. Element has multiple type codes in `type[]` array

### Mapping Strategy

Generate an enum with variants for each possible type:

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
/// Choice type for Patient.deceased field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}

pub struct Patient {
    // ...
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}
```

### Naming Convention

| FHIR Element | Element Name | Enum Name | Variants |
|--------------|--------------|-----------|----------|
| `deceased[x]` | `deceased` | `DeceasedChoice` | `Boolean`, `DateTime` |
| `value[x]` | `value` | `ValueChoice` | `Quantity`, `String`, `Integer`, etc. |
| `multipleBirth[x]` | `multiple_birth` | `MultipleBirthChoice` | `Boolean`, `Integer` |
| `onset[x]` | `onset` | `OnsetChoice` | `DateTime`, `Age`, `Period`, `Range`, `String` |

**Rules**:
1. Remove `[x]` suffix from path
2. Convert to snake_case for field name
3. Convert to PascalCase for enum name
4. Append "Choice" to enum name
5. Each variant is PascalCase of FHIR type code

### JSON Serialization

FHIR serializes choice types with the type in the field name:

**deceasedBoolean**:
```json
{
  "resourceType": "Patient",
  "id": "example",
  "deceasedBoolean": true
}
```

**deceasedDateTime**:
```json
{
  "resourceType": "Patient",
  "id": "example",
  "deceasedDateTime": "2015-02-14T13:42:00+10:00"
}
```

### Serde Configuration

```rust
#[serde(untagged)]  // No discriminator, infer type from value
#[serde(flatten)]   // Flatten into parent object
```

**Why untagged?**
- No type discriminator field in FHIR JSON
- Serde tries each variant in order until one succeeds

**Why flatten?**
- Choice type field is serialized directly into parent object
- Field name includes the type (e.g., `deceasedBoolean`)

### Common Choice Types in R5

| Element | Possible Types | Use Case |
|---------|----------------|----------|
| `value[x]` | Quantity, CodeableConcept, String, Boolean, Integer, Range, Ratio, SampledData, Time, DateTime, Period, Attachment | Observation values, extension values |
| `deceased[x]` | Boolean, DateTime | Patient death status |
| `multipleBirth[x]` | Boolean, Integer | Birth plurality |
| `onset[x]` | DateTime, Age, Period, Range, String | Condition/Allergy onset |
| `abatement[x]` | DateTime, Age, Period, Range, String, Boolean | Condition end |
| `occurrence[x]` | DateTime, Period, Timing | Event timing |
| `performed[x]` | DateTime, Period, String, Age, Range | Procedure timing |

### Large Choice Types

Some choice types have many variants (10+):

```rust
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
    SampledData(SampledData),
    Time(String),
    DateTime(String),
    Period(Period),
    Attachment(Attachment),
}
```

**Warning**: Order matters with `#[serde(untagged)]`. Serde tries variants in declaration order. Place more specific types before more general ones.

---

## Reference Types

FHIR references point to other resources.

### Basic Reference Type

```rust
/// Reference to another resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    /// Unique id for inter-element referencing
    pub id: Option<String>,
    
    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<String>,
    
    /// Type the reference refers to (e.g., "Patient")
    pub r#type: Option<String>,
    
    /// Logical reference, when literal reference is not known
    pub identifier: Option<Identifier>,
    
    /// Text alternative for the resource
    pub display: Option<String>,
}
```

### Usage in Resources

```rust
pub struct Observation {
    // ...
    
    /// Who/what the observation is about
    pub subject: Option<Reference>,
    
    /// Who performed the observation
    pub performer: Option<Vec<Reference>>,
}
```

### Generic Reference (Future Enhancement)

Could add phantom type parameter for type safety:

```rust
pub struct Reference<T> {
    pub reference: Option<String>,
    pub display: Option<String>,
    _phantom: PhantomData<T>,
}

pub struct Observation {
    pub subject: Option<Reference<Patient>>,  // Only Patient references
}
```

**Not implemented in current generator**, but could be added.

---

## Resource-Specific Types

### DomainResource Base

Most resources inherit from DomainResource:

```rust
/// Base resource with text, contained resources, and extensions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainResource {
    /// Logical id
    pub id: Option<String>,
    
    /// Metadata
    pub meta: Option<Meta>,
    
    /// A set of rules under which this content was created
    pub implicit_rules: Option<String>,
    
    /// Language of the resource content
    pub language: Option<String>,
    
    /// Text summary of the resource
    pub text: Option<Narrative>,
    
    /// Contained, inline Resources
    pub contained: Option<Vec<Resource>>,
    
    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
    
    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<Extension>>,
}
```

**Note**: Current generator duplicates these fields in each resource rather than using composition/inheritance. Rust lacks inheritance, so this is acceptable.

---

## BackboneElement Types

BackboneElements are complex types defined inline within resources.

### Example: Patient.contact

```rust
/// A contact party for the patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatientContact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,
    
    /// Extensions
    pub extension: Option<Vec<Extension>>,
    
    /// Extensions that cannot be ignored
    pub modifier_extension: Option<Vec<Extension>>,
    
    /// The kind of relationship
    pub relationship: Option<Vec<CodeableConcept>>,
    
    /// Name of the contact person
    pub name: Option<HumanName>,
    
    /// Contact details
    pub telecom: Option<Vec<ContactPoint>>,
    
    /// Address for the contact
    pub address: Option<Address>,
    
    /// Gender
    pub gender: Option<String>,
    
    /// Organization associated with contact
    pub organization: Option<Reference>,
    
    /// Time period when contact was/is valid
    pub period: Option<Period>,
}
```

### Naming

| FHIR Path | Struct Name |
|-----------|-------------|
| `Patient.contact` | `PatientContact` |
| `Patient.communication` | `PatientCommunication` |
| `Bundle.entry` | `BundleEntry` |
| `Bundle.entry.request` | `BundleEntryRequest` |

**Pattern**: `{ResourceName}{BackboneElementPath}`

---

## Special Types

### Extension

FHIR's extension mechanism allows adding custom data:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    /// Unique id
    pub id: Option<String>,
    
    /// Extension URL (identifies the extension)
    pub url: String,
    
    /// Value of extension
    #[serde(flatten)]
    pub value: Option<ExtensionValueChoice>,  // Large choice type
    
    /// Nested extensions
    pub extension: Option<Vec<Box<Extension>>>,  // Recursive
}
```

**Note**: Recursive type requires `Box<Extension>` to avoid infinite size.

### Resource Polymorphism

Bundle.entry.resource can contain any resource type:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "resourceType")]
pub enum Resource {
    Patient(Patient),
    Observation(Observation),
    Practitioner(Practitioner),
    // ... all resource types
}
```

**Challenge**: Generating this enum requires knowing all resource types.

**Current Approach**: Use `serde_json::Value` for polymorphic resource fields. Future enhancement could generate the Resource enum.

---

## Type Mapping Summary Table

| FHIR Category | FHIR Example | Rust Type | Notes |
|---------------|--------------|-----------|-------|
| Primitive (string-like) | `string`, `uri`, `code` | `String` | Direct mapping |
| Primitive (numeric) | `integer`, `decimal` | `i32`, `f64` | Native types |
| Primitive (boolean) | `boolean` | `bool` | Native type |
| Complex Type | `HumanName`, `Address` | `HumanName`, `Address` | Generated struct |
| Reference | `Reference(Patient)` | `Reference` | Generic reference |
| Choice Type | `value[x]` | `ValueChoice` | Enum with variants |
| BackboneElement | `Patient.contact` | `PatientContact` | Generated struct |
| Resource | `Patient`, `Observation` | `Patient`, `Observation` | Generated struct |
| Optional single | min=0, max=1 | `Option<T>` | May be absent |
| Optional multiple | min=0, max=* | `Option<Vec<T>>` | May be absent or empty |
| Required single | min=1, max=1 | `T` | Always present |
| Required multiple | min=1, max=* | `Vec<T>` | Always present, may be empty |

---

## Validation Considerations

### Compile-Time Validation

Rust's type system enforces:
- Required fields exist (min=1 fields are not Option)
- Type correctness (bool fields are bool, not String)
- Cardinality structure (single vs array)

### Runtime Validation (Not Implemented)

The generator does **not** currently validate:
- Format constraints (date format, regex patterns)
- Value constraints (positiveInt > 0, code in ValueSet)
- Cardinality constraints (min=2, max=5)
- Business rules (invariants, FHIRPath constraints)

**Future Enhancement**: Add `validate()` methods to check these constraints.

---

## R5-Specific Type Changes

Compared to R4, R5 may have:

1. **New Primitive Types**
   - `integer64` for large integers

2. **Modified Complex Types**
   - Additional fields in existing types
   - Changed cardinality on some fields

3. **New Complex Types**
   - R5 introduces new data types for emerging use cases

4. **Deprecated Types**
   - Some R4 types removed or replaced in R5

**Recommendation**: Review R5 changelog for specific changes when implementing.

---

## Future Type System Enhancements

1. **Newtype Wrappers**
   ```rust
   pub struct FhirDate(String);
   pub struct FhirUri(String);
   ```
   - Add validation in constructors
   - Implement `FromStr` and `Display`

2. **Generic Reference**
   ```rust
   pub struct Reference<T: Resource> { ... }
   ```
   - Type-safe resource references

3. **ValueSet Enums**
   - Generate enums for strongly-bound ValueSets
   - Safer than String for coded fields

4. **Builder Pattern**
   ```rust
   Patient::builder()
       .id("example")
       .active(true)
       .name(vec![HumanName { ... }])
       .build()
   ```

5. **Validation Traits**
   ```rust
   trait FhirValidate {
       fn validate(&self) -> Result<(), ValidationError>;
   }
   ```
