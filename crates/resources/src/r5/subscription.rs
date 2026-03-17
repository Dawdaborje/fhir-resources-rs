//! FHIR R5 Subscription Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Criteria for narrowing the subscription topic stream
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionFilterBy {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Allowed Resource (reference to definition) for this Subscription filter
    #[serde(rename = "resourceType")]
    pub resource_type: Option<String>,

    /// Filter label defined in SubscriptionTopic
    #[serde(rename = "filterParameter")]
    pub filter_parameter: String,

    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    pub comparator: Option<String>,

    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    pub modifier: Option<String>,

    /// Literal value or resource path
    pub value: String,
}

/// Channel type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name (key) of the parameter
    pub name: String,

    /// Value of the parameter to use or pass through
    pub value: String,
}

/// The subscription resource describes a particular client's request to be notified about a SubscriptionTopic.
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

    /// Additional identifiers (business identifier)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Human readable name for this subscription
    pub name: Option<String>,

    /// requested | active | error | off | entered-in-error
    pub status: String,

    /// Reference to the subscription topic being subscribed to
    pub topic: String,

    /// Contact details for source (e.g. troubleshooting)
    pub contact: Option<Vec<ContactPoint>>,

    /// When to automatically delete the subscription
    pub end: Option<String>,

    /// Entity responsible for Subscription changes
    #[serde(rename = "managingEntity")]
    pub managing_entity: Option<Box<Reference>>,

    /// Description of why this subscription was created
    pub reason: Option<String>,

    /// Criteria for narrowing the subscription topic stream
    #[serde(rename = "filterBy")]
    pub filter_by: Option<Vec<SubscriptionFilterBy>>,

    /// Channel type for notifications
    #[serde(rename = "channelType")]
    pub channel_type: Coding,

    /// Where the channel points to
    pub endpoint: Option<String>,

    /// Channel type
    pub parameter: Option<Vec<SubscriptionParameter>>,

    /// Interval in seconds to send 'heartbeat' notification
    #[serde(rename = "heartbeatPeriod")]
    pub heartbeat_period: Option<u32>,

    /// Timeout in seconds to attempt notification delivery
    pub timeout: Option<u32>,

    /// MIME type to send, or omit for no payload
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,

    /// empty | id-only | full-resource
    pub content: Option<String>,

    /// Maximum number of events that can be combined in a single notification
    #[serde(rename = "maxCount")]
    pub max_count: Option<i32>,
}
