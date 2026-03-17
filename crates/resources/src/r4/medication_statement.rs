//! FHIR R4 MedicationStatement Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// A record of a medication that is being consumed by a patient. A MedicationStatement may indicate that the patient may be taking the medication now or has taken the medication in the past or will be...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationStatement {
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

    /// External identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Fulfils plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// active | completed | entered-in-error | intended | stopped | on-hold | unknown | not-taken
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<Vec<CodeableConcept>>,

    /// Type of medication usage
    pub category: Option<CodeableConcept>,

    /// What medication was taken
    pub medication: serde_json::Value,

    /// Who is/was taking the medication
    pub subject: Box<Reference>,

    /// Encounter / Episode associated with MedicationStatement
    pub context: Option<Box<Reference>>,

    /// The date/time or interval when the medication is/was/will be taken
    pub effective: Option<serde_json::Value>,

    /// When the statement was asserted?
    #[serde(rename = "dateAsserted")]
    pub date_asserted: Option<String>,

    /// Person or organization that provided the information about the taking of this medication
    #[serde(rename = "informationSource")]
    pub information_source: Option<Box<Reference>>,

    /// Additional supporting information
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// Reason for why the medication is being/was taken
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Condition or observation that supports why the medication is being/was taken
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Further information about the statement
    pub note: Option<Vec<Annotation>>,

    /// Details of how medication is/was taken or should be taken
    pub dosage: Option<Vec<Dosage>>,
}
