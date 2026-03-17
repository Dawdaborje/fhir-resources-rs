//! FHIR R5 NutritionProduct Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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
pub struct NutritionProductCharacteristic {
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
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: String,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,
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

    /// The identifier for the physical instance, typically a serial number or manufacturer number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The name for the specific product
    pub name: Option<String>,

    /// The identification of the batch or lot of the product
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// The expiry date or date and time for the product
    pub expiry: Option<String>,

    /// The date until which the product is expected to be good for consumption
    #[serde(rename = "useBy")]
    pub use_by: Option<String>,

    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Box<Identifier>>,
}

/// A food or supplement that is consumed by patients.
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

    /// A code that can identify the detailed nutrients and ingredients in a specific food product
    pub code: Option<CodeableConcept>,

    /// active | inactive | entered-in-error
    pub status: String,

    /// Broad product groups or categories used to classify the product, such as Legume and Legume Products, Beverages, or Beef Products
    pub category: Option<Vec<CodeableConcept>>,

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
    pub characteristic: Option<Vec<NutritionProductCharacteristic>>,

    /// One or several physical instances or occurrences of the nutrition product
    pub instance: Option<Vec<NutritionProductInstance>>,

    /// Comments made about the product
    pub note: Option<Vec<Annotation>>,
}
