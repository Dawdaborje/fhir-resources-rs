//! FHIR R5 DeviceDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: String,

    /// The organization that assigns the identifier algorithm
    pub issuer: String,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: String,

    /// Indicates whether and when the device is available on the market
    #[serde(rename = "marketDistribution")]
    pub market_distribution: Option<Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistribution>>,
}

/// Indicates whether and when the device is available on the market
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionUdiDeviceIdentifierMarketDistribution {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Begin and end dates for the commercial distribution of the device
    #[serde(rename = "marketPeriod")]
    pub market_period: Period,

    /// National state or territory where the device is commercialized
    #[serde(rename = "subJurisdiction")]
    pub sub_jurisdiction: String,
}

/// Regulatory identifier(s) associated with this device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionRegulatoryIdentifier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// basic | master | license
    #[serde(rename = "type")]
    pub r#type: String,

    /// The identifier itself
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: String,

    /// The organization that issued this identifier
    pub issuer: String,

    /// The jurisdiction to which the deviceIdentifier applies
    pub jurisdiction: String,
}

/// The name or names of the device as given by the manufacturer
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

    /// A name that is used to refer to the device
    pub name: String,

    /// registered-name | user-friendly-name | patient-reported-name
    #[serde(rename = "type")]
    pub r#type: String,
}

/// What kind of device or device system this is
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionClassification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A classification or risk class of the device model
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Further information qualifying this classification of the device model
    pub justification: Option<Vec<RelatedArtifact>>,
}

/// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Describes the common type of the standard, specification, or formal guidance
    pub category: Option<CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the device adheres to the Device Specification type
    pub specification: CodeableConcept,

    /// The specific form or variant of the standard, specification or formal guidance
    pub version: Option<Vec<String>>,

    /// Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance
    pub source: Option<Vec<RelatedArtifact>>,
}

/// A device, part of the current one
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionHasPart {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to the part
    pub reference: Box<Reference>,

    /// Number of occurrences of the part
    pub count: Option<i32>,
}

/// Information about the packaging of the device, i.e. how the device is packaged
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackaging {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Business identifier of the packaged medication
    pub identifier: Option<Box<Identifier>>,

    /// A code that defines the specific type of packaging
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The number of items contained in the package (devices or sub-packages)
    pub count: Option<i32>,

    /// An organization that distributes the packaged device
    pub distributor: Option<Vec<DeviceDefinitionPackagingDistributor>>,

    /// Unique Device Identifier (UDI) Barcode string on the packaging
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>>,

    /// Allows packages within packages
    pub packaging: Option<Vec<DeviceDefinitionPackaging>>,
}

/// An organization that distributes the packaged device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionPackagingDistributor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Distributor's human-readable name
    pub name: Option<String>,

    /// Distributor as an Organization resource
    #[serde(rename = "organizationReference")]
    pub organization_reference: Option<Vec<Box<Reference>>>,
}

/// The version of the device or software
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionVersion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of the device version, e.g. manufacturer, approved, internal
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The hardware or software module of the device to which the version applies
    pub component: Option<Box<Identifier>>,

    /// The version text
    pub value: String,
}

/// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
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

    /// Code that specifies the property being represented
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Value of the property
    pub value: serde_json::Value,
}

/// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type indicates the relationship of the related device to the device instance
    pub relation: Coding,

    /// A reference to the linked device
    #[serde(rename = "relatedDevice")]
    pub related_device: CodeableReference,
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

    /// A relevant substance that the device contains, may contain, or is made of
    pub substance: CodeableConcept,

    /// Indicates an alternative material of the device
    pub alternate: Option<bool>,

    /// Whether the substance is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<bool>,
}

/// Information aimed at providing directions for the usage of this model of device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionGuideline {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The circumstances that form the setting for using the device
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Detailed written and visual directions for the user on how to use the device
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<String>,

    /// A source of information or reference for this guideline
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// A clinical condition for which the device was designed to be used
    pub indication: Option<Vec<CodeableConcept>>,

    /// A specific situation when a device should not be used because it may cause harm
    pub contraindication: Option<Vec<CodeableConcept>>,

    /// Specific hazard alert information that a user needs to know before using the device
    pub warning: Option<Vec<CodeableConcept>>,

    /// A description of the general purpose or medical use of the device or its function
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<String>,
}

/// Tracking of latest field safety corrective action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionCorrectiveAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether the corrective action was a recall
    pub recall: bool,

    /// model | lot-numbers | serial-numbers
    pub scope: Option<String>,

    /// Start and end dates of the corrective action
    pub period: Period,
}

/// Billing code or reference associated with the device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDefinitionChargeItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The code or reference for the charge item
    #[serde(rename = "chargeItemCode")]
    pub charge_item_code: CodeableReference,

    /// Coefficient applicable to the billing code
    pub count: Quantity,

    /// A specific time period in which this charge item applies
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The context to which this charge item applies
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,
}

/// This is a specialized resource that defines the characteristics and capabilities of a device.
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

    /// Additional information to describe the device
    pub description: Option<String>,

    /// Instance identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiDeviceIdentifier")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifier>>,

    /// Regulatory identifier(s) associated with this device
    #[serde(rename = "regulatoryIdentifier")]
    pub regulatory_identifier: Option<Vec<DeviceDefinitionRegulatoryIdentifier>>,

    /// The part number or catalog number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,

    /// Name of device manufacturer
    pub manufacturer: Option<Box<Reference>>,

    /// The name or names of the device as given by the manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDefinitionDeviceName>>,

    /// The catalog or model number for the device for example as defined by the manufacturer
    #[serde(rename = "modelNumber")]
    pub model_number: Option<String>,

    /// What kind of device or device system this is
    pub classification: Option<Vec<DeviceDefinitionClassification>>,

    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[serde(rename = "conformsTo")]
    pub conforms_to: Option<Vec<DeviceDefinitionConformsTo>>,

    /// A device, part of the current one
    #[serde(rename = "hasPart")]
    pub has_part: Option<Vec<DeviceDefinitionHasPart>>,

    /// Information about the packaging of the device, i.e. how the device is packaged
    pub packaging: Option<Vec<DeviceDefinitionPackaging>>,

    /// The version of the device or software
    pub version: Option<Vec<DeviceDefinitionVersion>>,

    /// Safety characteristics of the device
    pub safety: Option<Vec<CodeableConcept>>,

    /// Shelf Life and storage information
    #[serde(rename = "shelfLifeStorage")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,

    /// Language code for the human-readable text strings produced by the device (all supported)
    #[serde(rename = "languageCode")]
    pub language_code: Option<Vec<CodeableConcept>>,

    /// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
    pub property: Option<Vec<DeviceDefinitionProperty>>,

    /// Organization responsible for device
    pub owner: Option<Box<Reference>>,

    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,

    /// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
    pub link: Option<Vec<DeviceDefinitionLink>>,

    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,

    /// A substance used to create the material(s) of which the device is made
    pub material: Option<Vec<DeviceDefinitionMaterial>>,

    /// lot-number | manufactured-date | serial-number | expiration-date | biological-source | software-version
    #[serde(rename = "productionIdentifierInUDI")]
    pub production_identifier_in_u_d_i: Option<Vec<String>>,

    /// Information aimed at providing directions for the usage of this model of device
    pub guideline: Option<DeviceDefinitionGuideline>,

    /// Tracking of latest field safety corrective action
    #[serde(rename = "correctiveAction")]
    pub corrective_action: Option<DeviceDefinitionCorrectiveAction>,

    /// Billing code or reference associated with the device
    #[serde(rename = "chargeItem")]
    pub charge_item: Option<Vec<DeviceDefinitionChargeItem>>,
}
