//! FHIR R4B Medication Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Active or inactive ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual ingredient or content
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,

    #[serde(rename = "itemReference")]
    pub item_reference: Box<Reference>,

    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,

    /// Quantity of ingredient present
    pub strength: Option<Ratio>,
}

/// Details about packaged medications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationBatch {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifier assigned to batch
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// When batch will expire
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
}

/// This resource is primarily used for the identification and definition of a medication for the purposes of prescribing, dispensing, and administering a medication as well as for making statements ab...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Medication {
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

    /// Business identifier for this medication
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Codes that identify this medication
    pub code: Option<CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: Option<String>,

    /// Manufacturer of the item
    pub manufacturer: Option<Box<Reference>>,

    /// powder | tablets | capsule +
    pub form: Option<CodeableConcept>,

    /// Amount of drug in package
    pub amount: Option<Ratio>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationIngredient>>,

    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,
}
