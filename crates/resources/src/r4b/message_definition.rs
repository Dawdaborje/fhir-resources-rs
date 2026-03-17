//! FHIR R4B MessageDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Resource(s) that are the subject of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinitionFocus {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of resource
    pub code: String,

    /// Profile that must be adhered to by focus
    pub profile: Option<String>,

    /// Minimum number of focuses of this type
    pub min: u32,

    /// Maximum number of focuses of this type
    pub max: Option<String>,
}

/// Responses to this message
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinitionAllowedResponse {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to allowed message definition response
    pub message: String,

    /// When should this response be used
    pub situation: Option<String>,
}

/// Defines the characteristics of a message that can be shared between systems, including the type of event that initiates the message, the content to be transmitted and what response(s), if any, are ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageDefinition {
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

    /// Business Identifier for a given MessageDefinition
    pub url: Option<String>,

    /// Primary key for the message definition on a given server
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the message definition
    pub version: Option<String>,

    /// Name for this message definition (computer friendly)
    pub name: Option<String>,

    /// Name for this message definition (human friendly)
    pub title: Option<String>,

    /// Takes the place of
    pub replaces: Option<Vec<String>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: String,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the message definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for message definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this message definition is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Definition this one is based on
    pub base: Option<String>,

    /// Protocol/workflow this is part of
    pub parent: Option<Vec<String>>,

    /// Event code or link to the EventDefinition
    pub event: serde_json::Value,

    /// consequence | currency | notification
    pub category: Option<String>,

    /// Resource(s) that are the subject of the event
    pub focus: Option<Vec<MessageDefinitionFocus>>,

    /// always | on-error | never | on-success
    #[serde(rename = "responseRequired")]
    pub response_required: Option<String>,

    /// Responses to this message
    #[serde(rename = "allowedResponse")]
    pub allowed_response: Option<Vec<MessageDefinitionAllowedResponse>>,

    /// Canonical reference to a GraphDefinition
    pub graph: Option<Vec<String>>,
}
