//! FHIR R4B Device Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Unique Device Identifier (UDI) Barcode string
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUdiCarrier {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Mandatory fixed portion of UDI
    #[serde(rename = "deviceIdentifier")]
    pub device_identifier: Option<String>,

    /// UDI Issuing Organization
    pub issuer: Option<String>,

    /// Regional UDI authority
    pub jurisdiction: Option<String>,

    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_a_i_d_c: Option<String>,

    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_h_r_f: Option<String>,

    /// barcode | rfid | manual +
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,
}

/// The name of the device as given by the manufacturer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The name that identifies the device
    pub name: String,

    /// udi-label-name | user-friendly-name | patient-reported-name | manufacturer-name | model-name | other
    #[serde(rename = "type")]
    pub r#type: String,
}

/// The capabilities supported on a device, the standards to which the device conforms for a particular purpose, and used for the communication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSpecialization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The standard that is used to operate and communicate
    #[serde(rename = "systemType")]
    pub system_type: CodeableConcept,

    /// The version of the standard that is used to operate and communicate
    pub version: Option<String>,
}

/// The actual design of the device or software version running on the device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceVersion {
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

    /// A single component of the device version
    pub component: Option<Box<Identifier>>,

    /// The version text
    pub value: String,
}

/// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceProperty {
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

/// A type of a manufactured item that is used in the provision of healthcare without being substantially changed through that activity. The device may be a medical or non-medical device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
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

    /// The reference to the definition for the device
    pub definition: Option<Box<Reference>>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiCarrier")]
    pub udi_carrier: Option<Vec<DeviceUdiCarrier>>,

    /// active | inactive | entered-in-error | unknown
    pub status: Option<String>,

    /// online | paused | standby | offline | not-ready | transduc-discon | hw-discon | off
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,

    /// The distinct identification string
    #[serde(rename = "distinctIdentifier")]
    pub distinct_identifier: Option<String>,

    /// Name of device manufacturer
    pub manufacturer: Option<String>,

    /// Date when the device was made
    #[serde(rename = "manufactureDate")]
    pub manufacture_date: Option<String>,

    /// Date and time of expiry of this device (if applicable)
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,

    /// Lot number of manufacture
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// Serial number assigned by the manufacturer
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,

    /// The name of the device as given by the manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<Vec<DeviceDeviceName>>,

    /// The manufacturer's model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<String>,

    /// The part number or catalog number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,

    /// The kind or type of device
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The capabilities supported on a device, the standards to which the device conforms for a particular purpose, and used for the communication
    pub specialization: Option<Vec<DeviceSpecialization>>,

    /// The actual design of the device or software version running on the device
    pub version: Option<Vec<DeviceVersion>>,

    /// The actual configuration settings of a device as it actually operates, e.g., regulation status, time properties
    pub property: Option<Vec<DeviceProperty>>,

    /// Patient to whom Device is affixed
    pub patient: Option<Box<Reference>>,

    /// Organization responsible for device
    pub owner: Option<Box<Reference>>,

    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,

    /// Where the device is found
    pub location: Option<Box<Reference>>,

    /// Network address to contact device
    pub url: Option<String>,

    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,

    /// Safety Characteristics of Device
    pub safety: Option<Vec<CodeableConcept>>,

    /// The device that this device is attached to or is part of
    pub parent: Option<Box<Reference>>,
}
