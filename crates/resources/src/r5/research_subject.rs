//! FHIR R5 ResearchSubject Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Subject status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubjectProgress {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// state | milestone
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    #[serde(rename = "subjectState")]
    pub subject_state: Option<CodeableConcept>,

    /// SignedUp | Screened | Randomized
    pub milestone: Option<CodeableConcept>,

    /// State change reason
    pub reason: Option<CodeableConcept>,

    /// State change date
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,

    /// State change date
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

/// A ResearchSubject is a participant or object which is the recipient of investigative activities in a research study.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubject {
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

    /// Business Identifier for research subject in a study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Subject status
    pub progress: Option<Vec<ResearchSubjectProgress>>,

    /// Start and end of participation
    pub period: Option<Period>,

    /// Study subject is part of
    pub study: Box<Reference>,

    /// Who or what is part of study
    pub subject: Box<Reference>,

    /// What path should be followed
    #[serde(rename = "assignedComparisonGroup")]
    pub assigned_comparison_group: Option<String>,

    /// What path was followed
    #[serde(rename = "actualComparisonGroup")]
    pub actual_comparison_group: Option<String>,

    /// Agreement to participate in study
    pub consent: Option<Vec<Box<Reference>>>,
}
