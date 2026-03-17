//! FHIR R4B NutritionOrder Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Oral diet components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDiet {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Scheduled frequency of diet
    pub schedule: Option<Vec<Timing>>,

    /// Required nutrient modifications
    pub nutrient: Option<Vec<NutritionOrderOralDietNutrient>>,

    /// Required texture modifications
    pub texture: Option<Vec<NutritionOrderOralDietTexture>>,

    /// The required consistency of fluids and liquids provided to the patient
    #[serde(rename = "fluidConsistencyType")]
    pub fluid_consistency_type: Option<Vec<CodeableConcept>>,

    /// Instructions or additional information about the oral diet
    pub instruction: Option<String>,
}

/// Required nutrient modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of nutrient that is being modified
    pub modifier: Option<CodeableConcept>,

    /// Quantity of the specified nutrient
    pub amount: Option<Quantity>,
}

/// Required texture modifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderOralDietTexture {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code to indicate how to alter the texture of the foods, e.g. pureed
    pub modifier: Option<CodeableConcept>,

    /// Concepts that are used to identify an entity that is ingested for nutritional purposes
    #[serde(rename = "foodType")]
    pub food_type: Option<CodeableConcept>,
}

/// Supplement components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderSupplement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of supplement product requested
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Product or brand name of the nutritional supplement
    #[serde(rename = "productName")]
    pub product_name: Option<String>,

    /// Scheduled frequency of supplement
    pub schedule: Option<Vec<Timing>>,

    /// Amount of the nutritional supplement
    pub quantity: Option<Quantity>,

    /// Instructions or additional information about the oral supplement
    pub instruction: Option<String>,
}

/// Enteral formula components
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormula {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of enteral or infant formula
    #[serde(rename = "baseFormulaType")]
    pub base_formula_type: Option<CodeableConcept>,

    /// Product or brand name of the enteral or infant formula
    #[serde(rename = "baseFormulaProductName")]
    pub base_formula_product_name: Option<String>,

    /// Type of modular component to add to the feeding
    #[serde(rename = "additiveType")]
    pub additive_type: Option<CodeableConcept>,

    /// Product or brand name of the modular additive
    #[serde(rename = "additiveProductName")]
    pub additive_product_name: Option<String>,

    /// Amount of energy per specified volume that is required
    #[serde(rename = "caloricDensity")]
    pub caloric_density: Option<Quantity>,

    /// How the formula should enter the patient's gastrointestinal tract
    #[serde(rename = "routeofAdministration")]
    pub routeof_administration: Option<CodeableConcept>,

    /// Formula feeding instruction as structured data
    pub administration: Option<Vec<NutritionOrderEnteralFormulaAdministration>>,

    /// Upper limit on formula volume per unit of time
    #[serde(rename = "maxVolumeToDeliver")]
    pub max_volume_to_deliver: Option<Quantity>,

    /// Formula feeding instructions expressed as text
    #[serde(rename = "administrationInstruction")]
    pub administration_instruction: Option<String>,
}

/// Formula feeding instruction as structured data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrderEnteralFormulaAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Scheduled frequency of enteral feeding
    pub schedule: Option<Timing>,

    /// The volume of formula to provide
    pub quantity: Option<Quantity>,

    /// Speed with which the formula is provided per period of time
    pub rate: Option<serde_json::Value>,
}

/// A request to supply a diet, formula feeding (enteral) or oral nutritional supplement to a patient/resident.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionOrder {
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

    /// Identifiers assigned to this order
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Instantiates protocol or definition
    pub instantiates: Option<Vec<String>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// The person who requires the diet, formula or nutritional supplement
    pub patient: Box<Reference>,

    /// The encounter associated with this nutrition order
    pub encounter: Option<Box<Reference>>,

    /// Date and time the nutrition order was requested
    #[serde(rename = "dateTime")]
    pub date_time: String,

    /// Who ordered the diet, formula or nutritional supplement
    pub orderer: Option<Box<Reference>>,

    /// List of the patient's food and nutrition-related allergies and intolerances
    #[serde(rename = "allergyIntolerance")]
    pub allergy_intolerance: Option<Vec<Box<Reference>>>,

    /// Order-specific modifier about the type of food that should be given
    #[serde(rename = "foodPreferenceModifier")]
    pub food_preference_modifier: Option<Vec<CodeableConcept>>,

    /// Order-specific modifier about the type of food that should not be given
    #[serde(rename = "excludeFoodModifier")]
    pub exclude_food_modifier: Option<Vec<CodeableConcept>>,

    /// Oral diet components
    #[serde(rename = "oralDiet")]
    pub oral_diet: Option<NutritionOrderOralDiet>,

    /// Supplement components
    pub supplement: Option<Vec<NutritionOrderSupplement>>,

    /// Enteral formula components
    #[serde(rename = "enteralFormula")]
    pub enteral_formula: Option<NutritionOrderEnteralFormula>,

    /// Comments
    pub note: Option<Vec<Annotation>>,
}
