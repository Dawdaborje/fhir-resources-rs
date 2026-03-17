//! FHIR R4 ImagingStudy Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Each study has one or more series of instances
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeries {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// DICOM Series Instance UID for the series
    pub uid: String,

    /// Numeric identifier of this series
    pub number: Option<u32>,

    /// The modality of the instances in the series
    pub modality: Coding,

    /// A short human readable summary of the series
    pub description: Option<String>,

    /// Number of Series Related Instances
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<u32>,

    /// Series access endpoint
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// Body part examined
    #[serde(rename = "bodySite")]
    pub body_site: Option<Coding>,

    /// Body part laterality
    pub laterality: Option<Coding>,

    /// Specimen imaged
    pub specimen: Option<Vec<Box<Reference>>>,

    /// When the series started
    pub started: Option<String>,

    /// Who performed the series
    pub performer: Option<Vec<ImagingStudySeriesPerformer>>,

    /// A single SOP instance from the series
    pub instance: Option<Vec<ImagingStudySeriesInstance>>,
}

/// Who performed the series
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeriesPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performance
    pub function: Option<CodeableConcept>,

    /// Who performed the series
    pub actor: Box<Reference>,
}

/// A single SOP instance from the series
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudySeriesInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// DICOM SOP Instance UID
    pub uid: String,

    /// DICOM class type
    #[serde(rename = "sopClass")]
    pub sop_class: Coding,

    /// The number of this instance in the series
    pub number: Option<u32>,

    /// Description of instance
    pub title: Option<String>,
}

/// Representation of the content produced in a DICOM imaging study. A study comprises a set of series, each of which includes a set of Service-Object Pair Instances (SOP Instances - images or other da...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagingStudy {
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

    /// Identifiers for the whole study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// registered | available | cancelled | entered-in-error | unknown
    pub status: String,

    /// All series modality if actual acquisition modalities
    pub modality: Option<Vec<Coding>>,

    /// Who or what is the subject of the study
    pub subject: Box<Reference>,

    /// Encounter with which this imaging study is associated
    pub encounter: Option<Box<Reference>>,

    /// When the study was started
    pub started: Option<String>,

    /// Request fulfilled
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Referring physician
    pub referrer: Option<Box<Reference>>,

    /// Who interpreted images
    pub interpreter: Option<Vec<Box<Reference>>>,

    /// Study access endpoint
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// Number of Study Related Series
    #[serde(rename = "numberOfSeries")]
    pub number_of_series: Option<u32>,

    /// Number of Study Related Instances
    #[serde(rename = "numberOfInstances")]
    pub number_of_instances: Option<u32>,

    /// The performed Procedure reference
    #[serde(rename = "procedureReference")]
    pub procedure_reference: Option<Box<Reference>>,

    /// The performed procedure code
    #[serde(rename = "procedureCode")]
    pub procedure_code: Option<Vec<CodeableConcept>>,

    /// Where ImagingStudy occurred
    pub location: Option<Box<Reference>>,

    /// Why the study was requested
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why was study performed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// User-defined comments
    pub note: Option<Vec<Annotation>>,

    /// Institution-generated description
    pub description: Option<String>,

    /// Each study has one or more series of instances
    pub series: Option<Vec<ImagingStudySeries>>,
}
