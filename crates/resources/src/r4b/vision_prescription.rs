//! FHIR R4B VisionPrescription Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Vision lens authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescriptionLensSpecification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Product to be supplied
    pub product: CodeableConcept,

    /// right | left
    pub eye: String,

    /// Power of the lens
    pub sphere: Option<f64>,

    /// Lens power for astigmatism
    pub cylinder: Option<f64>,

    /// Lens meridian which contain no power for astigmatism
    pub axis: Option<i32>,

    /// Eye alignment compensation
    pub prism: Option<Vec<VisionPrescriptionLensSpecificationPrism>>,

    /// Added power for multifocal levels
    pub add: Option<f64>,

    /// Contact lens power
    pub power: Option<f64>,

    /// Contact lens back curvature
    #[serde(rename = "backCurve")]
    pub back_curve: Option<f64>,

    /// Contact lens diameter
    pub diameter: Option<f64>,

    /// Lens wear duration
    pub duration: Option<Quantity>,

    /// Color required
    pub color: Option<String>,

    /// Brand required
    pub brand: Option<String>,

    /// Notes for coatings
    pub note: Option<Vec<Annotation>>,
}

/// Eye alignment compensation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescriptionLensSpecificationPrism {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Amount of adjustment
    pub amount: f64,

    /// up | down | in | out
    pub base: String,
}

/// An authorization for the provision of glasses and/or contact lenses to a patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VisionPrescription {
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

    /// Business Identifier for vision prescription
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Response creation date
    pub created: String,

    /// Who prescription is for
    pub patient: Box<Reference>,

    /// Created during encounter / admission / stay
    pub encounter: Option<Box<Reference>>,

    /// When prescription was authorized
    #[serde(rename = "dateWritten")]
    pub date_written: String,

    /// Who authorized the vision prescription
    pub prescriber: Box<Reference>,

    /// Vision lens authorization
    #[serde(rename = "lensSpecification")]
    pub lens_specification: Vec<VisionPrescriptionLensSpecification>,
}
