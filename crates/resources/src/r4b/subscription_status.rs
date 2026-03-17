//! FHIR R4B SubscriptionStatus Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Detailed information about any events relevant to this notification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatusNotificationEvent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Event number
    #[serde(rename = "eventNumber")]
    pub event_number: String,

    /// The instant this event occurred
    pub timestamp: Option<String>,

    /// The focus of this event
    pub focus: Option<Box<Reference>>,

    /// Additional context for this event
    #[serde(rename = "additionalContext")]
    pub additional_context: Option<Vec<Box<Reference>>>,
}

/// The SubscriptionStatus resource describes the state of a Subscription during notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionStatus {
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

    /// requested | active | error | off | entered-in-error
    pub status: Option<String>,

    /// handshake | heartbeat | event-notification | query-status | query-event
    #[serde(rename = "type")]
    pub r#type: String,

    /// Events since the Subscription was created
    #[serde(rename = "eventsSinceSubscriptionStart")]
    pub events_since_subscription_start: Option<String>,

    /// Detailed information about any events relevant to this notification
    #[serde(rename = "notificationEvent")]
    pub notification_event: Option<Vec<SubscriptionStatusNotificationEvent>>,

    /// Reference to the Subscription responsible for this notification
    pub subscription: Box<Reference>,

    /// Reference to the SubscriptionTopic this notification relates to
    pub topic: Option<String>,

    /// List of errors on the subscription
    pub error: Option<Vec<CodeableConcept>>,
}
