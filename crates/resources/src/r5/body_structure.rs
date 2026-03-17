//! FHIR R5 BodyStructure Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Included anatomic location(s)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that represents the included structure
    pub structure: CodeableConcept,

    /// Code that represents the included structure laterality
    pub laterality: Option<CodeableConcept>,

    /// Landmark relative location
    #[serde(rename = "bodyLandmarkOrientation")]
    pub body_landmark_orientation:
        Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientation>>,

    /// Cartesian reference for structure
    #[serde(rename = "spatialReference")]
    pub spatial_reference: Option<Vec<Box<Reference>>>,

    /// Code that represents the included structure qualifier
    pub qualifier: Option<Vec<CodeableConcept>>,
}

/// Landmark relative location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Body ]andmark description
    #[serde(rename = "landmarkDescription")]
    pub landmark_description: Option<Vec<CodeableConcept>>,

    /// Clockface orientation
    #[serde(rename = "clockFacePosition")]
    pub clock_face_position: Option<Vec<CodeableConcept>>,

    /// Landmark relative location
    #[serde(rename = "distanceFromLandmark")]
    pub distance_from_landmark:
        Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark>>,

    /// Relative landmark surface orientation
    #[serde(rename = "surfaceOrientation")]
    pub surface_orientation: Option<Vec<CodeableConcept>>,
}

/// Landmark relative location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmark {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Measurement device
    pub device: Option<Vec<CodeableReference>>,

    /// Measured distance from body landmark
    pub value: Option<Vec<Quantity>>,
}

/// Record details about an anatomical structure. This resource may be used when a coded concept does not provide the necessary detail needed for the use case.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyStructure {
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

    /// Bodystructure identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this record is in active use
    pub active: Option<bool>,

    /// Kind of Structure
    pub morphology: Option<CodeableConcept>,

    /// Included anatomic location(s)
    #[serde(rename = "includedStructure")]
    pub included_structure: Vec<BodyStructureIncludedStructure>,

    /// Excluded anatomic locations(s)
    #[serde(rename = "excludedStructure")]
    pub excluded_structure: Option<Vec<BodyStructureIncludedStructure>>,

    /// Text description
    pub description: Option<String>,

    /// Attached images
    pub image: Option<Vec<Attachment>>,

    /// Who this is about
    pub patient: Box<Reference>,
}
