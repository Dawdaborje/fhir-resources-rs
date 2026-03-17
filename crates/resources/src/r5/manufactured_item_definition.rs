//! FHIR R5 ManufacturedItemDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<String>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,

    #[serde(rename = "valueReference")]
    pub value_reference: Option<Box<Reference>>,
}

/// Physical parts of the manufactured item, that it is intrisically made from. This is distinct from the ingredients that are part of its chemical makeup
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Defining type of the component e.g. shell, layer, ink
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The function of this component within the item e.g. delivers active ingredient, masks taste
    pub function: Option<Vec<CodeableConcept>>,

    /// The measurable amount of total quantity of all substances in the component, expressable in different ways (e.g. by mass or volume)
    pub amount: Option<Vec<Quantity>>,

    /// A reference to a constituent of the manufactured item as a whole, linked here so that its component location within the item can be indicated. This not where the item's ingredient are primarily sta...
    pub constituent: Option<Vec<ManufacturedItemDefinitionComponentConstituent>>,

    /// General characteristics of this component
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,

    /// A component that this component contains or is made from
    pub component: Option<Vec<ManufacturedItemDefinitionComponent>>,
}

/// A reference to a constituent of the manufactured item as a whole, linked here so that its component location within the item can be indicated. This not where the item's ingredient are primarily sta...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManufacturedItemDefinitionComponentConstituent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The measurable amount of the substance, expressable in different ways (e.g. by mass or volume)
    pub amount: Option<Vec<Quantity>>,

    /// The physical location of the constituent/ingredient within the component
    pub location: Option<Vec<CodeableConcept>>,

    /// The function of this constituent within the component e.g. binder
    pub function: Option<Vec<CodeableConcept>>,

    /// The ingredient that is the constituent of the given component
    #[serde(rename = "hasIngredient")]
    pub has_ingredient: Option<Vec<CodeableReference>>,
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

    /// A descriptive name applied to this item
    pub name: Option<String>,

    /// Dose form as manufactured (before any necessary transformation)
    #[serde(rename = "manufacturedDoseForm")]
    pub manufactured_dose_form: CodeableConcept,

    /// The “real-world” units in which the quantity of the item is described
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,

    /// Manufacturer of the item, one of several possible
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,

    /// The ingredients of this manufactured item. Only needed if these are not specified by incoming references from the Ingredient resource
    pub ingredient: Option<Vec<CodeableConcept>>,

    /// General characteristics of this item
    pub property: Option<Vec<ManufacturedItemDefinitionProperty>>,

    /// Physical parts of the manufactured item, that it is intrisically made from. This is distinct from the ingredients that are part of its chemical makeup
    pub component: Option<Vec<ManufacturedItemDefinitionComponent>>,
}
