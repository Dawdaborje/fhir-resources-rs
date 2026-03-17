//! FHIR R5 PackagedProductDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The legal status of supply of the packaged item as classified by the regulator
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionLegalStatusOfSupply {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual status of supply. In what situation this package type may be supplied for use
    pub code: Option<CodeableConcept>,

    /// The place where the legal status of supply applies
    pub jurisdiction: Option<CodeableConcept>,
}

/// A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// An identifier that is specific to this particular part of the packaging. Including possibly a Data Carrier Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The physical type of the container of the items
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than the packaging itself (e.g. a bottle or vial)
    #[serde(rename = "componentPart")]
    pub component_part: Option<bool>,

    /// The quantity of this level of packaging in the package that contains it (with the outermost level being 1)
    pub quantity: Option<i32>,

    /// Material type of the package item
    pub material: Option<Vec<CodeableConcept>>,

    /// A possible alternate material for this part of the packaging, that is allowed to be used instead of the usual material
    #[serde(rename = "alternateMaterial")]
    pub alternate_material: Option<Vec<CodeableConcept>>,

    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,

    /// Manufacturer of this packaging item (multiple means these are all potential manufacturers)
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// General characteristics of this item
    pub property: Option<Vec<PackagedProductDefinitionPackagingProperty>>,

    /// The item(s) within the packaging
    #[serde(rename = "containedItem")]
    pub contained_item: Option<Vec<PackagedProductDefinitionPackagingContainedItem>>,

    /// Allows containers (and parts of containers) within containers, still as a part of single packaged product
    pub packaging: Option<Vec<PackagedProductDefinitionPackaging>>,
}

/// General characteristics of this item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackagingProperty {
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
    pub value: Option<serde_json::Value>,
}

/// The item(s) within the packaging
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinitionPackagingContainedItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual item(s) of medication, as manufactured, or a device, or other medically related item (food, biologicals, raw materials, medical fluids, gases etc.), as contained in the package
    pub item: CodeableReference,

    /// The number of this type of item within this packaging or for continuous items such as liquids it is the quantity (for example 25ml). See also PackagedProductDefinition.containedItemQuantity (especi...
    pub amount: Option<Quantity>,
}

/// A medically related item or items, in a container or package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagedProductDefinition {
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

    /// A unique identifier for this package as whole - not for the content of the package
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// A name for this package. Typically as listed in a drug formulary, catalogue, inventory etc
    pub name: Option<String>,

    /// A high level category e.g. medicinal product, raw material, shipping container etc
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The product that this is a pack for
    #[serde(rename = "packageFor")]
    pub package_for: Option<Vec<Box<Reference>>>,

    /// The status within the lifecycle of this item. High level - not intended to duplicate details elsewhere e.g. legal status, or authorization/marketing status
    pub status: Option<CodeableConcept>,

    /// The date at which the given status became applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// A total of the complete count of contained items of a particular type/form, independent of sub-packaging or organization. This can be considered as the pack size. See also packaging.containedItem.a...
    #[serde(rename = "containedItemQuantity")]
    pub contained_item_quantity: Option<Vec<Quantity>>,

    /// Textual description. Note that this is not the name of the package or product
    pub description: Option<String>,

    /// The legal status of supply of the packaged item as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<Vec<PackagedProductDefinitionLegalStatusOfSupply>>,

    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,

    /// Identifies if the drug product is supplied with another item such as a diluent or adjuvant
    #[serde(rename = "copackagedIndicator")]
    pub copackaged_indicator: Option<bool>,

    /// Manufacturer of this package type (multiple means these are all possible manufacturers)
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// Additional information or supporting documentation about the packaged product
    #[serde(rename = "attachedDocument")]
    pub attached_document: Option<Vec<Box<Reference>>>,

    /// A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap
    pub packaging: Option<PackagedProductDefinitionPackaging>,

    /// Allows the key features to be recorded, such as "hospital pack", "nurse prescribable"
    pub characteristic: Option<Vec<PackagedProductDefinitionPackagingProperty>>,
}
