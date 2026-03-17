//! FHIR R4 Specimen Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Collection details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenCollection {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Who collected the specimen
    pub collector: Option<Box<Reference>>,

    /// Collection time
    pub collected: Option<serde_json::Value>,

    /// How long it took to collect specimen
    pub duration: Option<Duration>,

    /// The quantity of specimen collected
    pub quantity: Option<Quantity>,

    /// Technique used to perform collection
    pub method: Option<CodeableConcept>,

    /// Anatomical collection site
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Whether or how long patient abstained from food and/or drink
    #[serde(rename = "fastingStatus")]
    pub fasting_status: Option<serde_json::Value>,
}

/// Processing and processing step details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenProcessing {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Textual description of procedure
    pub description: Option<String>,

    /// Indicates the treatment step applied to the specimen
    pub procedure: Option<CodeableConcept>,

    /// Material used in the processing step
    pub additive: Option<Vec<Box<Reference>>>,

    /// Date and time of specimen processing
    pub time: Option<serde_json::Value>,
}

/// Direct container of specimen (tube/slide, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpecimenContainer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Id for the container
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Textual description of the container
    pub description: Option<String>,

    /// Kind of container directly associated with specimen
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Container volume or size
    pub capacity: Option<Quantity>,

    /// Quantity of specimen within container
    #[serde(rename = "specimenQuantity")]
    pub specimen_quantity: Option<Quantity>,

    /// Additive associated with container
    pub additive: Option<serde_json::Value>,
}

/// A sample to be used for analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Specimen {
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

    /// External Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Identifier assigned by the lab
    #[serde(rename = "accessionIdentifier")]
    pub accession_identifier: Option<Box<Identifier>>,

    /// available | unavailable | unsatisfactory | entered-in-error
    pub status: Option<String>,

    /// Kind of material that forms the specimen
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance or a device
    pub subject: Option<Box<Reference>>,

    /// The time when specimen was received for processing
    #[serde(rename = "receivedTime")]
    pub received_time: Option<String>,

    /// Specimen from which this specimen originated
    pub parent: Option<Vec<Box<Reference>>>,

    /// Why the specimen was collected
    pub request: Option<Vec<Box<Reference>>>,

    /// Collection details
    pub collection: Option<SpecimenCollection>,

    /// Processing and processing step details
    pub processing: Option<Vec<SpecimenProcessing>>,

    /// Direct container of specimen (tube/slide, etc.)
    pub container: Option<Vec<SpecimenContainer>>,

    /// State of the specimen
    pub condition: Option<Vec<CodeableConcept>>,

    /// Comments
    pub note: Option<Vec<Annotation>>,
}
