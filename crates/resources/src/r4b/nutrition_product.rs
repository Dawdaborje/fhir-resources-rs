//! FHIR R4B NutritionProduct Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The product's nutritional information expressed by the nutrients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductNutrient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The (relevant) nutrients in the product
    pub item: Option<CodeableReference>,

    /// The amount of nutrient expressed in one or more units: X per pack / per serving / per dose
    pub amount: Option<Vec<Ratio>>,
}

/// Ingredients contained in this product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The ingredient contained in the product
    pub item: CodeableReference,

    /// The amount of ingredient that is in the product
    pub amount: Option<Vec<Ratio>>,
}

/// Specifies descriptive properties of the nutrition product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductProductCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code specifying the type of characteristic
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The value of the characteristic
    pub value: serde_json::Value,
}

/// One or several physical instances or occurrences of the nutrition product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProductInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The amount of items or instances
    pub quantity: Option<Quantity>,

    /// The identifier for the physical instance, typically a serial number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The identification of the batch or lot of the product
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// The expiry date or date and time for the product
    pub expiry: Option<String>,

    /// The date until which the product is expected to be good for consumption
    #[serde(rename = "useBy")]
    pub use_by: Option<String>,
}

/// A food or fluid product that is consumed by patients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NutritionProduct {
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

    /// active | inactive | entered-in-error
    pub status: String,

    /// A category or class of the nutrition product (halal, kosher, gluten free, vegan, etc)
    pub category: Option<Vec<CodeableConcept>>,

    /// A code designating a specific type of nutritional product
    pub code: Option<CodeableConcept>,

    /// Manufacturer, representative or officially responsible for the product
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// The product's nutritional information expressed by the nutrients
    pub nutrient: Option<Vec<NutritionProductNutrient>>,

    /// Ingredients contained in this product
    pub ingredient: Option<Vec<NutritionProductIngredient>>,

    /// Known or suspected allergens that are a part of this product
    #[serde(rename = "knownAllergen")]
    pub known_allergen: Option<Vec<CodeableReference>>,

    /// Specifies descriptive properties of the nutrition product
    #[serde(rename = "productCharacteristic")]
    pub product_characteristic: Option<Vec<NutritionProductProductCharacteristic>>,

    /// One or several physical instances or occurrences of the nutrition product
    pub instance: Option<NutritionProductInstance>,

    /// Comments made about the product
    pub note: Option<Vec<Annotation>>,
}
