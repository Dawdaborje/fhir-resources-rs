//! FHIR R5 Condition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who or what participated in the activities related to the condition and how they were involved
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of involvement
    pub function: Option<CodeableConcept>,

    /// Who or what participated in the activities related to the condition
    pub actor: Box<Reference>,
}

/// Stage/grade, usually assessed formally
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionStage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Simple summary (disease specific)
    pub summary: Option<CodeableConcept>,

    /// Formal record of assessment
    pub assessment: Option<Vec<Box<Reference>>>,

    /// Kind of staging
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
}

/// A clinical condition, problem, diagnosis, or other event, situation, issue, or clinical concept that has risen to a level of concern.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
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

    /// External Ids for this condition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | recurrence | relapse | inactive | remission | resolved | unknown
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: CodeableConcept,

    /// unconfirmed | provisional | differential | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodeableConcept>,

    /// problem-list-item | encounter-diagnosis
    pub category: Option<Vec<CodeableConcept>>,

    /// Subjective severity of condition
    pub severity: Option<CodeableConcept>,

    /// Identification of the condition, problem or diagnosis
    pub code: Option<CodeableConcept>,

    /// Anatomical location, if relevant
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// Who has the condition?
    pub subject: Box<Reference>,

    /// The Encounter during which this Condition was created
    pub encounter: Option<Box<Reference>>,

    /// Estimated or actual date, date-time, or age
    pub onset: Option<serde_json::Value>,

    /// When in resolution/remission
    pub abatement: Option<serde_json::Value>,

    /// Date condition was first recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<String>,

    /// Who or what participated in the activities related to the condition and how they were involved
    pub participant: Option<Vec<ConditionParticipant>>,

    /// Stage/grade, usually assessed formally
    pub stage: Option<Vec<ConditionStage>>,

    /// Supporting evidence for the verification status
    pub evidence: Option<Vec<CodeableReference>>,

    /// Additional information about the Condition
    pub note: Option<Vec<Annotation>>,
}
