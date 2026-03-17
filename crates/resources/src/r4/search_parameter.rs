//! FHIR R4 SearchParameter Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// For Composite resources to define the parts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameterComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Defines how the part works
    pub definition: String,

    /// Subexpression relative to main expression
    pub expression: String,
}

/// A search parameter that defines a named search item that can be used to search/filter on a resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParameter {
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

    /// Canonical identifier for this search parameter, represented as a URI (globally unique)
    pub url: String,

    /// Business version of the search parameter
    pub version: Option<String>,

    /// Name for this search parameter (computer friendly)
    pub name: String,

    /// Original definition for the search parameter
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<String>,

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

    /// Natural language description of the search parameter
    pub description: String,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for search parameter (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this search parameter is defined
    pub purpose: Option<String>,

    /// Code used in URL
    pub code: String,

    /// The resource type(s) this search parameter applies to
    pub base: Vec<String>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "type")]
    pub r#type: String,

    /// FHIRPath expression that extracts the values
    pub expression: Option<String>,

    /// XPath that extracts the values
    pub xpath: Option<String>,

    /// normal | phonetic | nearby | distance | other
    #[serde(rename = "xpathUsage")]
    pub xpath_usage: Option<String>,

    /// Types of resource (if a resource reference)
    pub target: Option<Vec<String>>,

    /// Allow multiple values per parameter (or)
    #[serde(rename = "multipleOr")]
    pub multiple_or: Option<bool>,

    /// Allow multiple parameters (and)
    #[serde(rename = "multipleAnd")]
    pub multiple_and: Option<bool>,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<Vec<String>>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | ofType
    pub modifier: Option<Vec<String>>,

    /// Chained names supported
    pub chain: Option<Vec<String>>,

    /// For Composite resources to define the parts
    pub component: Option<Vec<SearchParameterComponent>>,
}
