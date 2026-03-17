//! FHIR R4 Consent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Policies covered by this consent
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentPolicy {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Enforcement source for policy
    pub authority: Option<String>,

    /// Specific policy covered by this consent
    pub uri: Option<String>,
}

/// Consent Verified by patient or family
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentVerification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Has been verified
    pub verified: bool,

    /// Person who verified
    #[serde(rename = "verifiedWith")]
    pub verified_with: Option<Box<Reference>>,

    /// When consent verified
    #[serde(rename = "verificationDate")]
    pub verification_date: Option<String>,
}

/// Constraints to the base Consent.policyRule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvision {
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

    /// Timeframe for this rule
    pub period: Option<Period>,

    /// Who|what controlled by this rule (or group, by role)
    pub actor: Option<Vec<ConsentProvisionActor>>,

    /// Actions controlled by this rule
    pub action: Option<Vec<CodeableConcept>>,

    /// Security Labels that define affected resources
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<Coding>>,

    /// Context of activities covered by this rule
    pub purpose: Option<Vec<Coding>>,

    /// e.g. Resource Type, Profile, CDA, etc.
    pub class: Option<Vec<Coding>>,

    /// e.g. LOINC or SNOMED CT code, etc. in the content
    pub code: Option<Vec<CodeableConcept>>,

    /// Timeframe for data controlled by this rule
    #[serde(rename = "dataPeriod")]
    pub data_period: Option<Period>,

    /// Data controlled by this rule
    pub data: Option<Vec<ConsentProvisionData>>,

    /// Nested Exception Rules
    pub provision: Option<Vec<ConsentProvision>>,
}

/// Who|what controlled by this rule (or group, by role)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvisionActor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How the actor is involved
    pub role: CodeableConcept,

    /// Resource for the actor (or group, by role)
    pub reference: Box<Reference>,
}

/// Data controlled by this rule
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentProvisionData {
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

/// A record of a healthcare consumer’s choices, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions within a given policy context, for specific purpos...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consent {
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

    /// Identifier for this record (external references)
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | proposed | active | rejected | inactive | entered-in-error
    pub status: String,

    /// Which of the four areas this resource covers (extensible)
    pub scope: CodeableConcept,

    /// Classification of the consent statement - for indexing/retrieval
    pub category: Vec<CodeableConcept>,

    /// Who the consent applies to
    pub patient: Option<Box<Reference>>,

    /// When this Consent was created or indexed
    #[serde(rename = "dateTime")]
    pub date_time: Option<String>,

    /// Who is agreeing to the policy and rules
    pub performer: Option<Vec<Box<Reference>>>,

    /// Custodian of the consent
    pub organization: Option<Vec<Box<Reference>>>,

    /// Source from which this consent is taken
    #[serde(rename = "sourceAttachment")]
    pub source_attachment: Option<Attachment>,

    #[serde(rename = "sourceReference")]
    pub source_reference: Option<Box<Reference>>,

    /// Policies covered by this consent
    pub policy: Option<Vec<ConsentPolicy>>,

    /// Regulation that this consents to
    #[serde(rename = "policyRule")]
    pub policy_rule: Option<CodeableConcept>,

    /// Consent Verified by patient or family
    pub verification: Option<Vec<ConsentVerification>>,

    /// Constraints to the base Consent.policyRule
    pub provision: Option<ConsentProvision>,
}
