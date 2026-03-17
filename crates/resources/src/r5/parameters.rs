//! FHIR R5 Parameters Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Operation Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParametersParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name from the definition
    pub name: String,

    /// If parameter is a data type
    pub value: Option<serde_json::Value>,

    /// If parameter is a whole resource
    pub resource: Option<serde_json::Value>,

    /// Named part of a multi-part parameter
    pub part: Option<Vec<ParametersParameter>>,
}

/// This resource is used to pass information into and back from an operation (whether invoked directly from REST or within a messaging environment). It is not persisted or allowed to be referenced by ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
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

    /// Operation Parameter
    pub parameter: Option<Vec<ParametersParameter>>,
}
