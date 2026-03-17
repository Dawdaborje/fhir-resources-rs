//! FHIR R5 NutritionIntake Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// What food or fluid product or item was consumed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeConsumedItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of food or fluid product
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Code that identifies the food or fluid product that was consumed
    #[serde(rename = "nutritionProduct")]
    pub nutrition_product: CodeableReference,

    /// Scheduled frequency of consumption
    pub schedule: Option<Timing>,

    /// Quantity of the specified food
    pub amount: Option<Quantity>,

    /// Rate at which enteral feeding was administered
    pub rate: Option<Quantity>,

    /// Flag to indicate if the food or fluid item was refused or otherwise not consumed
    #[serde(rename = "notConsumed")]
    pub not_consumed: Option<bool>,

    /// Reason food or fluid was not consumed
    #[serde(rename = "notConsumedReason")]
    pub not_consumed_reason: Option<CodeableConcept>,
}

/// Total nutrient for the whole meal, product, serving
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakeIngredientLabel {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Total nutrient consumed
    pub nutrient: CodeableReference,

    /// Total amount of nutrient consumed
    pub amount: Quantity,
}

/// Who was performed in the intake
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntakePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performer
    pub function: Option<CodeableConcept>,

    /// Who performed the intake
    pub actor: Box<Reference>,
}

/// A record of food or fluid that is being consumed by a patient. A NutritionIntake may indicate that the patient may be consuming the food or fluid now or has consumed the food or fluid in the past. ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionIntake {
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

    /// External identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Fulfils plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,

    /// Code representing an overall type of nutrition intake
    pub code: Option<CodeableConcept>,

    /// Who is/was consuming the food or fluid
    pub subject: Box<Reference>,

    /// Encounter associated with NutritionIntake
    pub encounter: Option<Box<Reference>>,

    /// The date/time or interval when the food or fluid is/was consumed
    pub occurrence: Option<serde_json::Value>,

    /// When the intake was recorded
    pub recorded: Option<String>,

    /// Person or organization that provided the information about the consumption of this food or fluid
    pub reported: Option<serde_json::Value>,

    /// What food or fluid product or item was consumed
    #[serde(rename = "consumedItem")]
    pub consumed_item: Vec<NutritionIntakeConsumedItem>,

    /// Total nutrient for the whole meal, product, serving
    #[serde(rename = "ingredientLabel")]
    pub ingredient_label: Option<Vec<NutritionIntakeIngredientLabel>>,

    /// Who was performed in the intake
    pub performer: Option<Vec<NutritionIntakePerformer>>,

    /// Where the intake occurred
    pub location: Option<Box<Reference>>,

    /// Additional supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// Reason for why the food or fluid is /was consumed
    pub reason: Option<Vec<CodeableReference>>,

    /// Further information about the consumption
    pub note: Option<Vec<Annotation>>,
}
