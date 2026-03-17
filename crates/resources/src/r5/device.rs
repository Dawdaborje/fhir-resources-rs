//! FHIR R5 Device Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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
    pub device_identifier: String,

    /// UDI Issuing Organization
    pub issuer: String,

    /// Regional UDI authority
    pub jurisdiction: Option<String>,

    /// UDI Machine Readable Barcode String
    #[serde(rename = "carrierAIDC")]
    pub carrier_a_i_d_c: Option<String>,

    /// UDI Human Readable Barcode String
    #[serde(rename = "carrierHRF")]
    pub carrier_h_r_f: Option<String>,

    /// barcode | rfid | manual | card | self-reported | electronic-transmission | unknown
    #[serde(rename = "entryType")]
    pub entry_type: Option<String>,
}

/// The name or names of the device as known to the manufacturer and/or patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceName {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The term that names the device
    pub value: String,

    /// registered-name | user-friendly-name | patient-reported-name
    #[serde(rename = "type")]
    pub r#type: String,

    /// The preferred device name
    pub display: Option<bool>,
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

    /// The hardware or software module of the device to which the version applies
    pub component: Option<Box<Identifier>>,

    /// The date the version was installed on the device
    #[serde(rename = "installDate")]
    pub install_date: Option<String>,

    /// The version text
    pub value: String,
}

/// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConformsTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Describes the common type of the standard, specification, or formal guidance. communication | performance | measurement
    pub category: Option<CodeableConcept>,

    /// Identifies the standard, specification, or formal guidance that the device adheres to
    pub specification: CodeableConcept,

    /// Specific form or variant of the standard
    pub version: Option<String>,
}

/// Inherent, essentially fixed, characteristics of the device. e.g., time properties, size, material, etc.
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

    /// Code that specifies the property being represented
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Value of the property
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,
}

/// This resource describes the properties (regulated, has real time clock, etc.), adminstrative (manufacturer name, model number, serial number, firmware, etc.), and type (knee replacement, blood pres...
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

    /// The name used to display by default when the device is referenced
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,

    /// The reference to the definition for the device
    pub definition: Option<CodeableReference>,

    /// Unique Device Identifier (UDI) Barcode string
    #[serde(rename = "udiCarrier")]
    pub udi_carrier: Option<Vec<DeviceUdiCarrier>>,

    /// active | inactive | entered-in-error
    pub status: Option<String>,

    /// lost | damaged | destroyed | available
    #[serde(rename = "availabilityStatus")]
    pub availability_status: Option<CodeableConcept>,

    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Box<Identifier>>,

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

    /// The name or names of the device as known to the manufacturer and/or patient
    pub name: Option<Vec<DeviceName>>,

    /// The manufacturer's model number for the device
    #[serde(rename = "modelNumber")]
    pub model_number: Option<String>,

    /// The part number or catalog number of the device
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,

    /// Indicates a high-level grouping of the device
    pub category: Option<Vec<CodeableConcept>>,

    /// The kind or type of device
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// The actual design of the device or software version running on the device
    pub version: Option<Vec<DeviceVersion>>,

    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[serde(rename = "conformsTo")]
    pub conforms_to: Option<Vec<DeviceConformsTo>>,

    /// Inherent, essentially fixed, characteristics of the device. e.g., time properties, size, material, etc.
    pub property: Option<Vec<DeviceProperty>>,

    /// The designated condition for performing a task
    pub mode: Option<CodeableConcept>,

    /// The series of occurrences that repeats during the operation of the device
    pub cycle: Option<Count>,

    /// A measurement of time during the device's operation (e.g., days, hours, mins, etc.)
    pub duration: Option<Duration>,

    /// Organization responsible for device
    pub owner: Option<Box<Reference>>,

    /// Details for human/organization for support
    pub contact: Option<Vec<ContactPoint>>,

    /// Where the device is found
    pub location: Option<Box<Reference>>,

    /// Network address to contact device
    pub url: Option<String>,

    /// Technical endpoints providing access to electronic services provided by the device
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// Linked device acting as a communication/data collector, translator or controller
    pub gateway: Option<Vec<CodeableReference>>,

    /// Device notes and comments
    pub note: Option<Vec<Annotation>>,

    /// Safety Characteristics of Device
    pub safety: Option<Vec<CodeableConcept>>,

    /// The higher level or encompassing device that this device is a logical part of
    pub parent: Option<Box<Reference>>,
}
