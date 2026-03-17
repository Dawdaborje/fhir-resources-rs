//! FHIR R4 MedicinalProductInteraction Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// The specific medication, food or laboratory test that interacts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductInteractionInteractant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific medication, food or laboratory test that interacts
    #[serde(rename = "itemReference")]
    pub item_reference: Box<Reference>,

    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,
}

/// The interactions of the medicinal product with other medicinal products, or other forms of interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductInteraction {
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

    /// The medication for which this is a described interaction
    pub subject: Option<Vec<Box<Reference>>>,

    /// The interaction described
    pub description: Option<String>,

    /// The specific medication, food or laboratory test that interacts
    pub interactant: Option<Vec<MedicinalProductInteractionInteractant>>,

    /// The type of the interaction e.g. drug-drug interaction, drug-food interaction, drug-lab test interaction
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The effect of the interaction, for example "reduced gastric absorption of primary medication"
    pub effect: Option<CodeableConcept>,

    /// The incidence of the interaction, e.g. theoretical, observed
    pub incidence: Option<CodeableConcept>,

    /// Actions for managing the interaction
    pub management: Option<CodeableConcept>,
}
