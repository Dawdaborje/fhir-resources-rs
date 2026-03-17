//! FHIR R4B ResearchSubject Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A physical entity which is the primary unit of operational and/or administrative interest in a study.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchSubject {
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

    /// Business Identifier for research subject in a study
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    pub status: String,

    /// Start and end of participation
    pub period: Option<Period>,

    /// Study subject is part of
    pub study: Box<Reference>,

    /// Who is part of study
    pub individual: Box<Reference>,

    /// What path should be followed
    #[serde(rename = "assignedArm")]
    pub assigned_arm: Option<String>,

    /// What path was followed
    #[serde(rename = "actualArm")]
    pub actual_arm: Option<String>,

    /// Agreement to participate in study
    pub consent: Option<Box<Reference>>,
}
