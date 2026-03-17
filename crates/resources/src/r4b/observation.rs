//! FHIR R4B Observation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Provides guide for interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationReferenceRange {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Low Range, if relevant
    pub low: Option<Quantity>,

    /// High Range, if relevant
    pub high: Option<Quantity>,

    /// Reference range qualifier
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Reference range population
    #[serde(rename = "appliesTo")]
    pub applies_to: Option<Vec<CodeableConcept>>,

    /// Applicable age range, if relevant
    pub age: Option<Range>,

    /// Text based reference range in an observation
    pub text: Option<String>,
}

/// Component results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservationComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of component observation (code / type)
    pub code: CodeableConcept,

    /// Actual component result
    pub value: Option<serde_json::Value>,

    /// Why the component result is missing
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,

    /// High, low, normal, etc.
    pub interpretation: Option<Vec<CodeableConcept>>,

    /// Provides guide for interpretation of component result
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<ObservationReferenceRange>>,
}

/// Measurements and simple assertions made about a patient, device or other subject.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
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

    /// Business Identifier for observation
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// registered | preliminary | final | amended +
    pub status: String,

    /// Classification of type of observation
    pub category: Option<Vec<CodeableConcept>>,

    /// Type of observation (code / type)
    pub code: CodeableConcept,

    /// Who and/or what the observation is about
    pub subject: Option<Box<Reference>>,

    /// What the observation is about, when it is not about the subject of record
    pub focus: Option<Vec<Box<Reference>>>,

    /// Healthcare event during which this observation is made
    pub encounter: Option<Box<Reference>>,

    /// Clinically relevant time/time-period for observation
    pub effective: Option<serde_json::Value>,

    /// Date/Time this version was made available
    pub issued: Option<String>,

    /// Who is responsible for the observation
    pub performer: Option<Vec<Box<Reference>>>,

    /// Actual result
    pub value: Option<serde_json::Value>,

    /// Why the result is missing
    #[serde(rename = "dataAbsentReason")]
    pub data_absent_reason: Option<CodeableConcept>,

    /// High, low, normal, etc.
    pub interpretation: Option<Vec<CodeableConcept>>,

    /// Comments about the observation
    pub note: Option<Vec<Annotation>>,

    /// Observed body part
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// How it was done
    pub method: Option<CodeableConcept>,

    /// Specimen used for this observation
    pub specimen: Option<Box<Reference>>,

    /// (Measurement) Device
    pub device: Option<Box<Reference>>,

    /// Provides guide for interpretation
    #[serde(rename = "referenceRange")]
    pub reference_range: Option<Vec<ObservationReferenceRange>>,

    /// Related resource that belongs to the Observation group
    #[serde(rename = "hasMember")]
    pub has_member: Option<Vec<Box<Reference>>>,

    /// Related measurements the observation is made from
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// Component results
    pub component: Option<Vec<ObservationComponent>>,
}
