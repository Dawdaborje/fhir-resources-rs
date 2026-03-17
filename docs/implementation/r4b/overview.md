# FHIR R4B Code Generator - Overview

## Purpose

The FHIR R4B code generator transforms FHIR JSON schema definitions into type-safe Rust structures. This enables compile-time validation of FHIR resources and provides idiomatic Rust APIs for healthcare data interchange.

**Note**: R4B schemas are located in `schemas/r4b/definitions.json/` subdirectory, unlike R4 which has them directly in `schemas/r4/`.

## Goals

1. **Type Safety** - Convert FHIR's dynamic JSON schemas into statically-typed Rust structs
2. **Completeness** - Support all FHIR R4 resources and complex types
3. **Correctness** - Preserve FHIR semantics including cardinality, choice types, and references
4. **Maintainability** - Generate readable, well-documented code that matches Rust conventions
5. **Scalability** - R4B shares common code generation modules with R4, making it easy to support future FHIR versions

## Scope

### In Scope

- Parse FHIR Bundle JSON schemas from `schemas/r4b/definitions.json/` (`profiles-resources.json`, `profiles-types.json`)
- Extract StructureDefinition entries
- Generate Rust structs for all resources (Patient, Observation, etc.)
- Generate Rust structs for complex types (HumanName, Address, Period, etc.)
- Handle cardinality (min/max → Option/Vec)
- Handle choice types (deceased[x] → enum)
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

## Input

### Schema Files

Located in `schemas/r4/`:

- `profiles-resources.json` - Bundle containing ~140 resource StructureDefinitions
- `profiles-types.json` - Bundle containing complex type StructureDefinitions

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

## Output

### File Structure

Generated code is written to `crates/resources/src/r4/`:

```
crates/resources/src/r4/
  mod.rs                    # Module declarations
  types.rs                  # Complex types (HumanName, Address, etc.)
  account.rs                # Account resource
  patient.rs                # Patient resource
  observation.rs            # Observation resource
  patient/                  # Nested BackboneElements for Patient
    contact.rs              # Patient.contact BackboneElement
    communication.rs        # Patient.communication BackboneElement
  ...
```

### Generated Code Example

```rust
use serde::{Deserialize, Serialize};

/// Demographics and other administrative information about an individual or animal receiving care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    /// Logical id of this artifact
    pub id: Option<String>,
    
    /// An identifier for this patient
    pub identifier: Option<Vec<Identifier>>,
    
    /// Whether this patient record is in active use
    pub active: Option<bool>,
    
    /// A name associated with the patient
    pub name: Option<Vec<HumanName>>,
    
    /// Indicates if the individual is deceased or not
    #[serde(flatten)]
    pub deceased: Option<DeceasedChoice>,
}

/// Choice type for Patient.deceased[x]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeceasedChoice {
    Boolean(bool),
    DateTime(String),
}
```

## Process Flow

```
1. CLI Entry Point (main.rs)
   ↓
2. Version Dispatcher (match r4 → r4_generator)
   ↓
3. Schema Parser (parse JSON Bundle → StructureDefinition)
   ↓
4. Type Mapper (FHIR types → Rust types)
   ↓
5. Code Generator (generate struct/enum code)
   ↓
6. File Writer (write to r4/, organize modules)
   ↓
7. Formatter (rustfmt on generated files)
```

## Design Principles

1. **Separation of Concerns** - Parser, mapper, generator, and writer are separate modules
2. **Version Isolation** - Each FHIR version has its own generator module (r4_generator, r4b_generator, etc.)
3. **Deterministic** - Same input always produces same output
4. **Build-time Only** - Generator runs as a build-time tool, not at runtime
5. **Idiomatic Rust** - Generated code follows Rust naming conventions and patterns
6. **Documentation** - Preserve FHIR documentation in code comments

## Non-Goals

- Dynamic/reflection-based FHIR handling
- GraphQL or REST API generation
- Database schema generation
- Migration between FHIR versions
- FHIR package management

## Success Metrics

1. All ~140 R4 resources generate successfully
2. Generated code compiles without errors
3. Generated code passes rustfmt
4. Cardinality is correctly mapped (Option/Vec)
5. Choice types are correctly handled (enums)
6. BackboneElements generate as nested structs
7. Documentation comments are preserved
