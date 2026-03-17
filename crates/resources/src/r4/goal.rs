//! FHIR R4 Goal Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Target outcome for the goal
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The parameter whose value is being tracked
    pub measure: Option<CodeableConcept>,

    /// The target value to be achieved
    pub detail: Option<serde_json::Value>,

    /// Reach goal on or before
    pub due: Option<serde_json::Value>,
}

/// Describes the intended objective(s) for a patient, group or organization care, for example, weight loss, restoring an activity of daily living, obtaining herd immunity via immunization, meeting a p...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
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

    /// External Ids for this goal
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    #[serde(rename = "lifecycleStatus")]
    pub lifecycle_status: String,

    /// in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    #[serde(rename = "achievementStatus")]
    pub achievement_status: Option<CodeableConcept>,

    /// E.g. Treatment, dietary, behavioral, etc.
    pub category: Option<Vec<CodeableConcept>>,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<CodeableConcept>,

    /// Code or text describing goal
    pub description: CodeableConcept,

    /// Who this goal is intended for
    pub subject: Box<Reference>,

    /// When goal pursuit begins
    pub start: Option<serde_json::Value>,

    /// Target outcome for the goal
    pub target: Option<Vec<GoalTarget>>,

    /// When goal status took effect
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<String>,

    /// Who's responsible for creating Goal?
    #[serde(rename = "expressedBy")]
    pub expressed_by: Option<Box<Reference>>,

    /// Issues addressed by this goal
    pub addresses: Option<Vec<Box<Reference>>>,

    /// Comments about the goal
    pub note: Option<Vec<Annotation>>,

    /// What result was achieved regarding the goal?
    #[serde(rename = "outcomeCode")]
    pub outcome_code: Option<Vec<CodeableConcept>>,

    /// Observation that resulted from goal
    #[serde(rename = "outcomeReference")]
    pub outcome_reference: Option<Vec<Box<Reference>>>,
}
