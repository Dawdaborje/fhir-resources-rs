//! FHIR R4 OperationDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Parameters for the operation/query
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name in Parameters.parameter.name or in URL
    pub name: String,

    /// in | out
    #[serde(rename = "use")]
    pub r#use: String,

    /// Minimum Cardinality
    pub min: i32,

    /// Maximum Cardinality (a number or *)
    pub max: String,

    /// Description of meaning/use
    pub documentation: Option<String>,

    /// What type this parameter has
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// If type is Reference | canonical, allowed targets
    #[serde(rename = "targetProfile")]
    pub target_profile: Option<Vec<String>>,

    /// number | date | string | token | reference | composite | quantity | uri | special
    #[serde(rename = "searchType")]
    pub search_type: Option<String>,

    /// ValueSet details if this is coded
    pub binding: Option<OperationDefinitionParameterBinding>,

    /// References to this parameter
    #[serde(rename = "referencedFrom")]
    pub referenced_from: Option<Vec<OperationDefinitionParameterReferencedFrom>>,

    /// Parts of a nested Parameter
    pub part: Option<Vec<OperationDefinitionParameter>>,
}

/// ValueSet details if this is coded
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameterBinding {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// required | extensible | preferred | example
    pub strength: String,

    /// Source of value set
    #[serde(rename = "valueSet")]
    pub value_set: String,
}

/// References to this parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionParameterReferencedFrom {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Referencing parameter
    pub source: String,

    /// Element id of reference
    #[serde(rename = "sourceId")]
    pub source_id: Option<String>,
}

/// Define overloaded variants for when generating code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinitionOverload {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of parameter to include in overload
    #[serde(rename = "parameterName")]
    pub parameter_name: Option<Vec<String>>,

    /// Comments to go on overload
    pub comment: Option<String>,
}

/// A formal computable definition of an operation (on the RESTful interface) or a named query (using the search interaction).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationDefinition {
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

    /// Canonical identifier for this operation definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Business version of the operation definition
    pub version: Option<String>,

    /// Name for this operation definition (computer friendly)
    pub name: String,

    /// Name for this operation definition (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// operation | query
    pub kind: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the operation definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for operation definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this operation definition is defined
    pub purpose: Option<String>,

    /// Whether content is changed by the operation
    #[serde(rename = "affectsState")]
    pub affects_state: Option<bool>,

    /// Name used to invoke the operation
    pub code: String,

    /// Additional information about use
    pub comment: Option<String>,

    /// Marks this as a profile of the base
    pub base: Option<String>,

    /// Types this operation applies to
    pub resource: Option<Vec<String>>,

    /// Invoke at the system level?
    pub system: bool,

    /// Invoke at the type level?
    #[serde(rename = "type")]
    pub r#type: bool,

    /// Invoke on an instance?
    pub instance: bool,

    /// Validation information for in parameters
    #[serde(rename = "inputProfile")]
    pub input_profile: Option<String>,

    /// Validation information for out parameters
    #[serde(rename = "outputProfile")]
    pub output_profile: Option<String>,

    /// Parameters for the operation/query
    pub parameter: Option<Vec<OperationDefinitionParameter>>,

    /// Define overloaded variants for when generating code
    pub overload: Option<Vec<OperationDefinitionOverload>>,
}
