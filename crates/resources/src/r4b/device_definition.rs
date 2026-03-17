//! FHIR R4B DeviceDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Unique Device Identifier (UDI) Barcode string
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionUdiDeviceIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdication porvided in the DeviceDefinition.udiDeviceIdentifier
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: String,

    /// The organization that assigns the identifier algorithm
    pub issuer: String,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: String,
}

/// A name given to the device to identify it
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionDeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The name of the device
    pub name: String,

    /// udi-label-name | user-friendly-name | patient-reported-name | manufacturer-name | model-name | other
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The capabilities supported on a device, the standards to which the device conforms for a particular purpose, and used for the communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionSpecialization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The standard that is used to operate and communicate
    #[serde(rename = "systemType")]
    pub system_type: String,

    /// The version of the standard that is used to operate and communicate
    pub version: Option<String>,
}

/// Device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCapability {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of capability
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Description of capability
    pub description: Option<Vec<CodeableConcept>>,
}

/// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that specifies the property DeviceDefinitionPropetyCode (Extensible)
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Property value as a quantity
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Vec<Quantity>>,

    /// Property value as a code, e.g., NTP4 (synced to NTP)
    #[serde(rename = "valueCode")]
    pub value_code: Option<Vec<CodeableConcept>>,
}

/// A substance used to create the material(s) of which the device is made
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionMaterial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The substance
    pub substance: CodeableConcept,

    /// Indicates an alternative material of the device
    pub alternate: Option<bool>,

    /// Whether the substance is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<bool>,
}

/// The characteristics, operational status and capabilities of a medical-related component of a medical device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinition {
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

    /// Instance identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>>,

    /// Name of device manufacturer
    pub manufacturer: Option<serde_json::Value>,

    /// A name given to the device to identify it
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDefinitionDeviceName>>,

    /// The model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<String>,

    /// What kind of device or device system this is
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The capabilities supported on a device, the standards to which the device conforms for a particular purpose, and used for the communication
    pub specialization: Option<Vec<DeviceDefinitionSpecialization>>,

    /// Available versions
    pub version: Option<Vec<String>>,

    /// Safety characteristics of the device
    pub safety: Option<Vec<CodeableConcept>>,

    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,

    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,

    /// Language code for the human-readable text strings produced by the device (all supported)
    #[serde(rename = "languageCode")]
    pub language_code: Option<Vec<CodeableConcept>>,

    /// Device capabilities
    pub capability: Option<Vec<DeviceDefinitionCapability>>,

    /// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
    pub property: Option<Vec<DeviceDefinitionProperty>>,

    /// Organization responsible for device
    pub owner: Option<Box<Reference>>,

    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,

    /// Network address to contact device
    pub url: Option<String>,

    /// Access to on-line information
    #[serde(rename = "onlineInformation")]
    pub online_information: Option<String>,

    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,

    /// The quantity of the device present in the packaging (e.g. the number of devices present in a pack, or the number of devices in the same package of the medicinal product)
    pub quantity: Option<Quantity>,

    /// The parent device it can be part of
    #[serde(rename = "parentDevice")]
    pub parent_device: Option<Box<Reference>>,

    /// A substance used to create the material(s) of which the device is made
    pub material: Option<Vec<DeviceDefinitionMaterial>>,
}
