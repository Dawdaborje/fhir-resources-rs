//! FHIR R4 CodeSystem Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Filter that can be used in a value set
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemFilter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that identifies the filter
    pub code: String,

    /// How or why the filter is used
    pub description: Option<String>,

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | exists
    pub operator: Vec<String>,

    /// What to use for the value
    pub value: String,
}

/// Additional information supplied about each concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies the property on the concepts, and when referred to in operations
    pub code: String,

    /// Formal identifier for the property
    pub uri: Option<String>,

    /// Why the property is defined, and/or what it conveys
    pub description: Option<String>,

    /// code | Coding | string | integer | boolean | dateTime | decimal
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Concepts in the code system
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConcept {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that identifies concept
    pub code: String,

    /// Text to display to the user
    pub display: Option<String>,

    /// Formal definition
    pub definition: Option<String>,

    /// Additional representations for the concept
    pub designation: Option<Vec<CodeSystemConceptDesignation>>,

    /// Property value for the concept
    pub property: Option<Vec<CodeSystemConceptProperty>>,

    /// Child Concepts (is-a/contains/categorizes)
    pub concept: Option<Vec<CodeSystemConcept>>,
}

/// Additional representations for the concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConceptDesignation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Human language of the designation
    pub language: Option<String>,

    /// Details how this designation would be used
    #[serde(rename = "use")]
    pub r#use: Option<Coding>,

    /// The text value for this designation
    pub value: String,
}

/// Property value for the concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystemConceptProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to CodeSystem.property.code
    pub code: String,

    /// Value of the property for this concept
    pub value: serde_json::Value,
}

/// The CodeSystem resource is used to declare the existence of and describe a code system or code system supplement and its key properties, and optionally define a part or all of its content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeSystem {
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

    /// Canonical identifier for this code system, represented as a URI (globally unique) (Coding.system)
    pub url: Option<String>,

    /// Additional identifier for the code system (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the code system (Coding.version)
    pub version: Option<String>,

    /// Name for this code system (computer friendly)
    pub name: Option<String>,

    /// Name for this code system (human friendly)
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

    /// Natural language description of the code system
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for code system (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this code system is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// If code comparison is case sensitive
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,

    /// Canonical reference to the value set with entire code system
    #[serde(rename = "valueSet")]
    pub value_set: Option<String>,

    /// grouped-by | is-a | part-of | classified-with
    #[serde(rename = "hierarchyMeaning")]
    pub hierarchy_meaning: Option<String>,

    /// If code system defines a compositional grammar
    pub compositional: Option<bool>,

    /// If definitions are not stable
    #[serde(rename = "versionNeeded")]
    pub version_needed: Option<bool>,

    /// not-present | example | fragment | complete | supplement
    pub content: String,

    /// Canonical URL of Code System this adds designations and properties to
    pub supplements: Option<String>,

    /// Total concepts in the code system
    pub count: Option<u32>,

    /// Filter that can be used in a value set
    pub filter: Option<Vec<CodeSystemFilter>>,

    /// Additional information supplied about each concept
    pub property: Option<Vec<CodeSystemProperty>>,

    /// Concepts in the code system
    pub concept: Option<Vec<CodeSystemConcept>>,
}
