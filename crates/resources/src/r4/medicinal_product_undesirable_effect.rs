//! FHIR R4 MedicinalProductUndesirableEffect Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Describe the undesirable effects of the medicinal product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductUndesirableEffect {
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

    /// The medication for which this is an indication
    pub subject: Option<Vec<Box<Reference>>>,

    /// The symptom, condition or undesirable effect
    #[serde(rename = "symptomConditionEffect")]
    pub symptom_condition_effect: Option<CodeableConcept>,

    /// Classification of the effect
    pub classification: Option<CodeableConcept>,

    /// The frequency of occurrence of the effect
    #[serde(rename = "frequencyOfOccurrence")]
    pub frequency_of_occurrence: Option<CodeableConcept>,

    /// The population group to which this applies
    pub population: Option<Vec<Population>>,
}
