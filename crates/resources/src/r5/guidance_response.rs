//! FHIR R5 GuidanceResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// A guidance response is the formal response to a guidance request, including any output parameters returned by the evaluation, as well as the description of any proposed actions to be taken.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuidanceResponse {
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

    /// The identifier of the request associated with this response, if any
    #[serde(rename = "requestIdentifier")]
    pub request_identifier: Option<Box<Identifier>>,

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// What guidance was requested
    #[serde(rename = "moduleUri")]
    pub module_uri: String,

    #[serde(rename = "moduleCanonical")]
    pub module_canonical: String,

    #[serde(rename = "moduleCodeableConcept")]
    pub module_codeable_concept: CodeableConcept,

    /// success | data-requested | data-required | in-progress | failure | entered-in-error
    pub status: String,

    /// Patient the request was performed for
    pub subject: Option<Box<Reference>>,

    /// Encounter during which the response was returned
    pub encounter: Option<Box<Reference>>,

    /// When the guidance response was processed
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    /// Device returning the guidance
    pub performer: Option<Box<Reference>>,

    /// Why guidance is needed
    pub reason: Option<Vec<CodeableReference>>,

    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,

    /// Messages resulting from the evaluation of the artifact or artifacts
    #[serde(rename = "evaluationMessage")]
    pub evaluation_message: Option<Box<Reference>>,

    /// The output parameters of the evaluation, if any
    #[serde(rename = "outputParameters")]
    pub output_parameters: Option<Box<Reference>>,

    /// Proposed actions, if any
    pub result: Option<Vec<Box<Reference>>>,

    /// Additional required data
    #[serde(rename = "dataRequirement")]
    pub data_requirement: Option<Vec<DataRequirement>>,
}
