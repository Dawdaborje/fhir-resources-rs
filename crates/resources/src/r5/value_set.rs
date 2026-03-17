//! FHIR R5 ValueSet Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// Property to return if client doesn't override
    pub property: Option<Vec<String>>,
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

    /// A copyright statement for the specific code system included in the value set
    pub copyright: Option<String>,
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

    /// Additional ways how this designation would be used
    #[serde(rename = "additionalUse")]
    pub additional_use: Option<Vec<Coding>>,

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

    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
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

    /// Opaque urls for paging through expansion results
    pub next: Option<String>,

    /// Time ValueSet expansion happened
    pub timestamp: String,

    /// Total number of codes in the expansion
    pub total: Option<i32>,

    /// Offset at which this resource starts
    pub offset: Option<i32>,

    /// Parameter that controlled the expansion process
    pub parameter: Option<Vec<ValueSetExpansionParameter>>,

    /// Additional information supplied about each concept
    pub property: Option<Vec<ValueSetExpansionProperty>>,

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
    pub value: Option<serde_json::Value>,
}

/// Additional information supplied about each concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionProperty {
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

    /// Property value for the concept
    pub property: Option<Vec<ValueSetExpansionContainsProperty>>,

    /// Codes contained under this entry
    pub contains: Option<Vec<ValueSetExpansionContains>>,
}

/// Property value for the concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to ValueSet.expansion.property.code
    pub code: String,

    /// Value of the property for this concept
    pub value: serde_json::Value,

    /// SubProperty value for the concept
    #[serde(rename = "subProperty")]
    pub sub_property: Option<Vec<ValueSetExpansionContainsPropertySubProperty>>,
}

/// SubProperty value for the concept
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetExpansionContainsPropertySubProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to ValueSet.expansion.property.code
    pub code: String,

    /// Value of the subproperty for this concept
    pub value: serde_json::Value,
}

/// Description of the semantic space the Value Set Expansion is intended to cover and should further clarify the text in ValueSet.description
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueSetScope {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Criteria describing which concepts or codes should be included and why
    #[serde(rename = "inclusionCriteria")]
    pub inclusion_criteria: Option<String>,

    /// Criteria describing which concepts or codes should be excluded and why
    #[serde(rename = "exclusionCriteria")]
    pub exclusion_criteria: Option<String>,
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

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

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

    /// Name of the publisher/steward (organization or individual)
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

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the ValueSet was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the ValueSet was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the ValueSet is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the ValueSet
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the ValueSet
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the ValueSet
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the ValueSet
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Content logical definition of the value set (CLD)
    pub compose: Option<ValueSetCompose>,

    /// Used when the value set is "expanded"
    pub expansion: Option<ValueSetExpansion>,

    /// Description of the semantic space the Value Set Expansion is intended to cover and should further clarify the text in ValueSet.description
    pub scope: Option<ValueSetScope>,
}
