//! FHIR R5 AdverseEvent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who was involved in the adverse event or the potential adverse event and what they did
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of involvement
    pub function: Option<CodeableConcept>,

    /// Who was involved in the adverse event or the potential adverse event
    pub actor: Box<Reference>,
}

/// The suspected agent causing the adverse event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Refers to the specific entity that caused the adverse event
    pub instance: serde_json::Value,

    /// Information on the possible cause of the event
    pub causality: Option<AdverseEventSuspectEntityCausality>,
}

/// Information on the possible cause of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSuspectEntityCausality {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Method of evaluating the relatedness of the suspected entity to the event
    #[serde(rename = "assessmentMethod")]
    pub assessment_method: Option<CodeableConcept>,

    /// Result of the assessment regarding the relatedness of the suspected entity to the event
    #[serde(rename = "entityRelatedness")]
    pub entity_relatedness: Option<CodeableConcept>,

    /// Author of the information on the possible cause of the event
    pub author: Option<Box<Reference>>,
}

/// Contributing factors suspected to have increased the probability or severity of the adverse event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventContributingFactor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item suspected to have increased the probability or severity of the adverse event
    pub item: serde_json::Value,
}

/// Preventive actions that contributed to avoiding the adverse event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventPreventiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Action that contributed to avoiding the adverse event
    pub item: serde_json::Value,
}

/// Ameliorating actions taken after the adverse event occured in order to reduce the extent of harm
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventMitigatingAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Ameliorating action taken after the adverse event occured in order to reduce the extent of harm
    pub item: serde_json::Value,
}

/// Supporting information relevant to the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEventSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Subject medical history or document relevant to this adverse event
    pub item: serde_json::Value,
}

/// An event (i.e. any change to current patient status) that may be related to unintended effects on a patient or research participant. The unintended effects may require additional monitoring, treatm...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdverseEvent {
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

    /// Business identifier for the event
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// in-progress | completed | entered-in-error | unknown
    pub status: String,

    /// actual | potential
    pub actuality: String,

    /// wrong-patient | procedure-mishap | medication-mishap | device | unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    pub category: Option<Vec<CodeableConcept>>,

    /// Event or incident that occurred or was averted
    pub code: Option<CodeableConcept>,

    /// Subject impacted by event
    pub subject: Box<Reference>,

    /// The Encounter associated with the start of the AdverseEvent
    pub encounter: Option<Box<Reference>>,

    /// When the event occurred
    pub occurrence: Option<serde_json::Value>,

    /// When the event was detected
    pub detected: Option<String>,

    /// When the event was recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<String>,

    /// Effect on the subject due to this event
    #[serde(rename = "resultingEffect")]
    pub resulting_effect: Option<Vec<Box<Reference>>>,

    /// Location where adverse event occurred
    pub location: Option<Box<Reference>>,

    /// Seriousness or gravity of the event
    pub seriousness: Option<CodeableConcept>,

    /// Type of outcome from the adverse event
    pub outcome: Option<Vec<CodeableConcept>>,

    /// Who recorded the adverse event
    pub recorder: Option<Box<Reference>>,

    /// Who was involved in the adverse event or the potential adverse event and what they did
    pub participant: Option<Vec<AdverseEventParticipant>>,

    /// Research study that the subject is enrolled in
    pub study: Option<Vec<Box<Reference>>>,

    /// Considered likely or probable or anticipated in the research study
    #[serde(rename = "expectedInResearchStudy")]
    pub expected_in_research_study: Option<bool>,

    /// The suspected agent causing the adverse event
    #[serde(rename = "suspectEntity")]
    pub suspect_entity: Option<Vec<AdverseEventSuspectEntity>>,

    /// Contributing factors suspected to have increased the probability or severity of the adverse event
    #[serde(rename = "contributingFactor")]
    pub contributing_factor: Option<Vec<AdverseEventContributingFactor>>,

    /// Preventive actions that contributed to avoiding the adverse event
    #[serde(rename = "preventiveAction")]
    pub preventive_action: Option<Vec<AdverseEventPreventiveAction>>,

    /// Ameliorating actions taken after the adverse event occured in order to reduce the extent of harm
    #[serde(rename = "mitigatingAction")]
    pub mitigating_action: Option<Vec<AdverseEventMitigatingAction>>,

    /// Supporting information relevant to the event
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<AdverseEventSupportingInfo>>,

    /// Comment on adverse event
    pub note: Option<Vec<Annotation>>,
}
