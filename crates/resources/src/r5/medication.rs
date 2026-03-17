//! FHIR R5 Medication Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// The ingredient (substance or medication) that the ingredient.strength relates to
    pub item: CodeableReference,

    /// Active ingredient indicator
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,

    /// Quantity of ingredient present
    #[serde(rename = "strengthRatio")]
    pub strength_ratio: Option<Ratio>,

    #[serde(rename = "strengthCodeableConcept")]
    pub strength_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "strengthQuantity")]
    pub strength_quantity: Option<Quantity>,
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

/// This resource is primarily used for the identification and definition of a medication, including ingredients, for the purposes of prescribing, dispensing, and administering a medication as well as ...
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

    /// Organization that has authorization to market medication
    #[serde(rename = "marketingAuthorizationHolder")]
    pub marketing_authorization_holder: Option<Box<Reference>>,

    /// powder | tablets | capsule +
    #[serde(rename = "doseForm")]
    pub dose_form: Option<CodeableConcept>,

    /// When the specified product code does not infer a package size, this is the specific amount of drug in the product
    #[serde(rename = "totalVolume")]
    pub total_volume: Option<Quantity>,

    /// Active or inactive ingredient
    pub ingredient: Option<Vec<MedicationIngredient>>,

    /// Details about packaged medications
    pub batch: Option<MedicationBatch>,

    /// Knowledge about this medication
    pub definition: Option<Box<Reference>>,
}
