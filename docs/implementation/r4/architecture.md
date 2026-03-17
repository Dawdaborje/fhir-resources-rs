# FHIR R4 Code Generator - Architecture

## Module Structure

```
crates/codegen/
  src/
    main.rs                      # CLI entry point, version dispatcher
    r4_generator/
      mod.rs                     # Public generate() function, orchestration
      parser.rs                  # Parse FHIR JSON Bundle and StructureDefinition
      type_mapper.rs             # Map FHIR types to Rust types
      codegen.rs                 # Generate Rust struct/enum source code
      writer.rs                  # File I/O and rustfmt integration
```

## Component Responsibilities

### CLI Entry Point (`main.rs`)

**Purpose**: Parse command-line arguments and dispatch to version-specific generator

**Interface**:
```rust
fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let version = args.get(1).context("Usage: codegen <version>")?;
    
    match version.as_str() {
        "r4" => r4_generator::generate()?,
        "r4b" => unimplemented!("R4B not yet supported"),
        "r5" => unimplemented!("R5 not yet supported"),
        "r6" => unimplemented!("R6 not yet supported"),
        _ => bail!("Unknown FHIR version: {}", version),
    }
    
    Ok(())
}
```

**Responsibilities**:
- Parse CLI arguments
- Validate version argument
- Dispatch to correct generator module
- Return error codes on failure

---

### Generator Module (`r4_generator/mod.rs`)

**Purpose**: Orchestrate the code generation pipeline

**Interface**:
```rust
pub fn generate() -> anyhow::Result<()> {
    // 1. Define paths
    let schema_dir = Path::new("schemas/r4");
    let output_dir = Path::new("crates/resources/src/r4");
    
    // 2. Parse schemas
    let resources = parser::parse_resources(schema_dir.join("profiles-resources.json"))?;
    let types = parser::parse_types(schema_dir.join("profiles-types.json"))?;
    
    // 3. Generate code
    let type_code = codegen::generate_types(&types)?;
    let resource_code = codegen::generate_resources(&resources)?;
    
    // 4. Write files
    writer::write_module(output_dir, type_code, resource_code)?;
    
    // 5. Format
    writer::format_code(output_dir)?;
    
    Ok(())
}
```

**Responsibilities**:
- Define input/output paths
- Coordinate parser → mapper → generator → writer
- Handle high-level errors
- Log progress

---

### Parser (`r4_generator/parser.rs`)

**Purpose**: Parse FHIR JSON schemas into intermediate representation

**Data Structures**:
```rust
#[derive(Debug, Deserialize)]
pub struct FhirBundle {
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    pub r#type: String,
    pub entry: Vec<BundleEntry>,
}

#[derive(Debug, Deserialize)]
pub struct BundleEntry {
    #[serde(rename = "fullUrl")]
    pub full_url: String,
    pub resource: StructureDefinition,
}

#[derive(Debug, Deserialize)]
pub struct StructureDefinition {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub r#type: String,
    pub snapshot: Snapshot,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub element: Vec<ElementDefinition>,
}

#[derive(Debug, Deserialize)]
pub struct ElementDefinition {
    pub id: String,
    pub path: String,
    pub min: u32,
    pub max: String,  // "*" or number
    pub r#type: Option<Vec<TypeRef>>,
    pub short: Option<String>,
    pub binding: Option<Binding>,
}

#[derive(Debug, Deserialize)]
pub struct TypeRef {
    pub code: String,
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Binding {
    pub strength: String,
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,
    pub description: Option<String>,
}
```

**Functions**:
```rust
pub fn parse_resources(path: PathBuf) -> anyhow::Result<Vec<StructureDefinition>>;
pub fn parse_types(path: PathBuf) -> anyhow::Result<Vec<StructureDefinition>>;
```

**Responsibilities**:
- Read JSON files
- Deserialize into typed structs
- Extract StructureDefinition entries
- Validate bundle structure
- Return parsed definitions

---

### Type Mapper (`r4_generator/type_mapper.rs`)

**Purpose**: Map FHIR types to Rust types and handle cardinality

**Data Structures**:
```rust
pub struct RustField {
    pub name: String,           // Rust field name (snake_case)
    pub rust_type: String,      // Rust type (String, Option<Vec<HumanName>>, etc.)
    pub fhir_name: String,      // Original FHIR name (for serde rename)
    pub documentation: String,  // Doc comment
    pub is_choice: bool,        // True if this is a choice type
}

pub struct RustStruct {
    pub name: String,           // PascalCase struct name
    pub documentation: String,  // Doc comment
    pub fields: Vec<RustField>,
    pub nested_types: Vec<RustStruct>,  // BackboneElements
}

pub struct RustEnum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_code_enum: bool,     // True for ValueSet enums, false for choice types
}

pub struct EnumVariant {
    pub name: String,           // Variant name
    pub rust_type: Option<String>,  // Some(Type) for choice enums, None for code enums
    pub serde_rename: Option<String>,  // Original name for serde
}
```

**Functions**:
```rust
pub fn map_primitive_type(fhir_type: &str) -> String;
pub fn map_cardinality(rust_type: &str, min: u32, max: &str) -> String;
pub fn detect_choice_type(elements: &[ElementDefinition]) -> Option<ChoiceType>;
pub fn extract_enum_variants(binding: &Binding) -> Vec<String>;
```

**Mapping Rules**:

| FHIR Type | Rust Type |
|-----------|-----------|
| `string` | `String` |
| `boolean` | `bool` |
| `integer` | `i32` |
| `decimal` | `f64` |
| `dateTime` | `String` |
| `date` | `String` |
| `time` | `String` |
| `uri` | `String` |
| `url` | `String` |
| `code` | `String` (or enum if bound) |
| `HumanName` | `HumanName` |
| `Reference` | `Reference<T>` |

**Cardinality Mapping**:

| min | max | Rust Type |
|-----|-----|-----------|
| 0 | 1 | `Option<T>` |
| 0 | * | `Option<Vec<T>>` |
| 1 | 1 | `T` |
| 1 | * | `Vec<T>` |

**Responsibilities**:
- Map FHIR primitive types to Rust types
- Apply cardinality wrapping
- Detect and handle choice types  
- Extract enum variants from ValueSet bindings
- Convert FHIR names to Rust conventions
- Handle reserved keywords (`type` → `r#type`)

---

### Code Generator (`r4_generator/codegen.rs`)

**Purpose**: Generate Rust source code from mapped types

**Functions**:
```rust
pub fn generate_types(definitions: &[StructureDefinition]) -> anyhow::Result<String>;
pub fn generate_resources(definitions: &[StructureDefinition]) -> anyhow::Result<HashMap<String, String>>;
pub fn generate_struct(rust_struct: &RustStruct) -> String;
pub fn generate_enum(rust_enum: &RustEnum) -> String;
pub fn generate_field(field: &RustField) -> String;
```

**Templates**:

Struct template:
```rust
/// {documentation}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct {Name} {{
    {fields}
}}
```

Field template:
```rust
/// {documentation}
{serde_rename}
pub {name}: {rust_type},
```

Choice enum template:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum {Name}Choice {{
    {variants}
}}
```

Code enum template:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum {Name}Code {{
    {variants}
}}
```

**Responsibilities**:
- Generate struct definitions
- Generate enum definitions
- Generate field declarations
- Generate documentation comments
- Generate serde annotations
- Format code with proper indentation
- Handle nested BackboneElements

---

### File Writer (`r4_generator/writer.rs`)

**Purpose**: Write generated code to files and invoke rustfmt

**Functions**:
```rust
pub fn write_module(
    output_dir: &Path,
    type_code: String,
    resource_code: HashMap<String, String>,
) -> anyhow::Result<()>;

pub fn write_file(path: &Path, content: &str) -> anyhow::Result<()>;
pub fn format_code(dir: &Path) -> anyhow::Result<()>;
pub fn create_mod_file(dir: &Path, modules: &[String]) -> anyhow::Result<()>;
```

**File Organization**:
```
r4/
  mod.rs          # pub mod types; pub mod patient; pub mod observation; ...
  types.rs        # Complex types
  patient.rs      # Patient resource + nested types
  observation.rs  # Observation resource + nested types
  ...
```

**Mod File Generation**:
```rust
// Auto-generated module declarations
pub mod types;
pub mod account;
pub mod observation;
pub mod patient;
// ... alphabetically sorted
```

**Responsibilities**:
- Create output directory structure
- Write code to files
- Generate mod.rs with module declarations
- Invoke rustfmt on generated files
- Handle file I/O errors
- Ensure atomic writes (temp file → rename)

---

## Data Flow

```
JSON Schema
    ↓
[Parser]
    ↓
StructureDefinition
    ↓
[Type Mapper]
    ↓
RustStruct / RustEnum
    ↓
[Code Generator]
    ↓
String (Rust source code)
    ↓
[File Writer]
    ↓
.rs files
    ↓
[rustfmt]
    ↓
Formatted .rs files
```

## Error Handling

- **Parser errors**: JSON deserialization failures, missing required fields
- **Mapping errors**: Unknown FHIR types, invalid cardinality
- **Generation errors**: Template failures, invalid Rust syntax
- **I/O errors**: File write failures, directory creation failures
- **Formatting errors**: rustfmt not found or execution failure

All errors propagate using `anyhow::Result` with contextual information.

## Dependencies

- `serde` + `serde_json` - JSON parsing
- `anyhow` - Error handling
- `std::fs` - File I/O
- `std::process` - rustfmt invocation
- `std::path` - Path manipulation

## Testing Strategy

1. **Unit tests** - Test individual functions (type mapping, name conversion)
2. **Integration tests** - Test full pipeline with sample schemas
3. **Snapshot tests** - Compare generated code against known-good output
4. **Compilation tests** - Ensure generated code compiles
5. **Round-trip tests** - Serialize/deserialize FHIR resources
