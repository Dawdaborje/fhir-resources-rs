//! FHIR R4B DeviceUseStatement Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A record of a device being used by a patient where the record is the result of a report from the patient or another clinician.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceUseStatement {
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

    /// active | completed | entered-in-error +
    pub status: String,

    /// Patient using device
    pub subject: Box<Reference>,

    /// Supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// How often the device was used
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,

    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,

    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<String>,

    /// When statement was recorded
    #[serde(rename = "recordedOn")]
    pub recorded_on: Option<String>,

    /// Who made the statement
    pub source: Option<Box<Reference>>,

    /// Reference to device used
    pub device: Box<Reference>,

    /// Why device was used
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why was DeviceUseStatement performed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Target body site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Addition details (comments, instructions)
    pub note: Option<Vec<Annotation>>,
}
