//! FHIR R4B Complex Types
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use serde::{Deserialize, Serialize};

/// Base StructureDefinition for Element Type: Base definition for all elements in a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

/// Base StructureDefinition for BackboneElement Type: Base definition for all elements that are defined inside a resource - but not those in a data type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackboneElement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,
}

/// Base StructureDefinition for base64Binary Type: A stream of bytes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct base64Binary {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for base64Binary
    pub value: Option<String>,
}

/// Base StructureDefinition for boolean Type: Value of "true" or "false"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct boolean {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for boolean
    pub value: Option<bool>,
}

/// Base StructureDefinition for canonical type: A URI that is a reference to a canonical URL on a FHIR resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct canonical {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for canonical
    pub value: Option<String>,
}

/// Base StructureDefinition for code type: A string which has at least one character and no leading or trailing whitespace and where there is no whitespace other than single spaces in the contents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct code {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for code
    pub value: Option<String>,
}

/// Base StructureDefinition for date Type: A date or partial date (e.g. just year or year + month). There is no time zone. The format is a union of the schema types gYear, gYearMonth and date. Dates S...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct date {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for date
    pub value: Option<String>,
}

/// Base StructureDefinition for dateTime Type: A date, date-time or partial date (e.g. just year or year + month). If hours and minutes are specified, a time zone SHALL be populated. The format is a u...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct dateTime {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for dateTime
    pub value: Option<String>,
}

/// Base StructureDefinition for decimal Type: A rational number with implicit precision
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct decimal {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for decimal
    pub value: Option<f64>,
}

/// Base StructureDefinition for id type: Any combination of letters, numerals, "-" and ".", with a length limit of 64 characters. (This might be an integer, an unprefixed OID, UUID or any other identi...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct id {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for id
    pub value: Option<String>,
}

/// Base StructureDefinition for instant Type: An instant in time - known at least to the second
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct instant {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for instant
    pub value: Option<String>,
}

/// Base StructureDefinition for integer Type: A whole number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct integer {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for integer
    pub value: Option<i32>,
}

/// Base StructureDefinition for markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct markdown {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for markdown
    pub value: Option<String>,
}

/// Base StructureDefinition for oid type: An OID represented as a URI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct oid {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for oid
    pub value: Option<String>,
}

/// Base StructureDefinition for positiveInt type: An integer with a value that is positive (e.g. >0)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct positiveInt {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for positiveInt
    pub value: Option<String>,
}

/// Base StructureDefinition for string Type: A sequence of Unicode characters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct string {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for string
    pub value: Option<String>,
}

/// Base StructureDefinition for time Type: A time during the day, with no date specified
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct time {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for time
    pub value: Option<String>,
}

/// Base StructureDefinition for unsignedInt type: An integer with a value that is not negative (e.g. >= 0)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct unsignedInt {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for unsignedInt
    pub value: Option<String>,
}

/// Base StructureDefinition for uri Type: String of characters used to identify a name or a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct uri {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for uri
    pub value: Option<String>,
}

/// Base StructureDefinition for url type: A URI that is a literal reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct url {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for url
    pub value: Option<String>,
}

/// Base StructureDefinition for uuid type: A UUID, represented as a URI
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct uuid {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for uuid
    pub value: Option<String>,
}

/// Base StructureDefinition for xhtml Type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct xhtml {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Extension>,

    /// Actual xhtml
    pub value: String,
}

/// Base StructureDefinition for Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats). This data type may be used to convey addresses for...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// home | work | temp | old | billing - purpose of this address
    #[serde(rename = "use")]
    pub r#use: Option<String>,

    /// postal | physical | both
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Text representation of the address
    pub text: Option<String>,

    /// Street name, number, direction & P.O. Box etc.
    pub line: Option<Vec<String>>,

    /// Name of city, town etc.
    pub city: Option<String>,

    /// District name (aka county)
    pub district: Option<String>,

    /// Sub-unit of country (abbreviations ok)
    pub state: Option<String>,

    /// Postal code for area
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,

    /// Country (e.g. can be ISO 3166 2 or 3 letter code)
    pub country: Option<String>,

    /// Time period when address was/is in use
    pub period: Option<Period>,
}

/// Base StructureDefinition for Age Type: A duration of time during which an organism (or a process) has existed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Age {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Base StructureDefinition for Annotation Type: A text note which also contains information about who made the statement and when.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Individual responsible for the annotation
    pub author: Option<serde_json::Value>,

    /// When the annotation was made
    pub time: Option<String>,

    /// The annotation - text content (as markdown)
    pub text: String,
}

/// Base StructureDefinition for Attachment Type: For referring to data content defined in other formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Mime type of the content, with charset etc.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    /// Human language of the content (BCP-47)
    pub language: Option<String>,

    /// Data inline, base64ed
    pub data: Option<String>,

    /// Uri where the data can be found
    pub url: Option<String>,

    /// Number of bytes of content (if url provided)
    pub size: Option<u32>,

    /// Hash of the data (sha-1, base64ed)
    pub hash: Option<String>,

    /// Label to display in place of the data
    pub title: Option<String>,

    /// Date attachment was first created
    pub creation: Option<String>,
}

/// Base StructureDefinition for CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeableConcept {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Code defined by a terminology system
    pub coding: Option<Vec<Coding>>,

    /// Plain text representation of the concept
    pub text: Option<String>,
}

/// Base StructureDefinition for CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeableReference {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Reference to a concept (by class)
    pub concept: Option<CodeableConcept>,

    /// Reference to a resource (by instance)
    pub reference: Option<Box<Reference>>,
}

/// Base StructureDefinition for Coding Type: A reference to a code defined by a terminology system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coding {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Identity of the terminology system
    pub system: Option<String>,

    /// Version of the system - if relevant
    pub version: Option<String>,

    /// Symbol in syntax defined by the system
    pub code: Option<String>,

    /// Representation defined by the system
    pub display: Option<String>,

    /// If this coding was chosen directly by the user
    #[serde(rename = "userSelected")]
    pub user_selected: Option<bool>,
}

/// Base StructureDefinition for ContactDetail Type: Specifies contact information for a person or organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Name of an individual to contact
    pub name: Option<String>,

    /// Contact details for individual or organization
    pub telecom: Option<Vec<ContactPoint>>,
}

/// Base StructureDefinition for ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContactPoint {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// phone | fax | email | pager | url | sms | other
    pub system: Option<String>,

    /// The actual contact point details
    pub value: Option<String>,

    /// home | work | temp | old | mobile - purpose of this contact point
    #[serde(rename = "use")]
    pub r#use: Option<String>,

    /// Specify preferred order of use (1 = highest)
    pub rank: Option<i32>,

    /// Time period when the contact point was/is in use
    pub period: Option<Period>,
}

/// Base StructureDefinition for Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// author | editor | reviewer | endorser
    #[serde(rename = "type")]
    pub r#type: String,

    /// Who contributed the content
    pub name: String,

    /// Contact details of the contributor
    pub contact: Option<Vec<ContactDetail>>,
}

/// Base StructureDefinition for Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including a...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Base StructureDefinition for DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataRequirement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// The type of the required data
    #[serde(rename = "type")]
    pub r#type: String,

    /// The profile of the required data
    pub profile: Option<Vec<String>>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    pub subject: Option<serde_json::Value>,

    /// Indicates specific structure elements that are referenced by the knowledge module
    #[serde(rename = "mustSupport")]
    pub must_support: Option<Vec<String>>,

    /// What codes are expected
    #[serde(rename = "codeFilter")]
    pub code_filter: Option<Vec<Element>>,

    /// What dates/date ranges are expected
    #[serde(rename = "dateFilter")]
    pub date_filter: Option<Vec<Element>>,

    /// Number of results
    pub limit: Option<i32>,

    /// Order of the results
    pub sort: Option<Vec<Element>>,
}

/// Base StructureDefinition for Distance Type: A length - a value with a unit that is a physical distance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Base StructureDefinition for Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dosage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The order of the dosage instructions
    pub sequence: Option<i32>,

    /// Free text dosage instructions e.g. SIG
    pub text: Option<String>,

    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    #[serde(rename = "additionalInstruction")]
    pub additional_instruction: Option<Vec<CodeableConcept>>,

    /// Patient or consumer oriented instructions
    #[serde(rename = "patientInstruction")]
    pub patient_instruction: Option<String>,

    /// When medication should be administered
    pub timing: Option<Timing>,

    /// Take "as needed" (for x)
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<serde_json::Value>,

    /// Body site to administer to
    pub site: Option<CodeableConcept>,

    /// How drug should enter body
    pub route: Option<CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<CodeableConcept>,

    /// Amount of medication administered
    #[serde(rename = "doseAndRate")]
    pub dose_and_rate: Option<Vec<Element>>,

    /// Upper limit on medication per unit of time
    #[serde(rename = "maxDosePerPeriod")]
    pub max_dose_per_period: Option<Ratio>,

    /// Upper limit on medication per administration
    #[serde(rename = "maxDosePerAdministration")]
    pub max_dose_per_administration: Option<Quantity>,

    /// Upper limit on medication per lifetime of the patient
    #[serde(rename = "maxDosePerLifetime")]
    pub max_dose_per_lifetime: Option<Quantity>,
}

/// Base StructureDefinition for Duration Type: A length of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Base StructureDefinition for ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Path of the element in the hierarchy of elements
    pub path: String,

    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    pub representation: Option<Vec<String>>,

    /// Name for this particular element (in a set of slices)
    #[serde(rename = "sliceName")]
    pub slice_name: Option<String>,

    /// If this slice definition constrains an inherited slice definition (or not)
    #[serde(rename = "sliceIsConstraining")]
    pub slice_is_constraining: Option<bool>,

    /// Name for element to display with or prompt for element
    pub label: Option<String>,

    /// Corresponding codes in terminologies
    pub code: Option<Vec<Coding>>,

    /// This element is sliced - slices follow
    pub slicing: Option<Element>,

    /// Concise definition for space-constrained presentation
    pub short: Option<String>,

    /// Full formal definition as narrative text
    pub definition: Option<String>,

    /// Comments about the use of this element
    pub comment: Option<String>,

    /// Why this resource has been created
    pub requirements: Option<String>,

    /// Other names
    pub alias: Option<Vec<String>>,

    /// Minimum Cardinality
    pub min: Option<u32>,

    /// Maximum Cardinality (a number or *)
    pub max: Option<String>,

    /// Base definition information for tools
    pub base: Option<Element>,

    /// Reference to definition of content for the element
    #[serde(rename = "contentReference")]
    pub content_reference: Option<String>,

    /// Data type and Profile for this element
    #[serde(rename = "type")]
    pub r#type: Option<Vec<Element>>,

    /// Specified value if missing from instance
    #[serde(rename = "defaultValue")]
    pub default_value: Option<serde_json::Value>,

    /// Implicit meaning when this element is missing
    #[serde(rename = "meaningWhenMissing")]
    pub meaning_when_missing: Option<String>,

    /// What the order of the elements means
    #[serde(rename = "orderMeaning")]
    pub order_meaning: Option<String>,

    /// Value must be exactly this
    pub fixed: Option<serde_json::Value>,

    /// Value must have at least these property values
    pub pattern: Option<serde_json::Value>,

    /// Example value (as defined for type)
    pub example: Option<Vec<Element>>,

    /// Minimum Allowed Value (for some types)
    #[serde(rename = "minValue")]
    pub min_value: Option<serde_json::Value>,

    /// Maximum Allowed Value (for some types)
    #[serde(rename = "maxValue")]
    pub max_value: Option<serde_json::Value>,

    /// Max length for strings
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,

    /// Reference to invariant about presence
    pub condition: Option<Vec<String>>,

    /// Condition that must evaluate to true
    pub constraint: Option<Vec<Element>>,

    /// If the element must be supported
    #[serde(rename = "mustSupport")]
    pub must_support: Option<bool>,

    /// If this modifies the meaning of other elements
    #[serde(rename = "isModifier")]
    pub is_modifier: Option<bool>,

    /// Reason that this element is marked as a modifier
    #[serde(rename = "isModifierReason")]
    pub is_modifier_reason: Option<String>,

    /// Include when _summary = true?
    #[serde(rename = "isSummary")]
    pub is_summary: Option<bool>,

    /// ValueSet details if this is coded
    pub binding: Option<Element>,

    /// Map element to another set of definitions
    pub mapping: Option<Vec<Element>>,
}

/// Base StructureDefinition for Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the e...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expression {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Natural language description of the condition
    pub description: Option<String>,

    /// Short name assigned to expression for reuse
    pub name: Option<String>,

    /// text/cql | text/fhirpath | application/x-fhir-query | text/cql-identifier | text/cql-expression | etc.
    pub language: String,

    /// Expression in specified language
    pub expression: Option<String>,

    /// Where the expression is found
    pub reference: Option<String>,
}

/// Base StructureDefinition for Extension Type: Optional Extension Element - found in all resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extension {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// identifies the meaning of the extension
    pub url: String,

    /// Value of extension
    pub value: Option<serde_json::Value>,
}

/// Base StructureDefinition for HumanName Type: A human's name with the ability to identify parts and usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HumanName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// usual | official | temp | nickname | anonymous | old | maiden
    #[serde(rename = "use")]
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

/// Base StructureDefinition for Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// usual | official | temp | secondary | old (If known)
    #[serde(rename = "use")]
    pub r#use: Option<String>,

    /// Description of identifier
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The namespace for the identifier value
    pub system: Option<String>,

    /// The value that is unique
    pub value: Option<String>,

    /// Time period when id is/was valid for use
    pub period: Option<Period>,

    /// Organization that issued id (may be just text)
    pub assigner: Option<Box<Reference>>,
}

/// Base StructureDefinition for MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketingStatus {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The country in which the marketing authorisation has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements
    pub country: Option<CodeableConcept>,

    /// Where a Medicines Regulatory Agency has granted a marketing authorisation for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlle...
    pub jurisdiction: Option<CodeableConcept>,

    /// This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples
    pub status: CodeableConcept,

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provi...
    #[serde(rename = "dateRange")]
    pub date_range: Option<Period>,

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorisation Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provi...
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<String>,
}

/// Base StructureDefinition for Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associate...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Version specific identifier
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,

    /// When the resource version last changed
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,

    /// Identifies where the resource comes from
    pub source: Option<String>,

    /// Profiles this resource claims to conform to
    pub profile: Option<Vec<String>>,

    /// Security Labels applied to this resource
    pub security: Option<Vec<Coding>>,

    /// Tags applied to this resource
    pub tag: Option<Vec<Coding>>,
}

/// Base StructureDefinition for Money Type: An amount of economic utility in some recognized currency.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// ISO 4217 Currency Code
    pub currency: Option<String>,
}

/// Base StructureDefinition for Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Narrative {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// generated | extensions | additional | empty
    pub status: String,

    /// Limited xhtml content
    pub div: String,
}

/// Base StructureDefinition for ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as p...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Name used to access the parameter value
    pub name: Option<String>,

    /// in | out
    #[serde(rename = "use")]
    pub r#use: String,

    /// Minimum cardinality
    pub min: Option<i32>,

    /// Maximum cardinality (a number of *)
    pub max: Option<String>,

    /// A brief description of the parameter
    pub documentation: Option<String>,

    /// What type of value
    #[serde(rename = "type")]
    pub r#type: String,

    /// What profile the value is expected to be
    pub profile: Option<String>,
}

/// Base StructureDefinition for Period Type: A time period defined by a start and end date and optionally time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Starting time with inclusive boundary
    pub start: Option<String>,

    /// End time with inclusive boundary, if not ongoing
    pub end: Option<String>,
}

/// Base StructureDefinition for Population Type: A populatioof people with some set of grouping criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Population {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The age of the specific population
    pub age: Option<serde_json::Value>,

    /// The gender of the specific population
    pub gender: Option<CodeableConcept>,

    /// Race of the specific population
    pub race: Option<CodeableConcept>,

    /// The existing physiological conditions of the specific population to which this applies
    #[serde(rename = "physiologicalCondition")]
    pub physiological_condition: Option<CodeableConcept>,
}

/// Base StructureDefinition for ProdCharacteristic Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProdCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Where applicable, the height can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminolo...
    pub height: Option<Quantity>,

    /// Where applicable, the width can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminolog...
    pub width: Option<Quantity>,

    /// Where applicable, the depth can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminolog...
    pub depth: Option<Quantity>,

    /// Where applicable, the weight can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminolo...
    pub weight: Option<Quantity>,

    /// Where applicable, the nominal volume can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting t...
    #[serde(rename = "nominalVolume")]
    pub nominal_volume: Option<Quantity>,

    /// Where applicable, the external diameter can be specified using a numerical value and its unit of measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resultin...
    #[serde(rename = "externalDiameter")]
    pub external_diameter: Option<Quantity>,

    /// Where applicable, the shape can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub shape: Option<String>,

    /// Where applicable, the color can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub color: Option<Vec<String>>,

    /// Where applicable, the imprint can be specified as text
    pub imprint: Option<Vec<String>>,

    /// Where applicable, the image can be provided The format of the image attachment shall be specified by regional implementations
    pub image: Option<Vec<Attachment>>,

    /// Where applicable, the scoring can be specified An appropriate controlled vocabulary shall be used The term and the term identifier shall be used
    pub scoring: Option<CodeableConcept>,
}

/// Base StructureDefinition for ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductShelfLife {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Unique identifier for the packaged Medicinal Product
    pub identifier: Option<Box<Identifier>>,

    /// This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life afte...
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 a...
    pub period: Quantity,

    /// Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "specialPrecautionsForStorage")]
    pub special_precautions_for_storage: Option<Vec<CodeableConcept>>,
}

/// Base StructureDefinition for Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, includin...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Base StructureDefinition for Range Type: A set of ordered Quantities defined by a low and high limit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Low limit
    pub low: Option<Quantity>,

    /// High limit
    pub high: Option<Quantity>,
}

/// Base StructureDefinition for Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratio {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerator value
    pub numerator: Option<Quantity>,

    /// Denominator value
    pub denominator: Option<Quantity>,
}

/// Base StructureDefinition for RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatioRange {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Low Numerator limit
    #[serde(rename = "lowNumerator")]
    pub low_numerator: Option<Quantity>,

    /// High Numerator limit
    #[serde(rename = "highNumerator")]
    pub high_numerator: Option<Quantity>,

    /// Denominator value
    pub denominator: Option<Quantity>,
}

/// Base StructureDefinition for Reference Type: A reference from one resource to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<String>,

    /// Type the reference refers to (e.g. "Patient")
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Logical reference, when literal reference is not known
    pub identifier: Option<Box<Identifier>>,

    /// Text alternative for the resource
    pub display: Option<String>,
}

/// Base StructureDefinition for RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedArtifact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of
    #[serde(rename = "type")]
    pub r#type: String,

    /// Short label
    pub label: Option<String>,

    /// Brief description of the related artifact
    pub display: Option<String>,

    /// Bibliographic citation for the artifact
    pub citation: Option<String>,

    /// Where the artifact can be accessed
    pub url: Option<String>,

    /// What document is being referenced
    pub document: Option<Attachment>,

    /// What resource is being referenced
    pub resource: Option<String>,
}

/// Base StructureDefinition for SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledData {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Zero value and units
    pub origin: Quantity,

    /// Number of milliseconds between samples
    pub period: f64,

    /// Multiply data by this before adding to origin
    pub factor: Option<f64>,

    /// Lower limit of detection
    #[serde(rename = "lowerLimit")]
    pub lower_limit: Option<f64>,

    /// Upper limit of detection
    #[serde(rename = "upperLimit")]
    pub upper_limit: Option<f64>,

    /// Number of sample points at each time point
    pub dimensions: i32,

    /// Decimal values with spaces, or "E" | "U" | "L"
    pub data: Option<String>,
}

/// Base StructureDefinition for Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Indication of the reason the entity signed the object(s)
    #[serde(rename = "type")]
    pub r#type: Vec<Coding>,

    /// When the signature was created
    pub when: String,

    /// Who signed
    pub who: Box<Reference>,

    /// The party represented
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,

    /// The technical format of the signed resources
    #[serde(rename = "targetFormat")]
    pub target_format: Option<String>,

    /// The technical format of the signature
    #[serde(rename = "sigFormat")]
    pub sig_format: Option<String>,

    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    pub data: Option<String>,
}

/// Base StructureDefinition for Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most com...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// When the event occurs
    pub event: Option<Vec<String>>,

    /// When the event is to occur
    pub repeat: Option<Element>,

    /// BID | TID | QID | AM | PM | QD | QOD | +
    pub code: Option<CodeableConcept>,
}

/// Base StructureDefinition for TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    #[serde(rename = "type")]
    pub r#type: String,

    /// Name or URI that identifies the event
    pub name: Option<String>,

    /// Timing of the event
    pub timing: Option<serde_json::Value>,

    /// Triggering data of the event (multiple = 'and')
    pub data: Option<Vec<DataRequirement>>,

    /// Whether the event triggers (boolean expression)
    pub condition: Option<Expression>,
}

/// Base StructureDefinition for UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageContext {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Type of context being specified
    pub code: Coding,

    /// Value that defines the context
    pub value: serde_json::Value,
}

/// An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyQuantity {}

/// A fixed quantity (no comparator)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleQuantity {}
