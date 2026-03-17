//! FHIR R5 Complex Types
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

/// Element Type: Base definition for all elements in a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Element {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

/// BackboneElement Type: Base definition for all elements that are defined inside a resource - but not those in a data type.
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

/// base64Binary Type: A stream of bytes
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

/// boolean Type: Value of "true" or "false"
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

/// canonical type: A URI that is a reference to a canonical URL on a FHIR resource
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

/// code type: A string which has at least one character and no leading or trailing whitespace and where there is no whitespace other than single spaces in the contents
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

/// date Type: A date or partial date (e.g. just year or year + month). There is no UTC offset. The format is a union of the schema types gYear, gYearMonth and date. Dates SHALL be valid dates.
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

/// dateTime Type: A date, date-time or partial date (e.g. just year or year + month). If hours and minutes are specified, a UTC offset SHALL be populated. The format is a union of the schema types gYe...
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

/// decimal Type: A rational number with implicit precision
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

/// id type: Any combination of letters, numerals, "-" and ".", with a length limit of 64 characters. (This might be an integer, an unprefixed OID, UUID or any other identifier pattern that meets these...
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

/// instant Type: An instant in time - known at least to the second
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

/// integer Type: A whole number
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

/// integer64 Type: A very large whole number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct integer64 {
    /// xml:id (or equivalent in JSON)
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Primitive value for integer64
    pub value: Option<i32>,
}

/// markdown type: A string that may contain Github Flavored Markdown syntax for optional processing by a mark down presentation engine
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

/// oid type: An OID represented as a URI
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

/// positiveInt type: An integer with a value that is positive (e.g. >0)
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

/// string Type: A sequence of Unicode characters
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

/// time Type: A time during the day, with no date specified
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

/// unsignedInt type: An integer with a value that is not negative (e.g. >= 0)
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

/// uri Type: String of characters used to identify a name or a resource
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

/// url type: A URI that is a literal reference
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

/// uuid type: A UUID, represented as a URI
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

/// xhtml Type definition
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

/// Address Type: An address expressed using postal conventions (as opposed to GPS or other location definition formats). This data type may be used to convey addresses for use in delivering mail as we...
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

    /// Country (e.g. may be ISO 3166 2 or 3 letter code)
    pub country: Option<String>,

    /// Time period when address was/is in use
    pub period: Option<Period>,
}

/// Age Type: A duration of time during which an organism (or a process) has existed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Age {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Annotation Type: A text note which also contains information about who made the statement and when.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Individual responsible for the annotation
    #[serde(rename = "authorReference")]
    pub author_reference: Option<Box<Reference>>,

    #[serde(rename = "authorString")]
    pub author_string: Option<String>,

    /// When the annotation was made
    pub time: Option<String>,

    /// The annotation - text content (as markdown)
    pub text: String,
}

/// Attachment Type: For referring to data content defined in other formats.
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
    pub size: Option<i64>,

    /// Hash of the data (sha-1, base64ed)
    pub hash: Option<String>,

    /// Label to display in place of the data
    pub title: Option<String>,

    /// Date attachment was first created
    pub creation: Option<String>,

    /// Height of the image in pixels (photo/video)
    pub height: Option<i32>,

    /// Width of the image in pixels (photo/video)
    pub width: Option<i32>,

    /// Number of frames if > 1 (photo)
    pub frames: Option<i32>,

    /// Length in seconds (audio / video)
    pub duration: Option<f64>,

    /// Number of printed pages
    pub pages: Option<i32>,
}

/// Availability Type: Availability data for an {item}.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Times the {item} is available
    #[serde(rename = "availableTime")]
    pub available_time: Option<Vec<Element>>,

    /// Not available during this time due to provided reason
    #[serde(rename = "notAvailableTime")]
    pub not_available_time: Option<Vec<Element>>,
}

/// BackboneType Type: Base definition for the few data types that are allowed to carry modifier extensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackboneType {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,
}

/// Base Type: Base definition for all types defined in FHIR type system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {}

/// CodeableConcept Type: A concept that may be defined by a formal reference to a terminology or ontology or may be provided by text.
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

/// CodeableReference Type: A reference to a resource (by instance), or instead, a reference to a concept defined in a terminology or ontology (by class).
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

/// Coding Type: A reference to a code defined by a terminology system.
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

/// ContactDetail Type: Specifies contact information for a person or organization.
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

/// ContactPoint Type: Details for all kinds of technology mediated contact points for a person or organization, including telephone, email, etc.
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

/// Contributor Type: A contributor to the content of a knowledge asset, including authors, editors, reviewers, and endorsers.
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

/// Count Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary un...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// DataRequirement Type: Describes a required data item for evaluation in terms of the type of data, and optional code or date-based filters of the data.
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
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Box<Reference>>,

    /// Indicates specific structure elements that are referenced by the knowledge module
    #[serde(rename = "mustSupport")]
    pub must_support: Option<Vec<String>>,

    /// What codes are expected
    #[serde(rename = "codeFilter")]
    pub code_filter: Option<Vec<Element>>,

    /// What dates/date ranges are expected
    #[serde(rename = "dateFilter")]
    pub date_filter: Option<Vec<Element>>,

    /// What values are expected
    #[serde(rename = "valueFilter")]
    pub value_filter: Option<Vec<Element>>,

    /// Number of results
    pub limit: Option<i32>,

    /// Order of the results
    pub sort: Option<Vec<Element>>,
}

/// DataType Type: The base class for all re-useable types defined as part of the FHIR Specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataType {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

/// Distance Type: A length - a value with a unit that is a physical distance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Distance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Dosage Type: Indicates how the medication is/was taken or should be taken by the patient.
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

    /// Take "as needed"
    #[serde(rename = "asNeeded")]
    pub as_needed: Option<bool>,

    /// Take "as needed" (for x)
    #[serde(rename = "asNeededFor")]
    pub as_needed_for: Option<Vec<CodeableConcept>>,

    /// Body site to administer to
    pub site: Option<CodeableConcept>,

    /// How drug should enter body
    pub route: Option<CodeableConcept>,

    /// Technique for administering medication
    pub method: Option<CodeableConcept>,

    /// Amount of medication administered, to be administered or typical amount to be administered
    #[serde(rename = "doseAndRate")]
    pub dose_and_rate: Option<Vec<Element>>,

    /// Upper limit on medication per unit of time
    #[serde(rename = "maxDosePerPeriod")]
    pub max_dose_per_period: Option<Vec<Ratio>>,

    /// Upper limit on medication per administration
    #[serde(rename = "maxDosePerAdministration")]
    pub max_dose_per_administration: Option<Quantity>,

    /// Upper limit on medication per lifetime of the patient
    #[serde(rename = "maxDosePerLifetime")]
    pub max_dose_per_lifetime: Option<Quantity>,
}

/// Duration Type: A length of time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// ElementDefinition Type: Captures constraints on each element within the resource, profile, or extension.
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
    #[serde(rename = "defaultValueBase64Binary")]
    pub default_value_base64binary: Option<String>,

    #[serde(rename = "defaultValueBoolean")]
    pub default_value_boolean: Option<bool>,

    #[serde(rename = "defaultValueCanonical")]
    pub default_value_canonical: Option<String>,

    #[serde(rename = "defaultValueCode")]
    pub default_value_code: Option<String>,

    #[serde(rename = "defaultValueDate")]
    pub default_value_date: Option<String>,

    #[serde(rename = "defaultValueDateTime")]
    pub default_value_date_time: Option<String>,

    #[serde(rename = "defaultValueDecimal")]
    pub default_value_decimal: Option<f64>,

    #[serde(rename = "defaultValueId")]
    pub default_value_id: Option<String>,

    #[serde(rename = "defaultValueInstant")]
    pub default_value_instant: Option<String>,

    #[serde(rename = "defaultValueInteger")]
    pub default_value_integer: Option<i32>,

    #[serde(rename = "defaultValueInteger64")]
    pub default_value_integer64: Option<i64>,

    #[serde(rename = "defaultValueMarkdown")]
    pub default_value_markdown: Option<String>,

    #[serde(rename = "defaultValueOid")]
    pub default_value_oid: Option<String>,

    #[serde(rename = "defaultValuePositiveInt")]
    pub default_value_positive_int: Option<i32>,

    #[serde(rename = "defaultValueString")]
    pub default_value_string: Option<String>,

    #[serde(rename = "defaultValueTime")]
    pub default_value_time: Option<String>,

    #[serde(rename = "defaultValueUnsignedInt")]
    pub default_value_unsigned_int: Option<u32>,

    #[serde(rename = "defaultValueUri")]
    pub default_value_uri: Option<String>,

    #[serde(rename = "defaultValueUrl")]
    pub default_value_url: Option<String>,

    #[serde(rename = "defaultValueUuid")]
    pub default_value_uuid: Option<String>,

    #[serde(rename = "defaultValueAddress")]
    pub default_value_address: Option<Address>,

    #[serde(rename = "defaultValueAge")]
    pub default_value_age: Option<Age>,

    #[serde(rename = "defaultValueAnnotation")]
    pub default_value_annotation: Option<Annotation>,

    #[serde(rename = "defaultValueAttachment")]
    pub default_value_attachment: Option<Attachment>,

    #[serde(rename = "defaultValueCodeableConcept")]
    pub default_value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "defaultValueCodeableReference")]
    pub default_value_codeable_reference: Option<CodeableReference>,

    #[serde(rename = "defaultValueCoding")]
    pub default_value_coding: Option<Coding>,

    #[serde(rename = "defaultValueContactPoint")]
    pub default_value_contact_point: Option<ContactPoint>,

    #[serde(rename = "defaultValueCount")]
    pub default_value_count: Option<Count>,

    #[serde(rename = "defaultValueDistance")]
    pub default_value_distance: Option<Distance>,

    #[serde(rename = "defaultValueDuration")]
    pub default_value_duration: Option<Duration>,

    #[serde(rename = "defaultValueHumanName")]
    pub default_value_human_name: Option<HumanName>,

    #[serde(rename = "defaultValueIdentifier")]
    pub default_value_identifier: Option<Box<Identifier>>,

    #[serde(rename = "defaultValueMoney")]
    pub default_value_money: Option<Money>,

    #[serde(rename = "defaultValuePeriod")]
    pub default_value_period: Option<Period>,

    #[serde(rename = "defaultValueQuantity")]
    pub default_value_quantity: Option<Quantity>,

    #[serde(rename = "defaultValueRange")]
    pub default_value_range: Option<Range>,

    #[serde(rename = "defaultValueRatio")]
    pub default_value_ratio: Option<Ratio>,

    #[serde(rename = "defaultValueRatioRange")]
    pub default_value_ratio_range: Option<RatioRange>,

    #[serde(rename = "defaultValueReference")]
    pub default_value_reference: Option<Box<Reference>>,

    #[serde(rename = "defaultValueSampledData")]
    pub default_value_sampled_data: Option<SampledData>,

    #[serde(rename = "defaultValueSignature")]
    pub default_value_signature: Option<Signature>,

    #[serde(rename = "defaultValueTiming")]
    pub default_value_timing: Option<Timing>,

    #[serde(rename = "defaultValueContactDetail")]
    pub default_value_contact_detail: Option<ContactDetail>,

    #[serde(rename = "defaultValueDataRequirement")]
    pub default_value_data_requirement: Option<DataRequirement>,

    #[serde(rename = "defaultValueExpression")]
    pub default_value_expression: Option<Expression>,

    #[serde(rename = "defaultValueParameterDefinition")]
    pub default_value_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "defaultValueRelatedArtifact")]
    pub default_value_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "defaultValueTriggerDefinition")]
    pub default_value_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "defaultValueUsageContext")]
    pub default_value_usage_context: Option<UsageContext>,

    #[serde(rename = "defaultValueAvailability")]
    pub default_value_availability: Option<Availability>,

    #[serde(rename = "defaultValueExtendedContactDetail")]
    pub default_value_extended_contact_detail: Option<ExtendedContactDetail>,

    #[serde(rename = "defaultValueDosage")]
    pub default_value_dosage: Option<Dosage>,

    #[serde(rename = "defaultValueMeta")]
    pub default_value_meta: Option<Meta>,

    /// Implicit meaning when this element is missing
    #[serde(rename = "meaningWhenMissing")]
    pub meaning_when_missing: Option<String>,

    /// What the order of the elements means
    #[serde(rename = "orderMeaning")]
    pub order_meaning: Option<String>,

    /// Value must be exactly this
    #[serde(rename = "fixedBase64Binary")]
    pub fixed_base64binary: Option<String>,

    #[serde(rename = "fixedBoolean")]
    pub fixed_boolean: Option<bool>,

    #[serde(rename = "fixedCanonical")]
    pub fixed_canonical: Option<String>,

    #[serde(rename = "fixedCode")]
    pub fixed_code: Option<String>,

    #[serde(rename = "fixedDate")]
    pub fixed_date: Option<String>,

    #[serde(rename = "fixedDateTime")]
    pub fixed_date_time: Option<String>,

    #[serde(rename = "fixedDecimal")]
    pub fixed_decimal: Option<f64>,

    #[serde(rename = "fixedId")]
    pub fixed_id: Option<String>,

    #[serde(rename = "fixedInstant")]
    pub fixed_instant: Option<String>,

    #[serde(rename = "fixedInteger")]
    pub fixed_integer: Option<i32>,

    #[serde(rename = "fixedInteger64")]
    pub fixed_integer64: Option<i64>,

    #[serde(rename = "fixedMarkdown")]
    pub fixed_markdown: Option<String>,

    #[serde(rename = "fixedOid")]
    pub fixed_oid: Option<String>,

    #[serde(rename = "fixedPositiveInt")]
    pub fixed_positive_int: Option<i32>,

    #[serde(rename = "fixedString")]
    pub fixed_string: Option<String>,

    #[serde(rename = "fixedTime")]
    pub fixed_time: Option<String>,

    #[serde(rename = "fixedUnsignedInt")]
    pub fixed_unsigned_int: Option<u32>,

    #[serde(rename = "fixedUri")]
    pub fixed_uri: Option<String>,

    #[serde(rename = "fixedUrl")]
    pub fixed_url: Option<String>,

    #[serde(rename = "fixedUuid")]
    pub fixed_uuid: Option<String>,

    #[serde(rename = "fixedAddress")]
    pub fixed_address: Option<Address>,

    #[serde(rename = "fixedAge")]
    pub fixed_age: Option<Age>,

    #[serde(rename = "fixedAnnotation")]
    pub fixed_annotation: Option<Annotation>,

    #[serde(rename = "fixedAttachment")]
    pub fixed_attachment: Option<Attachment>,

    #[serde(rename = "fixedCodeableConcept")]
    pub fixed_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "fixedCodeableReference")]
    pub fixed_codeable_reference: Option<CodeableReference>,

    #[serde(rename = "fixedCoding")]
    pub fixed_coding: Option<Coding>,

    #[serde(rename = "fixedContactPoint")]
    pub fixed_contact_point: Option<ContactPoint>,

    #[serde(rename = "fixedCount")]
    pub fixed_count: Option<Count>,

    #[serde(rename = "fixedDistance")]
    pub fixed_distance: Option<Distance>,

    #[serde(rename = "fixedDuration")]
    pub fixed_duration: Option<Duration>,

    #[serde(rename = "fixedHumanName")]
    pub fixed_human_name: Option<HumanName>,

    #[serde(rename = "fixedIdentifier")]
    pub fixed_identifier: Option<Box<Identifier>>,

    #[serde(rename = "fixedMoney")]
    pub fixed_money: Option<Money>,

    #[serde(rename = "fixedPeriod")]
    pub fixed_period: Option<Period>,

    #[serde(rename = "fixedQuantity")]
    pub fixed_quantity: Option<Quantity>,

    #[serde(rename = "fixedRange")]
    pub fixed_range: Option<Range>,

    #[serde(rename = "fixedRatio")]
    pub fixed_ratio: Option<Ratio>,

    #[serde(rename = "fixedRatioRange")]
    pub fixed_ratio_range: Option<RatioRange>,

    #[serde(rename = "fixedReference")]
    pub fixed_reference: Option<Box<Reference>>,

    #[serde(rename = "fixedSampledData")]
    pub fixed_sampled_data: Option<SampledData>,

    #[serde(rename = "fixedSignature")]
    pub fixed_signature: Option<Signature>,

    #[serde(rename = "fixedTiming")]
    pub fixed_timing: Option<Timing>,

    #[serde(rename = "fixedContactDetail")]
    pub fixed_contact_detail: Option<ContactDetail>,

    #[serde(rename = "fixedDataRequirement")]
    pub fixed_data_requirement: Option<DataRequirement>,

    #[serde(rename = "fixedExpression")]
    pub fixed_expression: Option<Expression>,

    #[serde(rename = "fixedParameterDefinition")]
    pub fixed_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "fixedRelatedArtifact")]
    pub fixed_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "fixedTriggerDefinition")]
    pub fixed_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "fixedUsageContext")]
    pub fixed_usage_context: Option<UsageContext>,

    #[serde(rename = "fixedAvailability")]
    pub fixed_availability: Option<Availability>,

    #[serde(rename = "fixedExtendedContactDetail")]
    pub fixed_extended_contact_detail: Option<ExtendedContactDetail>,

    #[serde(rename = "fixedDosage")]
    pub fixed_dosage: Option<Dosage>,

    #[serde(rename = "fixedMeta")]
    pub fixed_meta: Option<Meta>,

    /// Value must have at least these property values
    #[serde(rename = "patternBase64Binary")]
    pub pattern_base64binary: Option<String>,

    #[serde(rename = "patternBoolean")]
    pub pattern_boolean: Option<bool>,

    #[serde(rename = "patternCanonical")]
    pub pattern_canonical: Option<String>,

    #[serde(rename = "patternCode")]
    pub pattern_code: Option<String>,

    #[serde(rename = "patternDate")]
    pub pattern_date: Option<String>,

    #[serde(rename = "patternDateTime")]
    pub pattern_date_time: Option<String>,

    #[serde(rename = "patternDecimal")]
    pub pattern_decimal: Option<f64>,

    #[serde(rename = "patternId")]
    pub pattern_id: Option<String>,

    #[serde(rename = "patternInstant")]
    pub pattern_instant: Option<String>,

    #[serde(rename = "patternInteger")]
    pub pattern_integer: Option<i32>,

    #[serde(rename = "patternInteger64")]
    pub pattern_integer64: Option<i64>,

    #[serde(rename = "patternMarkdown")]
    pub pattern_markdown: Option<String>,

    #[serde(rename = "patternOid")]
    pub pattern_oid: Option<String>,

    #[serde(rename = "patternPositiveInt")]
    pub pattern_positive_int: Option<i32>,

    #[serde(rename = "patternString")]
    pub pattern_string: Option<String>,

    #[serde(rename = "patternTime")]
    pub pattern_time: Option<String>,

    #[serde(rename = "patternUnsignedInt")]
    pub pattern_unsigned_int: Option<u32>,

    #[serde(rename = "patternUri")]
    pub pattern_uri: Option<String>,

    #[serde(rename = "patternUrl")]
    pub pattern_url: Option<String>,

    #[serde(rename = "patternUuid")]
    pub pattern_uuid: Option<String>,

    #[serde(rename = "patternAddress")]
    pub pattern_address: Option<Address>,

    #[serde(rename = "patternAge")]
    pub pattern_age: Option<Age>,

    #[serde(rename = "patternAnnotation")]
    pub pattern_annotation: Option<Annotation>,

    #[serde(rename = "patternAttachment")]
    pub pattern_attachment: Option<Attachment>,

    #[serde(rename = "patternCodeableConcept")]
    pub pattern_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "patternCodeableReference")]
    pub pattern_codeable_reference: Option<CodeableReference>,

    #[serde(rename = "patternCoding")]
    pub pattern_coding: Option<Coding>,

    #[serde(rename = "patternContactPoint")]
    pub pattern_contact_point: Option<ContactPoint>,

    #[serde(rename = "patternCount")]
    pub pattern_count: Option<Count>,

    #[serde(rename = "patternDistance")]
    pub pattern_distance: Option<Distance>,

    #[serde(rename = "patternDuration")]
    pub pattern_duration: Option<Duration>,

    #[serde(rename = "patternHumanName")]
    pub pattern_human_name: Option<HumanName>,

    #[serde(rename = "patternIdentifier")]
    pub pattern_identifier: Option<Box<Identifier>>,

    #[serde(rename = "patternMoney")]
    pub pattern_money: Option<Money>,

    #[serde(rename = "patternPeriod")]
    pub pattern_period: Option<Period>,

    #[serde(rename = "patternQuantity")]
    pub pattern_quantity: Option<Quantity>,

    #[serde(rename = "patternRange")]
    pub pattern_range: Option<Range>,

    #[serde(rename = "patternRatio")]
    pub pattern_ratio: Option<Ratio>,

    #[serde(rename = "patternRatioRange")]
    pub pattern_ratio_range: Option<RatioRange>,

    #[serde(rename = "patternReference")]
    pub pattern_reference: Option<Box<Reference>>,

    #[serde(rename = "patternSampledData")]
    pub pattern_sampled_data: Option<SampledData>,

    #[serde(rename = "patternSignature")]
    pub pattern_signature: Option<Signature>,

    #[serde(rename = "patternTiming")]
    pub pattern_timing: Option<Timing>,

    #[serde(rename = "patternContactDetail")]
    pub pattern_contact_detail: Option<ContactDetail>,

    #[serde(rename = "patternDataRequirement")]
    pub pattern_data_requirement: Option<DataRequirement>,

    #[serde(rename = "patternExpression")]
    pub pattern_expression: Option<Expression>,

    #[serde(rename = "patternParameterDefinition")]
    pub pattern_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "patternRelatedArtifact")]
    pub pattern_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "patternTriggerDefinition")]
    pub pattern_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "patternUsageContext")]
    pub pattern_usage_context: Option<UsageContext>,

    #[serde(rename = "patternAvailability")]
    pub pattern_availability: Option<Availability>,

    #[serde(rename = "patternExtendedContactDetail")]
    pub pattern_extended_contact_detail: Option<ExtendedContactDetail>,

    #[serde(rename = "patternDosage")]
    pub pattern_dosage: Option<Dosage>,

    #[serde(rename = "patternMeta")]
    pub pattern_meta: Option<Meta>,

    /// Example value (as defined for type)
    pub example: Option<Vec<Element>>,

    /// Minimum Allowed Value (for some types)
    #[serde(rename = "minValueDate")]
    pub min_value_date: Option<String>,

    #[serde(rename = "minValueDateTime")]
    pub min_value_date_time: Option<String>,

    #[serde(rename = "minValueInstant")]
    pub min_value_instant: Option<String>,

    #[serde(rename = "minValueTime")]
    pub min_value_time: Option<String>,

    #[serde(rename = "minValueDecimal")]
    pub min_value_decimal: Option<f64>,

    #[serde(rename = "minValueInteger")]
    pub min_value_integer: Option<i32>,

    #[serde(rename = "minValueInteger64")]
    pub min_value_integer64: Option<i64>,

    #[serde(rename = "minValuePositiveInt")]
    pub min_value_positive_int: Option<i32>,

    #[serde(rename = "minValueUnsignedInt")]
    pub min_value_unsigned_int: Option<u32>,

    #[serde(rename = "minValueQuantity")]
    pub min_value_quantity: Option<Quantity>,

    /// Maximum Allowed Value (for some types)
    #[serde(rename = "maxValueDate")]
    pub max_value_date: Option<String>,

    #[serde(rename = "maxValueDateTime")]
    pub max_value_date_time: Option<String>,

    #[serde(rename = "maxValueInstant")]
    pub max_value_instant: Option<String>,

    #[serde(rename = "maxValueTime")]
    pub max_value_time: Option<String>,

    #[serde(rename = "maxValueDecimal")]
    pub max_value_decimal: Option<f64>,

    #[serde(rename = "maxValueInteger")]
    pub max_value_integer: Option<i32>,

    #[serde(rename = "maxValueInteger64")]
    pub max_value_integer64: Option<i64>,

    #[serde(rename = "maxValuePositiveInt")]
    pub max_value_positive_int: Option<i32>,

    #[serde(rename = "maxValueUnsignedInt")]
    pub max_value_unsigned_int: Option<u32>,

    #[serde(rename = "maxValueQuantity")]
    pub max_value_quantity: Option<Quantity>,

    /// Max length for string type data
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,

    /// Reference to invariant about presence
    pub condition: Option<Vec<String>>,

    /// Condition that must evaluate to true
    pub constraint: Option<Vec<Element>>,

    /// For primitives, that a value must be present - not replaced by an extension
    #[serde(rename = "mustHaveValue")]
    pub must_have_value: Option<bool>,

    /// Extensions that are allowed to replace a primitive value
    #[serde(rename = "valueAlternatives")]
    pub value_alternatives: Option<Vec<String>>,

    /// If the element must be supported (discouraged - see obligations)
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

/// Expression Type: A expression that is evaluated in a specified context and returns a value. The context of use of the expression must specify the context in which the expression is evaluated, and h...
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

    /// text/cql | text/fhirpath | application/x-fhir-query | etc.
    pub language: Option<String>,

    /// Expression in specified language
    pub expression: Option<String>,

    /// Where the expression is found
    pub reference: Option<String>,
}

/// ExtendedContactDetail Type: Specifies contact information for a specific purpose over a period of time, might be handled/monitored by a specific named person or organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtendedContactDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// The type of contact
    pub purpose: Option<CodeableConcept>,

    /// Name of an individual to contact
    pub name: Option<Vec<HumanName>>,

    /// Contact details (e.g.phone/fax/url)
    pub telecom: Option<Vec<ContactPoint>>,

    /// Address for the contact
    pub address: Option<Address>,

    /// This contact detail is handled/monitored by a specific organization
    pub organization: Option<Box<Reference>>,

    /// Period that this contact was valid for usage
    pub period: Option<Period>,
}

/// Extension Type: Optional Extension Element - found in all resources.
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
    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueCanonical")]
    pub value_canonical: Option<String>,

    #[serde(rename = "valueCode")]
    pub value_code: Option<String>,

    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<String>,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<f64>,

    #[serde(rename = "valueId")]
    pub value_id: Option<String>,

    #[serde(rename = "valueInstant")]
    pub value_instant: Option<String>,

    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i32>,

    #[serde(rename = "valueInteger64")]
    pub value_integer64: Option<i64>,

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<String>,

    #[serde(rename = "valueOid")]
    pub value_oid: Option<String>,

    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: Option<i32>,

    #[serde(rename = "valueString")]
    pub value_string: Option<String>,

    #[serde(rename = "valueTime")]
    pub value_time: Option<String>,

    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: Option<u32>,

    #[serde(rename = "valueUri")]
    pub value_uri: Option<String>,

    #[serde(rename = "valueUrl")]
    pub value_url: Option<String>,

    #[serde(rename = "valueUuid")]
    pub value_uuid: Option<String>,

    #[serde(rename = "valueAddress")]
    pub value_address: Option<Address>,

    #[serde(rename = "valueAge")]
    pub value_age: Option<Age>,

    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Option<Annotation>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueCodeableReference")]
    pub value_codeable_reference: Option<CodeableReference>,

    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,

    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: Option<ContactPoint>,

    #[serde(rename = "valueCount")]
    pub value_count: Option<Count>,

    #[serde(rename = "valueDistance")]
    pub value_distance: Option<Distance>,

    #[serde(rename = "valueDuration")]
    pub value_duration: Option<Duration>,

    #[serde(rename = "valueHumanName")]
    pub value_human_name: Option<HumanName>,

    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Option<Box<Identifier>>,

    #[serde(rename = "valueMoney")]
    pub value_money: Option<Money>,

    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,

    #[serde(rename = "valueRatioRange")]
    pub value_ratio_range: Option<RatioRange>,

    #[serde(rename = "valueReference")]
    pub value_reference: Option<Box<Reference>>,

    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,

    #[serde(rename = "valueSignature")]
    pub value_signature: Option<Signature>,

    #[serde(rename = "valueTiming")]
    pub value_timing: Option<Timing>,

    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: Option<ContactDetail>,

    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: Option<DataRequirement>,

    #[serde(rename = "valueExpression")]
    pub value_expression: Option<Expression>,

    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: Option<UsageContext>,

    #[serde(rename = "valueAvailability")]
    pub value_availability: Option<Availability>,

    #[serde(rename = "valueExtendedContactDetail")]
    pub value_extended_contact_detail: Option<ExtendedContactDetail>,

    #[serde(rename = "valueDosage")]
    pub value_dosage: Option<Dosage>,

    #[serde(rename = "valueMeta")]
    pub value_meta: Option<Meta>,
}

/// HumanName Type: A name, normally of a human, that can be used for other living entities (e.g. animals but not organizations) that have been assigned names by a human and may need the use of name pa...
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

/// Identifier Type: An identifier - identifies some entity uniquely and unambiguously. Typically this is used for business identifiers.
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

/// MarketingStatus Type: The marketing status describes the date when a medicinal product is actually put on the market or the date as of which it is no longer available.
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

    /// The country in which the marketing authorization has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements
    pub country: Option<CodeableConcept>,

    /// Where a Medicines Regulatory Agency has granted a marketing authorization for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlle...
    pub jurisdiction: Option<CodeableConcept>,

    /// This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples
    pub status: CodeableConcept,

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provi...
    #[serde(rename = "dateRange")]
    pub date_range: Option<Period>,

    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provi...
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<String>,
}

/// Meta Type: The metadata about a resource. This is content in the resource that is maintained by the infrastructure. Changes to the content might not always be associated with version changes to the...
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

/// MonetaryComponent Type: Availability data for an {item}.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonetaryComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// base | surcharge | deduction | discount | tax | informational
    #[serde(rename = "type")]
    pub r#type: String,

    /// Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.
    pub code: Option<CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<f64>,

    /// Explicit value amount to be used
    pub amount: Option<Money>,
}

/// Money Type: An amount of economic utility in some recognized currency.
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

/// Narrative Type: A human-readable summary of the resource conveying the essential clinical and business information for the resource.
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

/// ParameterDefinition Type: The parameters to the module. This collection specifies both the input and output parameters. Input parameters are provided by the caller as part of the $evaluate operatio...
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

/// Period Type: A time period defined by a start and end date and optionally time.
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

/// PrimitiveType Type: The base type for all re-useable types defined that have a simple property.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrimitiveType {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,
}

/// ProductShelfLife Type: The shelf-life and storage information for a medicinal product item or container can be described using this class.
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

    /// This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life afte...
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 a...
    #[serde(rename = "periodDuration")]
    pub period_duration: Option<Duration>,

    #[serde(rename = "periodString")]
    pub period_string: Option<String>,

    /// Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[serde(rename = "specialPrecautionsForStorage")]
    pub special_precautions_for_storage: Option<Vec<CodeableConcept>>,
}

/// Quantity Type: A measured amount (or an amount that can potentially be measured). Note that measured amounts include amounts that are not precisely quantified, including amounts involving arbitrary...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quantity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Numerical value (with implicit precision)
    pub value: Option<f64>,

    /// < | <= | >= | > | ad - how to understand the value
    pub comparator: Option<String>,

    /// Unit representation
    pub unit: Option<String>,

    /// System that defines coded unit form
    pub system: Option<String>,

    /// Coded form of the unit
    pub code: Option<String>,
}

/// Range Type: A set of ordered Quantities defined by a low and high limit.
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

/// Ratio Type: A relationship of two Quantity values - expressed as a numerator and a denominator.
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

/// RatioRange Type: A range of ratios expressed as a low and high numerator and a denominator.
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

/// Reference Type: A reference from one resource to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Literal reference, Relative, internal or absolute URL
    pub reference: Option<String>,

    /// Type the reference refers to (e.g. "Patient") - must be a resource in resources
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Logical reference, when literal reference is not known
    pub identifier: Option<Box<Identifier>>,

    /// Text alternative for the resource
    pub display: Option<String>,
}

/// RelatedArtifact Type: Related artifacts such as additional documentation, justification, or bibliographic references.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedArtifact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comment...
    #[serde(rename = "type")]
    pub r#type: String,

    /// Additional classifiers
    pub classifier: Option<Vec<CodeableConcept>>,

    /// Short label
    pub label: Option<String>,

    /// Brief description of the related artifact
    pub display: Option<String>,

    /// Bibliographic citation for the artifact
    pub citation: Option<String>,

    /// What document is being referenced
    pub document: Option<Attachment>,

    /// What artifact is being referenced
    pub resource: Option<String>,

    /// What artifact, if not a conformance resource
    #[serde(rename = "resourceReference")]
    pub resource_reference: Option<Box<Reference>>,

    /// draft | active | retired | unknown
    #[serde(rename = "publicationStatus")]
    pub publication_status: Option<String>,

    /// Date of publication of the artifact being referred to
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,
}

/// SampledData Type: A series of measurements taken by a device, with upper and lower limits. There may be more than one dimension in the data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledData {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Zero value and units
    pub origin: Quantity,

    /// Number of intervalUnits between samples
    pub interval: Option<f64>,

    /// The measurement unit of the interval between samples
    #[serde(rename = "intervalUnit")]
    pub interval_unit: String,

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

    /// Defines the codes used in the data
    #[serde(rename = "codeMap")]
    pub code_map: Option<String>,

    /// Offsets, typically in time, at which data values were taken
    pub offsets: Option<String>,

    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    pub data: Option<String>,
}

/// Signature Type: A signature along with supporting context. The signature may be a digital signature that is cryptographic in nature, or some other signature acceptable to the domain. This other sig...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Indication of the reason the entity signed the object(s)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<Coding>>,

    /// When the signature was created
    pub when: Option<String>,

    /// Who signed
    pub who: Option<Box<Reference>>,

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

/// Timing Type: Specifies an event that may occur multiple times. Timing schedules are used to record when things are planned, expected or requested to occur. The most common usage is in dosage instru...
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

    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    pub code: Option<CodeableConcept>,
}

/// TriggerDefinition Type: A description of a triggering event. Triggering events can be named events, data events, or periodic, as determined by the type element.
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

    /// Coded definition of the event
    pub code: Option<CodeableConcept>,

    /// What event
    #[serde(rename = "subscriptionTopic")]
    pub subscription_topic: Option<String>,

    /// Timing of the event
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,

    #[serde(rename = "timingReference")]
    pub timing_reference: Option<Box<Reference>>,

    #[serde(rename = "timingDate")]
    pub timing_date: Option<String>,

    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<String>,

    /// Triggering data of the event (multiple = 'and')
    pub data: Option<Vec<DataRequirement>>,

    /// Whether the event triggers (boolean expression)
    pub condition: Option<Expression>,
}

/// UsageContext Type: Specifies clinical/business/etc. metadata that can be used to retrieve, index and/or categorize an artifact. This metadata can either be specific to the applicable population (e....
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
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueReference")]
    pub value_reference: Box<Reference>,
}

/// VirtualServiceDetail Type: Virtual Service Contact Details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VirtualServiceDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Channel Type
    #[serde(rename = "channelType")]
    pub channel_type: Option<Coding>,

    /// Contact address/number
    #[serde(rename = "addressUrl")]
    pub address_url: Option<String>,

    #[serde(rename = "addressString")]
    pub address_string: Option<String>,

    #[serde(rename = "addressContactPoint")]
    pub address_contact_point: Option<ContactPoint>,

    #[serde(rename = "addressExtendedContactDetail")]
    pub address_extended_contact_detail: Option<ExtendedContactDetail>,

    /// Address to see alternative connection details
    #[serde(rename = "additionalInfo")]
    pub additional_info: Option<Vec<String>>,

    /// Maximum number of participants supported by the virtual service
    #[serde(rename = "maxParticipants")]
    pub max_participants: Option<i32>,

    /// Session Key required by the virtual service
    #[serde(rename = "sessionKey")]
    pub session_key: Option<String>,
}

/// An amount of money. With regard to precision, see [Decimal Precision](datatypes.html#precision)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoneyQuantity {}

/// A fixed quantity (no comparator)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleQuantity {}
