//! FHIR R5 Permission Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The asserted justification for using the data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionJustification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The regulatory grounds upon which this Permission builds
    pub basis: Option<Vec<CodeableConcept>>,

    /// Justifing rational
    pub evidence: Option<Vec<Box<Reference>>>,
}

/// Constraints to the Permission
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRule {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// deny | permit
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// The selection criteria to identify data that is within scope of this provision
    pub data: Option<Vec<PermissionRuleData>>,

    /// A description or definition of which activities are allowed to be done on the data
    pub activity: Option<Vec<PermissionRuleActivity>>,

    /// What limits apply to the use of the data
    pub limit: Option<Vec<CodeableConcept>>,
}

/// The selection criteria to identify data that is within scope of this provision
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleData {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Explicit FHIR Resource references
    pub resource: Option<Vec<PermissionRuleDataResource>>,

    /// Security tag code on .meta.security
    pub security: Option<Vec<Coding>>,

    /// Timeframe encompasing data create/update
    pub period: Option<Vec<Period>>,

    /// Expression identifying the data
    pub expression: Option<Expression>,
}

/// Explicit FHIR Resource references
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleDataResource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// instance | related | dependents | authoredby
    pub meaning: String,

    /// The actual data reference
    pub reference: Box<Reference>,
}

/// A description or definition of which activities are allowed to be done on the data
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRuleActivity {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Authorized actor(s)
    pub actor: Option<Vec<Box<Reference>>>,

    /// Actions controlled by this rule
    pub action: Option<Vec<CodeableConcept>>,

    /// The purpose for which the permission is given
    pub purpose: Option<Vec<CodeableConcept>>,
}

/// Permission resource holds access rules for a given data and context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permission {
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

    /// active | entered-in-error | draft | rejected
    pub status: String,

    /// The person or entity that asserts the permission
    pub asserter: Option<Box<Reference>>,

    /// The date that permission was asserted
    pub date: Option<Vec<String>>,

    /// The period in which the permission is active
    pub validity: Option<Period>,

    /// The asserted justification for using the data
    pub justification: Option<PermissionJustification>,

    /// deny-overrides | permit-overrides | ordered-deny-overrides | ordered-permit-overrides | deny-unless-permit | permit-unless-deny
    pub combining: String,

    /// Constraints to the Permission
    pub rule: Option<Vec<PermissionRule>>,
}
