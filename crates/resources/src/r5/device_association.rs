//! FHIR R5 DeviceAssociation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The details about the device when it is in use to describe its operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAssociationOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Device operational condition
    pub status: CodeableConcept,

    /// The individual performing the action enabled by the device
    pub operator: Option<Vec<Box<Reference>>>,

    /// Begin and end dates and times for the device's operation
    pub period: Option<Period>,
}

/// A record of association of a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceAssociation {
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

    /// Reference to the devices associated with the patient or group
    pub device: Box<Reference>,

    /// Describes the relationship between the device and subject
    pub category: Option<Vec<CodeableConcept>>,

    /// implanted | explanted | attached | entered-in-error | unknown
    pub status: CodeableConcept,

    /// The reasons given for the current association status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,

    /// The individual, group of individuals or device that the device is on or associated with
    pub subject: Option<Box<Reference>>,

    /// Current anatomical location of the device in/on subject
    #[serde(rename = "bodyStructure")]
    pub body_structure: Option<Box<Reference>>,

    /// Begin and end dates and times for the device association
    pub period: Option<Period>,

    /// The details about the device when it is in use to describe its operation
    pub operation: Option<Vec<DeviceAssociationOperation>>,
}
