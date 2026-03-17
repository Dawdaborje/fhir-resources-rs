//! FHIR R4 MedicinalProductPackaged Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Batch numbering
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackagedBatchIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A number appearing on the outer packaging of a specific batch
    #[serde(rename = "outerPackaging")]
    pub outer_packaging: Box<Identifier>,

    /// A number appearing on the immediate packaging (and not the outer packaging)
    #[serde(rename = "immediatePackaging")]
    pub immediate_packaging: Option<Box<Identifier>>,
}

/// A packaging item, as a contained for medicine, possibly with other packaging items within
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackagedPackageItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Including possibly Data Carrier Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The physical type of the container of the medicine
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The quantity of this package in the medicinal product, at the current level of packaging. The outermost is always 1
    pub quantity: Quantity,

    /// Material type of the package item
    pub material: Option<Vec<CodeableConcept>>,

    /// A possible alternate material for the packaging
    #[serde(rename = "alternateMaterial")]
    pub alternate_material: Option<Vec<CodeableConcept>>,

    /// A device accompanying a medicinal product
    pub device: Option<Vec<Box<Reference>>>,

    /// The manufactured item as contained in the packaged medicinal product
    #[serde(rename = "manufacturedItem")]
    pub manufactured_item: Option<Vec<Box<Reference>>>,

    /// Allows containers within containers
    #[serde(rename = "packageItem")]
    pub package_item: Option<Vec<MedicinalProductPackagedPackageItem>>,

    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,

    /// Other codeable characteristics
    #[serde(rename = "otherCharacteristics")]
    pub other_characteristics: Option<Vec<CodeableConcept>>,

    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,

    /// Manufacturer of this Package Item
    pub manufacturer: Option<Vec<Box<Reference>>>,
}

/// A medicinal product in a container or package.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPackaged {
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

    /// The product with this is a pack for
    pub subject: Option<Vec<Box<Reference>>>,

    /// Textual description
    pub description: Option<String>,

    /// The legal status of supply of the medicinal product as classified by the regulator
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,

    /// Marketing information
    #[serde(rename = "marketingStatus")]
    pub marketing_status: Option<Vec<MarketingStatus>>,

    /// Manufacturer of this Package Item
    #[serde(rename = "marketingAuthorization")]
    pub marketing_authorization: Option<Box<Reference>>,

    /// Manufacturer of this Package Item
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// Batch numbering
    #[serde(rename = "batchIdentifier")]
    pub batch_identifier: Option<Vec<MedicinalProductPackagedBatchIdentifier>>,

    /// A packaging item, as a contained for medicine, possibly with other packaging items within
    #[serde(rename = "packageItem")]
    pub package_item: Vec<MedicinalProductPackagedPackageItem>,
}
