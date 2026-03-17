//! FHIR R5 Consent Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Computable version of the backing policy
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsentPolicyBasis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference backing policy resource
    pub reference: Option<Box<Reference>>,

    /// URL to a computable backing policy
    pub url: Option<String>,
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

    /// Business case of verification
    #[serde(rename = "verificationType")]
    pub verification_type: Option<CodeableConcept>,

    /// Person conducting verification
    #[serde(rename = "verifiedBy")]
    pub verified_by: Option<Box<Reference>>,

    /// Person who verified
    #[serde(rename = "verifiedWith")]
    pub verified_with: Option<Box<Reference>>,

    /// When consent verified
    #[serde(rename = "verificationDate")]
    pub verification_date: Option<Vec<String>>,
}

/// Constraints to the base Consent.policyRule/Consent.policy
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

    /// Timeframe for this provision
    pub period: Option<Period>,

    /// Who|what controlled by this provision (or group, by role)
    pub actor: Option<Vec<ConsentProvisionActor>>,

    /// Actions controlled by this provision
    pub action: Option<Vec<CodeableConcept>>,

    /// Security Labels that define affected resources
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<Coding>>,

    /// Context of activities covered by this provision
    pub purpose: Option<Vec<Coding>>,

    /// e.g. Resource Type, Profile, CDA, etc
    #[serde(rename = "documentType")]
    pub document_type: Option<Vec<Coding>>,

    /// e.g. Resource Type, Profile, etc
    #[serde(rename = "resourceType")]
    pub resource_type: Option<Vec<Coding>>,

    /// e.g. LOINC or SNOMED CT code, etc. in the content
    pub code: Option<Vec<CodeableConcept>>,

    /// Timeframe for data controlled by this provision
    #[serde(rename = "dataPeriod")]
    pub data_period: Option<Period>,

    /// Data controlled by this provision
    pub data: Option<Vec<ConsentProvisionData>>,

    /// A computable expression of the consent
    pub expression: Option<Expression>,

    /// Nested Exception Provisions
    pub provision: Option<Vec<ConsentProvision>>,
}

/// Who|what controlled by this provision (or group, by role)
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
    pub role: Option<CodeableConcept>,

    /// Resource for the actor (or group, by role)
    pub reference: Option<Box<Reference>>,
}

/// Data controlled by this provision
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

/// A record of a healthcare consumer’s choices or choices made on their behalf by a third party, which permits or denies identified recipient(s) or recipient role(s) to perform one or more actions w...
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

    /// draft | active | inactive | not-done | entered-in-error | unknown
    pub status: String,

    /// Classification of the consent statement - for indexing/retrieval
    pub category: Option<Vec<CodeableConcept>>,

    /// Who the consent applies to
    pub subject: Option<Box<Reference>>,

    /// Fully executed date of the consent
    pub date: Option<String>,

    /// Effective period for this Consent
    pub period: Option<Period>,

    /// Who is granting rights according to the policy and rules
    pub grantor: Option<Vec<Box<Reference>>>,

    /// Who is agreeing to the policy and rules
    pub grantee: Option<Vec<Box<Reference>>>,

    /// Consent workflow management
    pub manager: Option<Vec<Box<Reference>>>,

    /// Consent Enforcer
    pub controller: Option<Vec<Box<Reference>>>,

    /// Source from which this consent is taken
    #[serde(rename = "sourceAttachment")]
    pub source_attachment: Option<Vec<Attachment>>,

    /// Source from which this consent is taken
    #[serde(rename = "sourceReference")]
    pub source_reference: Option<Vec<Box<Reference>>>,

    /// Regulations establishing base Consent
    #[serde(rename = "regulatoryBasis")]
    pub regulatory_basis: Option<Vec<CodeableConcept>>,

    /// Computable version of the backing policy
    #[serde(rename = "policyBasis")]
    pub policy_basis: Option<ConsentPolicyBasis>,

    /// Human Readable Policy
    #[serde(rename = "policyText")]
    pub policy_text: Option<Vec<Box<Reference>>>,

    /// Consent Verified by patient or family
    pub verification: Option<Vec<ConsentVerification>>,

    /// deny | permit
    pub decision: Option<String>,

    /// Constraints to the base Consent.policyRule/Consent.policy
    pub provision: Option<Vec<ConsentProvision>>,
}
