//! FHIR R4B CarePlan Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Action to occur as part of plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarePlanActivity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Results of the activity
    #[serde(rename = "outcomeCodeableConcept")]
    pub outcome_codeable_concept: Option<Vec<CodeableConcept>>,

    /// Appointment, Encounter, Procedure, etc.
    #[serde(rename = "outcomeReference")]
    pub outcome_reference: Option<Vec<Box<Reference>>>,

    /// Comments about the activity status/progress
    pub progress: Option<Vec<Annotation>>,

    /// Activity details defined in specific resource
    pub reference: Option<Box<Reference>>,

    /// In-line definition of activity
    pub detail: Option<CarePlanActivityDetail>,
}

/// In-line definition of activity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarePlanActivityDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Appointment | CommunicationRequest | DeviceRequest | MedicationRequest | NutritionOrder | Task | ServiceRequest | VisionPrescription
    pub kind: Option<String>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Detail type of activity
    pub code: Option<CodeableConcept>,

    /// Why activity should be done or why activity was prohibited
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why activity is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Goals this activity relates to
    pub goal: Option<Vec<Box<Reference>>>,

    /// not-started | scheduled | in-progress | on-hold | completed | cancelled | stopped | unknown | entered-in-error
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// If true, activity is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// When activity is to occur
    pub scheduled: Option<serde_json::Value>,

    /// Where it should happen
    pub location: Option<Box<Reference>>,

    /// Who will be responsible?
    pub performer: Option<Vec<Box<Reference>>>,

    /// What is to be administered/supplied
    pub product: Option<serde_json::Value>,

    /// How to consume/day?
    #[serde(rename = "dailyAmount")]
    pub daily_amount: Option<Quantity>,

    /// How much to administer/supply/consume
    pub quantity: Option<Quantity>,

    /// Extra info describing activity to perform
    pub description: Option<String>,
}

/// Describes the intention of how one or more practitioners intend to deliver care for a particular patient, group or community for a period of time, possibly limited to care for a specific condition ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan {
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

    /// External Ids for this plan
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Fulfills CarePlan
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// CarePlan replaced by this CarePlan
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Part of referenced CarePlan
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// proposal | plan | order | option
    pub intent: String,

    /// Type of plan
    pub category: Option<Vec<CodeableConcept>>,

    /// Human-friendly name for the care plan
    pub title: Option<String>,

    /// Summary of nature of plan
    pub description: Option<String>,

    /// Who the care plan is for
    pub subject: Box<Reference>,

    /// Encounter created as part of
    pub encounter: Option<Box<Reference>>,

    /// Time period plan covers
    pub period: Option<Period>,

    /// Date record was first recorded
    pub created: Option<String>,

    /// Who is the designated responsible party
    pub author: Option<Box<Reference>>,

    /// Who provided the content of the care plan
    pub contributor: Option<Vec<Box<Reference>>>,

    /// Who's involved in plan?
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<Box<Reference>>>,

    /// Health issues this plan addresses
    pub addresses: Option<Vec<Box<Reference>>>,

    /// Information considered as part of plan
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Desired outcome of plan
    pub goal: Option<Vec<Box<Reference>>>,

    /// Action to occur as part of plan
    pub activity: Option<Vec<CarePlanActivity>>,

    /// Comments about the plan
    pub note: Option<Vec<Annotation>>,
}
