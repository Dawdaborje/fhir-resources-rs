//! FHIR R4B Subscription Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The channel on which to report matches to the criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionChannel {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// rest-hook | websocket | email | sms | message
    #[serde(rename = "type")]
    pub r#type: String,

    /// Where the channel points to
    pub endpoint: Option<String>,

    /// MIME type to send, or omit for no payload
    pub payload: Option<String>,

    /// Usage depends on the channel type
    pub header: Option<Vec<String>>,
}

/// The subscription resource is used to define a push-based subscription from a server to another system. Once a subscription is registered with the server, the server checks every resource that is cr...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
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

    /// requested | active | error | off
    pub status: String,

    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<ContactPoint>>,

    /// When to automatically delete the subscription
    pub end: Option<String>,

    /// Description of why this subscription was created
    pub reason: String,

    /// Rule for server push
    pub criteria: String,

    /// Latest error note
    pub error: Option<String>,

    /// The channel on which to report matches to the criteria
    pub channel: SubscriptionChannel,
}
