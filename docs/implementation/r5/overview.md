# FHIR R5 Code Generator - Overview

## Purpose

The FHIR R5 code generator transforms FHIR JSON schema definitions into type-safe Rust structures. This enables compile-time validation of FHIR resources and provides idiomatic Rust APIs for healthcare data interchange using the latest FHIR R5 (5.0.0) specification.

## Goals

1. **Type Safety** - Convert FHIR's dynamic JSON schemas into statically-typed Rust structs
2. **Completeness** - Support all FHIR R5 resources and complex types
3. **Correctness** - Preserve FHIR semantics including cardinality, choice types, and references
4. **Maintainability** - Generate readable, well-documented code that matches Rust conventions
5. **Code Reuse** - Leverage common code generation modules shared with R4 and R4B implementations

## R5 Key Changes from R4

FHIR R5 introduces several significant changes:

- **New Resources** - Additional resource types for emerging healthcare domains
- **Enhanced Data Types** - New complex types and refined existing ones
- **Updated Cardinality** - Some elements have changed min/max constraints
- **Improved Terminology** - Better ValueSet bindings and code systems
- **Breaking Changes** - Some R4 elements removed or restructured

## Scope

### In Scope

- Parse FHIR Bundle JSON schemas (`profiles-resources.json`, `profiles-types.json`)
- Extract StructureDefinition entries
- Generate Rust structs for all R5 resources (Patient, Observation, etc.)
- Generate Rust structs for all R5 complex types (HumanName, Address, Period, etc.)
- Handle cardinality (min/max → Option/Vec)
- Handle choice types (value[x] → enum)
- Generate enums for ValueSet bindings
- Generate nested BackboneElement types
- Format generated code with rustfmt
- Provide serde serialization/deserialization

### Out of Scope

- Runtime FHIR validation (schema validation, business rules)
- FHIR server/client implementation
- Search parameter implementation
- FHIR path evaluation
- Terminology services
- Resource narrative generation
- Cross-version migration utilities (R4 → R5 conversion)

## Input

### Schema Files

Located in `schemas/r5/`:

- `profiles-resources.json` - Bundle containing FHIR R5 resource StructureDefinitions (~499K lines)
- `profiles-types.json` - Bundle containing complex type StructureDefinitions (~36K lines)
- `valuesets.json` - ValueSet definitions for code bindings
- `fhir.schema.json` - JSON Schema for validation
- `version.info` - Version metadata (5.0.0)

### Schema Directory Structure

Unlike R4B which uses `schemas/r4b/definitions.json/` subdirectory, R5 follows the R4 pattern with schemas directly in `schemas/r5/`:

```
schemas/r5/
├── profiles-resources.json    # Resource definitions
├── profiles-types.json         # Type definitions
├── valuesets.json              # Value sets
├── conceptmaps.json            # Concept maps
├── search-parameters.json      # Search parameters
├── fhir.schema.json            # JSON Schema
└── version.info                # Version metadata
```

### Schema Structure

```json
{
  "resourceType": "Bundle",
  "type": "collection",
  "entry": [
    {
      "fullUrl": "http://hl7.org/fhir/StructureDefinition/Patient",
      "resource": {
        "resourceType": "StructureDefinition",
        "id": "Patient",
        "name": "Patient",
        "kind": "resource",
        "type": "Patient",
        "snapshot": {
          "element": [
            {
              "id": "Patient.identifier",
              "path": "Patient.identifier",
              "min": 0,
              "max": "*",
              "type": [{"code": "Identifier"}]
            }
          ]
        }
      }
    }
  ]
}
```

**Note**: R5 may have additional or modified fields in StructureDefinition compared to R4.

## Output

### File Structure

Generated code will be written to `crates/resources/src/r5/`:

```
crates/resources/src/r5/
├── mod.rs                      # Module declarations
├── types.rs                    # Complex types (HumanName, Address, etc.)
├── account.rs                  # Account resource
├── patient.rs                  # Patient resource
├── observation.rs              # Observation resource
├── patient/                    # Nested BackboneElements for Patient
│   ├── mod.rs                 # Nested module declarations
│   ├── contact.rs             # Patient.contact BackboneElement
│   └── communication.rs       # Patient.communication BackboneElement
└── ...                         # Other resources
```

### Generated Code Example

```rust
/// Demographics and other administrative information about an individual receiving care or other health-related services
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
    
    /// male | female | other | unknown
    pub gender: Option<String>,
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}
```

## Implementation Strategy

### Phase 1: Generator Module Setup

1. Create `crates/codegen/src/r5_generator/` module
2. Implement `mod.rs` with main `generate()` function
3. Implement `parser.rs` to parse R5 Bundle files
4. Test parsing with sample R5 schemas

### Phase 2: Code Generation

1. Leverage `common::codegen` module for type and resource generation
2. Implement R5-specific type mappings if needed
3. Generate all resource files
4. Generate types.rs with complex types

### Phase 3: Validation & Testing

1. Verify all resources compile
2. Test serialization/deserialization with sample FHIR R5 instances
3. Document R5-specific features and differences

### Phase 4: Integration

1. Wire up R5 generator in `main.rs`
2. Update workspace Cargo.toml
3. Add documentation and examples

## Dependencies

### Shared Modules

R5 generator will use these common modules:

- `common::codegen` - Core code generation logic
- `common::type_mapper` - FHIR to Rust type mapping
- `common::writer` - File I/O and formatting

### Crate Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
```

## Migration Notes

### For Users Migrating from R4 to R5

- **Breaking Changes**: Some R4 resources/elements may not exist in R5
- **New Features**: R5 introduces new resources and data types
- **API Compatibility**: Generated Rust APIs will differ between R4 and R5
- **Recommendation**: Support both versions if interoperability is required

### For Developers

- R5 generator is modeled after R4B implementation
- Uses flat schema directory structure like R4 (not subdirectory like R4B)
- Shares common code generation modules with R4 and R4B
- May require R5-specific type mappings for new data types

## Next Steps

1. Review R5 schema differences from R4/R4B
2. Identify any R5-specific type mappings needed
3. Implement r5_generator module
4. Generate initial code and validate compilation
5. Create examples demonstrating R5 feature usage
