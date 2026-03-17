//! FHIR R4B Media Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A photo, video, or audio recording acquired or used in healthcare. The actual content may be inline or provided by direct reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media {
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

    /// Identifier(s) for the image
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Procedure that caused this media to be created
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    pub status: String,

    /// Classification of media as image, video, or audio
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The type of acquisition equipment/process
    pub modality: Option<CodeableConcept>,

    /// Imaging view, e.g. Lateral or Antero-posterior
    pub view: Option<CodeableConcept>,

    /// Who/What this Media is a record of
    pub subject: Option<Box<Reference>>,

    /// Encounter associated with media
    pub encounter: Option<Box<Reference>>,

    /// When Media was collected
    pub created: Option<serde_json::Value>,

    /// Date/Time this version was made available
    pub issued: Option<String>,

    /// The person who generated the image
    pub operator: Option<Box<Reference>>,

    /// Why was event performed?
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Observed body part
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Name of the device/manufacturer
    #[serde(rename = "deviceName")]
    pub device_name: Option<String>,

    /// Observing Device
    pub device: Option<Box<Reference>>,

    /// Height of the image in pixels (photo/video)
    pub height: Option<i32>,

    /// Width of the image in pixels (photo/video)
    pub width: Option<i32>,

    /// Number of frames if > 1 (photo)
    pub frames: Option<i32>,

    /// Length in seconds (audio / video)
    pub duration: Option<f64>,

    /// Actual Media - reference or data
    pub content: Attachment,

    /// Comments made about the media
    pub note: Option<Vec<Annotation>>,
}
