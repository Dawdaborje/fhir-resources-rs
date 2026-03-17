//! FHIR R4 Schedule Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// A container for slots of time that may be available for booking appointments.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
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

    /// External Ids for this item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this schedule is in active use
    pub active: Option<bool>,

    /// High-level category
    #[serde(rename = "serviceCategory")]
    pub service_category: Option<Vec<CodeableConcept>>,

    /// Specific service
    #[serde(rename = "serviceType")]
    pub service_type: Option<Vec<CodeableConcept>>,

    /// Type of specialty needed
    pub specialty: Option<Vec<CodeableConcept>>,

    /// Resource(s) that availability information is being provided for
    pub actor: Vec<Box<Reference>>,

    /// Period of time covered by schedule
    #[serde(rename = "planningHorizon")]
    pub planning_horizon: Option<Period>,

    /// Comments on availability
    pub comment: Option<String>,
}
