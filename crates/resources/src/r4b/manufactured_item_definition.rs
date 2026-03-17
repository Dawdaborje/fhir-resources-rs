//! FHIR R4B ManufacturedItemDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// General characteristics of this item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A code expressing the type of characteristic
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// A value for the characteristic
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,
}

/// The definition and characteristics of a medicinal manufactured item, such as a tablet or capsule, as contained in a packaged medicinal product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinition {
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

    /// Unique identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// Dose form as manufactured (before any necessary transformation)
    #[serde(rename = "manufacturedDoseForm")]
    pub manufactured_dose_form: CodeableConcept,

    /// The “real world” units in which the quantity of the item is described
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,

    /// Manufacturer of the item (Note that this should be named "manufacturer" but it currently causes technical issues)
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// The ingredients of this manufactured item. Only needed if these are not specified by incoming references from the Ingredient resource
    pub ingredient: Option<Vec<CodeableConcept>>,

    /// General characteristics of this item
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,
}
