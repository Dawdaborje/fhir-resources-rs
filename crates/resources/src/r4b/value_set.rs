//! FHIR R4B ValueSet Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Content logical definition of the value set (CLD)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetCompose {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Fixed date for references with no specified version (transitive)
    #[serde(rename = "lockedDate")]
    pub locked_date: Option<String>,

    /// Whether inactive codes are in the value set
    pub inactive: Option<bool>,

    /// Include one or more codes from a code system or other value set(s)
    pub include: Vec<ValueSetComposeInclude>,

    /// Explicitly exclude codes from a code system or other value sets
    pub exclude: Option<Vec<ValueSetComposeInclude>>,
}

/// Include one or more codes from a code system or other value set(s)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeInclude {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The system the codes come from
    pub system: Option<String>,

    /// Specific version of the code system referred to
    pub version: Option<String>,

    /// A concept defined in the system
    pub concept: Option<Vec<ValueSetComposeIncludeConcept>>,

    /// Select codes/concepts by their properties (including relationships)
    pub filter: Option<Vec<ValueSetComposeIncludeFilter>>,

    /// Select the contents included in this value set
    #[serde(rename = "valueSet")]
    pub value_set: Option<Vec<String>>,
}

/// A concept defined in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeConcept {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code or expression from system
    pub code: String,

    /// Text to display for this code for this value set in this valueset
    pub display: Option<String>,

    /// Additional representations for this concept
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignation>>,
}

/// Additional representations for this concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Human language of the designation
    pub language: Option<String>,

    /// Types of uses of designations
    #[serde(rename = "use")]
    pub r#use: Option<Coding>,

    /// The text value for this designation
    pub value: String,
}

/// Select codes/concepts by their properties (including relationships)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetComposeIncludeFilter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A property/filter defined by the code system
    pub property: String,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | exists
    pub op: String,

    /// Code from the system, or regex criteria, or boolean value for exists
    pub value: String,
}

/// Used when the value set is "expanded"
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies the value set expansion (business identifier)
    pub identifier: Option<String>,

    /// Time ValueSet expansion happened
    pub timestamp: String,

    /// Total number of codes in the expansion
    pub total: Option<i32>,

    /// Offset at which this resource starts
    pub offset: Option<i32>,

    /// Parameter that controlled the expansion process
    pub parameter: Option<Vec<ValueSetExpansionParameter>>,

    /// Codes in the value set
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// Parameter that controlled the expansion process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name as assigned by the client or server
    pub name: String,

    /// Value of the named parameter
    #[serde(rename = "valueString")]
    pub value_string: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i32>,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<f64>,

    #[serde(rename = "valueUri")]
    pub value_uri: Option<String>,

    #[serde(rename = "valueCode")]
    pub value_code: Option<String>,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<String>,
}

/// Codes in the value set
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContains {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// System value for the code
    pub system: Option<String>,

    /// If user cannot select this entry
    #[serde(rename = "abstract")]
    pub r#abstract: Option<bool>,

    /// If concept is inactive in the code system
    pub inactive: Option<bool>,

    /// Version in which this code/display is defined
    pub version: Option<String>,

    /// Code - if blank, this is not a selectable code
    pub code: Option<String>,

    /// User display for the concept
    pub display: Option<String>,

    /// Additional representations for this item
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignation>>,

    /// Codes contained under this entry
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// A ValueSet resource instance specifies a set of codes drawn from one or more code systems, intended for use in a particular context. Value sets link between [[[CodeSystem]]] definitions and their u...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSet {
    /// The type of resource
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// Logical id of this artifact
    pub id: Option<String>,

    /// Metadata about the resource
    pub meta: Option<Meta>,

    /// A set of rules under which this content was created
    #[serde(rename = "implicitRules")]
    pub implicit_rules: Option<String>,

    /// Language of the resource content
    pub language: Option<String>,

    /// Text summary of the resource, for human interpretation
    pub text: Option<Narrative>,

    /// Contained, inline Resources
    pub contained: Option<Vec<serde_json::Value>>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Canonical identifier for this value set, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the value set (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the value set
    pub version: Option<String>,

    /// Name for this value set (computer friendly)
    pub name: Option<String>,

    /// Name for this value set (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the value set
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for value set (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Indicates whether or not any change to the content logical definition may occur
    pub immutable: Option<bool>,

    /// Why this value set is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Content logical definition of the value set (CLD)
    pub compose: Option<ValueSetCompose>,

    /// Used when the value set is "expanded"
    pub expansion: Option<ValueSetExpansion>,
}
