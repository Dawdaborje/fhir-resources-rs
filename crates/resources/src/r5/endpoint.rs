//! FHIR R5 Endpoint Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Set of payloads that are provided by this endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointPayload {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of content that may be used at this endpoint (e.g. XDS Discharge summaries)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Mimetype to send. If not specified, the content could be anything (including no payload, if the connectionType defined this)
    #[serde(rename = "mimeType")]
    pub mime_type: Option<Vec<String>>,
}

/// The technical details of an endpoint that can be used for electronic services, such as for web services providing XDS.b, a REST endpoint for another FHIR server, or a s/Mime email address. This may...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
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

    /// Identifies this endpoint across multiple systems
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | suspended | error | off | entered-in-error | test
    pub status: String,

    /// Protocol/Profile/Standard to be used with this endpoint connection
    #[serde(rename = "connectionType")]
    pub connection_type: Vec<CodeableConcept>,

    /// A name that this endpoint can be identified by
    pub name: Option<String>,

    /// Additional details about the endpoint that could be displayed as further information to identify the description beyond its name
    pub description: Option<String>,

    /// The type of environment(s) exposed at this endpoint
    #[serde(rename = "environmentType")]
    pub environment_type: Option<Vec<CodeableConcept>>,

    /// Organization that manages this endpoint (might not be the organization that exposes the endpoint)
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Box<Reference>>,

    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<ContactPoint>>,

    /// Interval the endpoint is expected to be operational
    pub period: Option<Period>,

    /// Set of payloads that are provided by this endpoint
    pub payload: Option<Vec<EndpointPayload>>,

    /// The technical base address for connecting to this endpoint
    pub address: String,

    /// Usage depends on the channel type
    pub header: Option<Vec<String>>,
}
