//! FHIR R5 ImagingSelection Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Selector of the instances (human or machine)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performer
    pub function: Option<CodeableConcept>,

    /// Author (human or machine)
    pub actor: Option<Box<Reference>>,
}

/// The selected instances
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// DICOM SOP Instance UID
    pub uid: String,

    /// DICOM Instance Number
    pub number: Option<u32>,

    /// DICOM SOP Class UID
    #[serde(rename = "sopClass")]
    pub sop_class: Option<Coding>,

    /// The selected subset of the SOP Instance
    pub subset: Option<Vec<String>>,

    /// A specific 2D region in a DICOM image / frame
    #[serde(rename = "imageRegion2D")]
    pub image_region2d: Option<Vec<ImagingSelectionInstanceImageRegion2D>>,

    /// A specific 3D region in a DICOM frame of reference
    #[serde(rename = "imageRegion3D")]
    pub image_region3d: Option<Vec<ImagingSelectionInstanceImageRegion3D>>,
}

/// A specific 2D region in a DICOM image / frame
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion2D {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// point | polyline | interpolated | circle | ellipse
    #[serde(rename = "regionType")]
    pub region_type: String,

    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<f64>,
}

/// A specific 3D region in a DICOM frame of reference
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelectionInstanceImageRegion3D {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    #[serde(rename = "regionType")]
    pub region_type: String,

    /// Specifies the coordinates that define the image region
    pub coordinate: Vec<f64>,
}

/// A selection of DICOM SOP instances and/or frames within a single Study and Series. This might include additional specifics such as an image region, an Observation UID or a Segmentation Number, allo...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingSelection {
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

    /// Business Identifier for Imaging Selection
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// available | entered-in-error | unknown
    pub status: String,

    /// Subject of the selected instances
    pub subject: Option<Box<Reference>>,

    /// Date / Time when this imaging selection was created
    pub issued: Option<String>,

    /// Selector of the instances (human or machine)
    pub performer: Option<Vec<ImagingSelectionPerformer>>,

    /// Associated request
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Classifies the imaging selection
    pub category: Option<Vec<CodeableConcept>>,

    /// Imaging Selection purpose text or code
    pub code: CodeableConcept,

    /// DICOM Study Instance UID
    #[serde(rename = "studyUid")]
    pub study_uid: Option<String>,

    /// The imaging study from which the imaging selection is derived
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// The network service providing retrieval for the images referenced in the imaging selection
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// DICOM Series Instance UID
    #[serde(rename = "seriesUid")]
    pub series_uid: Option<String>,

    /// DICOM Series Number
    #[serde(rename = "seriesNumber")]
    pub series_number: Option<u32>,

    /// The Frame of Reference UID for the selected images
    #[serde(rename = "frameOfReferenceUid")]
    pub frame_of_reference_uid: Option<String>,

    /// Body part examined
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableReference>,

    /// Related resource that is the focus for the imaging selection
    pub focus: Option<Vec<Box<Reference>>>,

    /// The selected instances
    pub instance: Option<Vec<ImagingSelectionInstance>>,
}
