//! FHIR R5 EpisodeOfCare Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Past list of status codes (the current status may be included to cover the start date of the status)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareStatusHistory {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: String,

    /// Duration the EpisodeOfCare was in the specified status
    pub period: Period,
}

/// The list of medical reasons that are expected to be addressed during the episode of care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareReason {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What the reason value should be used for/as
    #[serde(rename = "use")]
    pub r#use: Option<CodeableConcept>,

    /// Medical reason to be addressed
    pub value: Option<Vec<CodeableReference>>,
}

/// The list of medical conditions that were addressed during the episode of care
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCareDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The medical condition that was addressed during the episode of care
    pub condition: Option<Vec<CodeableReference>>,

    /// Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)
    #[serde(rename = "use")]
    pub r#use: Option<CodeableConcept>,
}

/// An association between a patient and an organization / healthcare provider(s) during which time encounters may occur. The managing organization assumes a level of responsibility for the patient dur...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeOfCare {
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

    /// Business Identifier(s) relevant for this EpisodeOfCare
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    pub status: String,

    /// Past list of status codes (the current status may be included to cover the start date of the status)
    #[serde(rename = "statusHistory")]
    pub status_history: Option<Vec<EpisodeOfCareStatusHistory>>,

    /// Type/class - e.g. specialist referral, disease management
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// The list of medical reasons that are expected to be addressed during the episode of care
    pub reason: Option<Vec<EpisodeOfCareReason>>,

    /// The list of medical conditions that were addressed during the episode of care
    pub diagnosis: Option<Vec<EpisodeOfCareDiagnosis>>,

    /// The patient who is the focus of this episode of care
    pub patient: Box<Reference>,

    /// Organization that assumes responsibility for care coordination
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Box<Reference>>,

    /// Interval during responsibility is assumed
    pub period: Option<Period>,

    /// Originating Referral Request(s)
    #[serde(rename = "referralRequest")]
    pub referral_request: Option<Vec<Box<Reference>>>,

    /// Care manager/care coordinator for the patient
    #[serde(rename = "careManager")]
    pub care_manager: Option<Box<Reference>>,

    /// Other practitioners facilitating this episode of care
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<Box<Reference>>>,

    /// The set of accounts that may be used for billing for this EpisodeOfCare
    pub account: Option<Vec<Box<Reference>>>,
}
