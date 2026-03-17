//! FHIR R4B ResearchStudy Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Defined path through the study for a subject
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyArm {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for study arm
    pub name: String,

    /// Categorization of study arm
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<String>,
}

/// A goal for the study
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyObjective {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for the objective
    pub name: Option<String>,

    /// primary | secondary | exploratory
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,
}

/// A process where a researcher or organization plans and then executes a series of steps intended to increase the field of healthcare-related knowledge. This includes studies of safety, efficacy, com...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudy {
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

    /// Business Identifier for study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Name for this study
    pub title: Option<String>,

    /// Steps followed in executing study
    pub protocol: Option<Vec<Box<Reference>>>,

    /// Part of larger study
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// active | administratively-completed | approved | closed-to-accrual | closed-to-accrual-and-intervention | completed | disapproved | in-review | temporarily-closed-to-accrual | temporarily-closed-to...
    pub status: String,

    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    #[serde(rename = "primaryPurposeType")]
    pub primary_purpose_type: Option<CodeableConcept>,

    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    pub phase: Option<CodeableConcept>,

    /// Classifications for the study
    pub category: Option<Vec<CodeableConcept>>,

    /// Drugs, devices, etc. under study
    pub focus: Option<Vec<CodeableConcept>>,

    /// Condition being studied
    pub condition: Option<Vec<CodeableConcept>>,

    /// Contact details for the study
    pub contact: Option<Vec<ContactDetail>>,

    /// References and dependencies
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Used to search for the study
    pub keyword: Option<Vec<CodeableConcept>>,

    /// Geographic region(s) for study
    pub location: Option<Vec<CodeableConcept>>,

    /// What this is study doing
    pub description: Option<String>,

    /// Inclusion & exclusion criteria
    pub enrollment: Option<Vec<Box<Reference>>>,

    /// When the study began and ended
    pub period: Option<Period>,

    /// Organization that initiates and is legally responsible for the study
    pub sponsor: Option<Box<Reference>>,

    /// Researcher who oversees multiple aspects of the study
    #[serde(rename = "principalInvestigator")]
    pub principal_investigator: Option<Box<Reference>>,

    /// Facility where study activities are conducted
    pub site: Option<Vec<Box<Reference>>>,

    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    #[serde(rename = "reasonStopped")]
    pub reason_stopped: Option<CodeableConcept>,

    /// Comments made about the study
    pub note: Option<Vec<Annotation>>,

    /// Defined path through the study for a subject
    pub arm: Option<Vec<ResearchStudyArm>>,

    /// A goal for the study
    pub objective: Option<Vec<ResearchStudyObjective>>,
}
