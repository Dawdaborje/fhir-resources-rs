//! FHIR R5 DeviceUsage Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// How device is being used
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUsageAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// always | never | sometimes
    pub code: CodeableConcept,

    /// lost | stolen | prescribed | broken | burned | forgot
    pub reason: Vec<CodeableConcept>,
}

/// A record of a device being used by a patient where the record is the result of a report from the patient or a clinician.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUsage {
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

    /// External identifier for this record
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// active | completed | not-done | entered-in-error +
    pub status: String,

    /// The category of the statement - classifying how the statement is made
    pub category: Option<Vec<CodeableConcept>>,

    /// Patient using device
    pub patient: Box<Reference>,

    /// Supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// The encounter or episode of care that establishes the context for this device use statement
    pub context: Option<Box<Reference>>,

    /// How often the device was used
    pub timing: Option<serde_json::Value>,

    /// When the statement was made (and recorded)
    #[serde(rename = "dateAsserted")]
    pub date_asserted: Option<String>,

    /// The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement
    #[serde(rename = "usageStatus")]
    pub usage_status: Option<CodeableConcept>,

    /// The reason for asserting the usage status - for example forgot, lost, stolen, broken
    #[serde(rename = "usageReason")]
    pub usage_reason: Option<Vec<CodeableConcept>>,

    /// How device is being used
    pub adherence: Option<DeviceUsageAdherence>,

    /// Who made the statement
    #[serde(rename = "informationSource")]
    pub information_source: Option<Box<Reference>>,

    /// Code or Reference to device used
    pub device: CodeableReference,

    /// Why device was used
    pub reason: Option<Vec<CodeableReference>>,

    /// Target body site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableReference>,

    /// Addition details (comments, instructions)
    pub note: Option<Vec<Annotation>>,
}
