//! FHIR R5 CarePlan Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Action to occur or has occurred as part of plan
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

    /// Results of the activity (concept, or Appointment, Encounter, Procedure, etc.)
    #[serde(rename = "performedActivity")]
    pub performed_activity: Option<Vec<CodeableReference>>,

    /// Comments about the activity status/progress
    pub progress: Option<Vec<Annotation>>,

    /// Activity that is intended to be part of the care plan
    #[serde(rename = "plannedActivityReference")]
    pub planned_activity_reference: Option<Box<Reference>>,
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

    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// CarePlan replaced by this CarePlan
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Part of referenced CarePlan
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// proposal | plan | order | option | directive
    pub intent: String,

    /// Type of plan
    pub category: Option<Vec<CodeableConcept>>,

    /// Human-friendly name for the care plan
    pub title: Option<String>,

    /// Summary of nature of plan
    pub description: Option<String>,

    /// Who the care plan is for
    pub subject: Box<Reference>,

    /// The Encounter during which this CarePlan was created
    pub encounter: Option<Box<Reference>>,

    /// Time period plan covers
    pub period: Option<Period>,

    /// Date record was first recorded
    pub created: Option<String>,

    /// Who is the designated responsible party
    pub custodian: Option<Box<Reference>>,

    /// Who provided the content of the care plan
    pub contributor: Option<Vec<Box<Reference>>>,

    /// Who's involved in plan?
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<Box<Reference>>>,

    /// Health issues this plan addresses
    pub addresses: Option<Vec<CodeableReference>>,

    /// Information considered as part of plan
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Desired outcome of plan
    pub goal: Option<Vec<Box<Reference>>>,

    /// Action to occur or has occurred as part of plan
    pub activity: Option<Vec<CarePlanActivity>>,

    /// Comments about the plan
    pub note: Option<Vec<Annotation>>,
}
