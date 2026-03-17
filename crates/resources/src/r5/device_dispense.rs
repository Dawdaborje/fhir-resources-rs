//! FHIR R5 DeviceDispense Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who performed event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Who performed the dispense and what they did
    pub function: Option<CodeableConcept>,

    /// Individual who was performing
    pub actor: Box<Reference>,
}

/// Indicates that a device is to be or has been dispensed for a named person/patient. This includes a description of the product (supply) provided and the instructions for using the device.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDispense {
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

    /// Business identifier for this dispensation
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The order or request that this dispense is fulfilling
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// The bigger event that this dispense is a part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    pub status: String,

    /// Why a dispense was or was not performed
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableReference>,

    /// Type of device dispense
    pub category: Option<Vec<CodeableConcept>>,

    /// What device was supplied
    pub device: CodeableReference,

    /// Who the dispense is for
    pub subject: Box<Reference>,

    /// Who collected the device or where the medication was delivered
    pub receiver: Option<Box<Reference>>,

    /// Encounter associated with event
    pub encounter: Option<Box<Reference>>,

    /// Information that supports the dispensing of the device
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// Who performed event
    pub performer: Option<Vec<DeviceDispensePerformer>>,

    /// Where the dispense occurred
    pub location: Option<Box<Reference>>,

    /// Trial fill, partial fill, emergency fill, etc
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<Quantity>,

    /// When product was packaged and reviewed
    #[serde(rename = "preparedDate")]
    pub prepared_date: Option<String>,

    /// When product was given out
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<String>,

    /// Where the device was sent or should be sent
    pub destination: Option<Box<Reference>>,

    /// Information about the dispense
    pub note: Option<Vec<Annotation>>,

    /// Full representation of the usage instructions
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<String>,

    /// A list of relevant lifecycle events
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Box<Reference>>>,
}
