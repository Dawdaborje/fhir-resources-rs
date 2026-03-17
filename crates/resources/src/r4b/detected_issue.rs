//! FHIR R4B DetectedIssue Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Supporting evidence
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueEvidence {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Manifestation
    pub code: Option<Vec<CodeableConcept>>,

    /// Supporting information
    pub detail: Option<Vec<Box<Reference>>>,
}

/// Step taken to address
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssueMitigation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What mitigation?
    pub action: CodeableConcept,

    /// Date committed
    pub date: Option<String>,

    /// Who is committing?
    pub author: Option<Box<Reference>>,
}

/// Indicates an actual or potential clinical issue with or between one or more active or proposed clinical actions for a patient; e.g. Drug-drug interaction, Ineffective treatment frequency, Procedure...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DetectedIssue {
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

    /// Unique id for the detected issue
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// registered | preliminary | final | amended +
    pub status: String,

    /// Issue Category, e.g. drug-drug, duplicate therapy, etc.
    pub code: Option<CodeableConcept>,

    /// high | moderate | low
    pub severity: Option<String>,

    /// Associated patient
    pub patient: Option<Box<Reference>>,

    /// When identified
    pub identified: Option<serde_json::Value>,

    /// The provider or device that identified the issue
    pub author: Option<Box<Reference>>,

    /// Problem resource
    pub implicated: Option<Vec<Box<Reference>>>,

    /// Supporting evidence
    pub evidence: Option<Vec<DetectedIssueEvidence>>,

    /// Description and context
    pub detail: Option<String>,

    /// Authority for issue
    pub reference: Option<String>,

    /// Step taken to address
    pub mitigation: Option<Vec<DetectedIssueMitigation>>,
}
