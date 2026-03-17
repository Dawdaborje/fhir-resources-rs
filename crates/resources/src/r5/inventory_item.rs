//! FHIR R5 InventoryItem Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The item name(s) - the brand name, or common name, functional name, generic name or others
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'
    #[serde(rename = "nameType")]
    pub name_type: Coding,

    /// The language used to express the item name
    pub language: String,

    /// The name or designation of the item
    pub name: String,
}

/// Organization(s) responsible for the product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemResponsibleOrganization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The role of the organization e.g. manufacturer, distributor, or other
    pub role: CodeableConcept,

    /// An organization that is associated with the item
    pub organization: Box<Reference>,
}

/// Descriptive characteristics of the item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemDescription {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The language that is used in the item description
    pub language: Option<String>,

    /// Textual description of the item
    pub description: Option<String>,
}

/// Association with other items or products
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemAssociation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of association between the device and the other item
    #[serde(rename = "associationType")]
    pub association_type: CodeableConcept,

    /// The related item or product
    #[serde(rename = "relatedItem")]
    pub related_item: Box<Reference>,

    /// The quantity of the product in this product
    pub quantity: Ratio,
}

/// Characteristic of the item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The characteristic that is being defined
    #[serde(rename = "characteristicType")]
    pub characteristic_type: CodeableConcept,

    /// The value of the attribute
    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueUrl")]
    pub value_url: String,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,

    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,

    #[serde(rename = "valueAddress")]
    pub value_address: Address,

    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,
}

/// Instances or occurrences of the product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The identifier for the physical instance, typically a serial number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The lot or batch number of the item
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// The expiry date or date and time for the product
    pub expiry: Option<String>,

    /// The subject that the item is associated with
    pub subject: Option<Box<Reference>>,

    /// The location that the item is associated with
    pub location: Option<Box<Reference>>,
}

/// functional description of an inventory item used in inventory and supply-related workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
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

    /// Business identifier for the inventory item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | inactive | entered-in-error | unknown
    pub status: String,

    /// Category or class of the item
    pub category: Option<Vec<CodeableConcept>>,

    /// Code designating the specific type of item
    pub code: Option<Vec<CodeableConcept>>,

    /// The item name(s) - the brand name, or common name, functional name, generic name or others
    pub name: Option<Vec<InventoryItemName>>,

    /// Organization(s) responsible for the product
    #[serde(rename = "responsibleOrganization")]
    pub responsible_organization: Option<Vec<InventoryItemResponsibleOrganization>>,

    /// Descriptive characteristics of the item
    pub description: Option<InventoryItemDescription>,

    /// The usage status like recalled, in use, discarded
    #[serde(rename = "inventoryStatus")]
    pub inventory_status: Option<Vec<CodeableConcept>>,

    /// The base unit of measure - the unit in which the product is used or counted
    #[serde(rename = "baseUnit")]
    pub base_unit: Option<CodeableConcept>,

    /// Net content or amount present in the item
    #[serde(rename = "netContent")]
    pub net_content: Option<Quantity>,

    /// Association with other items or products
    pub association: Option<Vec<InventoryItemAssociation>>,

    /// Characteristic of the item
    pub characteristic: Option<Vec<InventoryItemCharacteristic>>,

    /// Instances or occurrences of the product
    pub instance: Option<InventoryItemInstance>,

    /// Link to a product resource used in clinical workflows
    #[serde(rename = "productReference")]
    pub product_reference: Option<Box<Reference>>,
}
