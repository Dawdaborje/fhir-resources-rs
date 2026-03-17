//! FHIR R4B VerificationResult Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Information about the primary source(s) involved in validation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultPrimarySource {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to the primary source
    pub who: Option<Box<Reference>>,

    /// Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Method for exchanging information with the primary source
    #[serde(rename = "communicationMethod")]
    pub communication_method: Option<Vec<CodeableConcept>>,

    /// successful | failed | unknown
    #[serde(rename = "validationStatus")]
    pub validation_status: Option<CodeableConcept>,

    /// When the target was validated against the primary source
    #[serde(rename = "validationDate")]
    pub validation_date: Option<String>,

    /// yes | no | undetermined
    #[serde(rename = "canPushUpdates")]
    pub can_push_updates: Option<CodeableConcept>,

    /// specific | any | source
    #[serde(rename = "pushTypeAvailable")]
    pub push_type_available: Option<Vec<CodeableConcept>>,
}

/// Information about the entity attesting to information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultAttestation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The individual or organization attesting to information
    pub who: Option<Box<Reference>>,

    /// When the who is asserting on behalf of another (organization or individual)
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,

    /// The method by which attested information was submitted/retrieved
    #[serde(rename = "communicationMethod")]
    pub communication_method: Option<CodeableConcept>,

    /// The date the information was attested to
    pub date: Option<String>,

    /// A digital identity certificate associated with the attestation source
    #[serde(rename = "sourceIdentityCertificate")]
    pub source_identity_certificate: Option<String>,

    /// A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source
    #[serde(rename = "proxyIdentityCertificate")]
    pub proxy_identity_certificate: Option<String>,

    /// Proxy signature
    #[serde(rename = "proxySignature")]
    pub proxy_signature: Option<Signature>,

    /// Attester signature
    #[serde(rename = "sourceSignature")]
    pub source_signature: Option<Signature>,
}

/// Information about the entity validating information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResultValidator {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to the organization validating information
    pub organization: Box<Reference>,

    /// A digital identity certificate associated with the validator
    #[serde(rename = "identityCertificate")]
    pub identity_certificate: Option<String>,

    /// Validator signature
    #[serde(rename = "attestationSignature")]
    pub attestation_signature: Option<Signature>,
}

/// Describes validation requirements, source(s), status and dates for one or more elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationResult {
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

    /// A resource that was validated
    pub target: Option<Vec<Box<Reference>>>,

    /// The fhirpath location(s) within the resource that was validated
    #[serde(rename = "targetLocation")]
    pub target_location: Option<Vec<String>>,

    /// none | initial | periodic
    pub need: Option<CodeableConcept>,

    /// attested | validated | in-process | req-revalid | val-fail | reval-fail
    pub status: String,

    /// When the validation status was updated
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// nothing | primary | multiple
    #[serde(rename = "validationType")]
    pub validation_type: Option<CodeableConcept>,

    /// The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)
    #[serde(rename = "validationProcess")]
    pub validation_process: Option<Vec<CodeableConcept>>,

    /// Frequency of revalidation
    pub frequency: Option<Timing>,

    /// The date/time validation was last completed (including failed validations)
    #[serde(rename = "lastPerformed")]
    pub last_performed: Option<String>,

    /// The date when target is next validated, if appropriate
    #[serde(rename = "nextScheduled")]
    pub next_scheduled: Option<String>,

    /// fatal | warn | rec-only | none
    #[serde(rename = "failureAction")]
    pub failure_action: Option<CodeableConcept>,

    /// Information about the primary source(s) involved in validation
    #[serde(rename = "primarySource")]
    pub primary_source: Option<Vec<VerificationResultPrimarySource>>,

    /// Information about the entity attesting to information
    pub attestation: Option<VerificationResultAttestation>,

    /// Information about the entity validating information
    pub validator: Option<Vec<VerificationResultValidator>>,
}
