//! FHIR R5 EnrollmentResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// This resource provides enrollment and plan details from the processing of an EnrollmentRequest resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentResponse {
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

    /// Business Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: Option<String>,

    /// Claim reference
    pub request: Option<Box<Reference>>,

    /// queued | complete | error | partial
    pub outcome: Option<String>,

    /// Disposition Message
    pub disposition: Option<String>,

    /// Creation date
    pub created: Option<String>,

    /// Insurer
    pub organization: Option<Box<Reference>>,

    /// Responsible practitioner
    #[serde(rename = "requestProvider")]
    pub request_provider: Option<Box<Reference>>,
}
