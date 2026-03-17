//! FHIR R5 ResearchStudy Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Additional names for the study
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyLabel {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// primary | official | scientific | plain-language | subtitle | short-title | acronym | earlier-title | language | auto-translated | human-use | machine-use | duplicate-uid
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The name
    pub value: Option<String>,
}

/// Sponsors, collaborators, and other parties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyAssociatedParty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of associated party
    pub name: Option<String>,

    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator | collaborator | funding-source | general-contact | recruitment-contact | sub-investigator | study-director | study-chair
    pub role: CodeableConcept,

    /// When active in the role
    pub period: Option<Vec<Period>>,

    /// nih | fda | government | nonprofit | academic | industry
    pub classifier: Option<Vec<CodeableConcept>>,

    /// Individual or organization associated with study (use practitionerRole to specify their organisation)
    pub party: Option<Box<Reference>>,
}

/// Status of study with time for that status
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyProgressStatus {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for status or state (e.g. recruitment status)
    pub state: CodeableConcept,

    /// Actual if true else anticipated
    pub actual: Option<bool>,

    /// Date range
    pub period: Option<Period>,
}

/// Target or actual group of participants enrolled in study
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyRecruitment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Estimated total number of participants to be enrolled
    #[serde(rename = "targetNumber")]
    pub target_number: Option<u32>,

    /// Actual total number of participants enrolled in study
    #[serde(rename = "actualNumber")]
    pub actual_number: Option<u32>,

    /// Inclusion and exclusion criteria
    pub eligibility: Option<Box<Reference>>,

    /// Group of participants who were enrolled in study
    #[serde(rename = "actualGroup")]
    pub actual_group: Option<Box<Reference>>,
}

/// Defined path through the study for a subject
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyComparisonGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// Label for study comparisonGroup
    pub name: String,

    /// Categorization of study comparisonGroup
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Short explanation of study path
    pub description: Option<String>,

    /// Interventions or exposures in this comparisonGroup or cohort
    #[serde(rename = "intendedExposure")]
    pub intended_exposure: Option<Vec<Box<Reference>>>,

    /// Group of participants who were enrolled in study comparisonGroup
    #[serde(rename = "observedGroup")]
    pub observed_group: Option<Box<Reference>>,
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

    /// Description of the objective
    pub description: Option<String>,
}

/// A variable measured during the study
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchStudyOutcomeMeasure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for the outcome
    pub name: Option<String>,

    /// primary | secondary | exploratory
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Description of the outcome
    pub description: Option<String>,

    /// Structured outcome definition
    pub reference: Option<Box<Reference>>,
}

/// A scientific study of nature that sometimes includes processes involved in health and disease. For example, clinical trials are research studies that involve people. These studies may be related to...
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

    /// Canonical identifier for this study resource
    pub url: Option<String>,

    /// Business Identifier for study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The business version for the study record
    pub version: Option<String>,

    /// Name for this study (computer friendly)
    pub name: Option<String>,

    /// Human readable name of the study
    pub title: Option<String>,

    /// Additional names for the study
    pub label: Option<Vec<ResearchStudyLabel>>,

    /// Steps followed in executing study
    pub protocol: Option<Vec<Box<Reference>>>,

    /// Part of larger study
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// References, URLs, and attachments
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Date the resource last changed
    pub date: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    #[serde(rename = "primaryPurposeType")]
    pub primary_purpose_type: Option<CodeableConcept>,

    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    pub phase: Option<CodeableConcept>,

    /// Classifications of the study design characteristics
    #[serde(rename = "studyDesign")]
    pub study_design: Option<Vec<CodeableConcept>>,

    /// Drugs, devices, etc. under study
    pub focus: Option<Vec<CodeableReference>>,

    /// Condition being studied
    pub condition: Option<Vec<CodeableConcept>>,

    /// Used to search for the study
    pub keyword: Option<Vec<CodeableConcept>>,

    /// Geographic area for the study
    pub region: Option<Vec<CodeableConcept>>,

    /// Brief text explaining the study
    #[serde(rename = "descriptionSummary")]
    pub description_summary: Option<String>,

    /// Detailed narrative of the study
    pub description: Option<String>,

    /// When the study began and ended
    pub period: Option<Period>,

    /// Facility where study activities are conducted
    pub site: Option<Vec<Box<Reference>>>,

    /// Comments made about the study
    pub note: Option<Vec<Annotation>>,

    /// Classification for the study
    pub classifier: Option<Vec<CodeableConcept>>,

    /// Sponsors, collaborators, and other parties
    #[serde(rename = "associatedParty")]
    pub associated_party: Option<Vec<ResearchStudyAssociatedParty>>,

    /// Status of study with time for that status
    #[serde(rename = "progressStatus")]
    pub progress_status: Option<Vec<ResearchStudyProgressStatus>>,

    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    #[serde(rename = "whyStopped")]
    pub why_stopped: Option<CodeableConcept>,

    /// Target or actual group of participants enrolled in study
    pub recruitment: Option<ResearchStudyRecruitment>,

    /// Defined path through the study for a subject
    #[serde(rename = "comparisonGroup")]
    pub comparison_group: Option<Vec<ResearchStudyComparisonGroup>>,

    /// A goal for the study
    pub objective: Option<Vec<ResearchStudyObjective>>,

    /// A variable measured during the study
    #[serde(rename = "outcomeMeasure")]
    pub outcome_measure: Option<Vec<ResearchStudyOutcomeMeasure>>,

    /// Link to results generated during the study
    pub result: Option<Vec<Box<Reference>>>,
}
