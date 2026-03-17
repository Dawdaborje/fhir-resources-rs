//! FHIR R4B AllergyIntolerance Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Adverse Reaction Events linked to exposure to substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntoleranceReaction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specific substance or pharmaceutical product considered to be responsible for event
    pub substance: Option<CodeableConcept>,

    /// Clinical symptoms/signs associated with the Event
    pub manifestation: Vec<CodeableConcept>,

    /// Description of the event as a whole
    pub description: Option<String>,

    /// Date(/time) when manifestations showed
    pub onset: Option<String>,

    /// mild | moderate | severe (of event as a whole)
    pub severity: Option<String>,

    /// How the subject was exposed to the substance
    #[serde(rename = "exposureRoute")]
    pub exposure_route: Option<CodeableConcept>,

    /// Text about event not captured in other fields
    pub note: Option<Vec<Annotation>>,
}

/// Risk of harmful or undesirable, physiological response which is unique to an individual and associated with exposure to a substance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllergyIntolerance {
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

    /// External ids for this item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | inactive | resolved
    #[serde(rename = "clinicalStatus")]
    pub clinical_status: Option<CodeableConcept>,

    /// unconfirmed | confirmed | refuted | entered-in-error
    #[serde(rename = "verificationStatus")]
    pub verification_status: Option<CodeableConcept>,

    /// allergy | intolerance - Underlying mechanism (if known)
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// food | medication | environment | biologic
    pub category: Option<Vec<String>>,

    /// low | high | unable-to-assess
    pub criticality: Option<String>,

    /// Code that identifies the allergy or intolerance
    pub code: Option<CodeableConcept>,

    /// Who the sensitivity is for
    pub patient: Box<Reference>,

    /// Encounter when the allergy or intolerance was asserted
    pub encounter: Option<Box<Reference>>,

    /// When allergy or intolerance was identified
    pub onset: Option<serde_json::Value>,

    /// Date first version of the resource instance was recorded
    #[serde(rename = "recordedDate")]
    pub recorded_date: Option<String>,

    /// Who recorded the sensitivity
    pub recorder: Option<Box<Reference>>,

    /// Source of the information about the allergy
    pub asserter: Option<Box<Reference>>,

    /// Date(/time) of last known occurrence of a reaction
    #[serde(rename = "lastOccurrence")]
    pub last_occurrence: Option<String>,

    /// Additional text not captured in other fields
    pub note: Option<Vec<Annotation>>,

    /// Adverse Reaction Events linked to exposure to substance
    pub reaction: Option<Vec<AllergyIntoleranceReaction>>,
}
