//! FHIR R5 MedicationStatement Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Indicates whether the medication is or is not being consumed or administered
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationStatementAdherence {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of adherence
    pub code: CodeableConcept,

    /// Details of the reason for the current use of the medication
    pub reason: Option<CodeableConcept>,
}

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

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// recorded | entered-in-error | draft
    pub status: String,

    /// Type of medication statement
    pub category: Option<Vec<CodeableConcept>>,

    /// What medication was taken
    pub medication: CodeableReference,

    /// Who is/was taking the medication
    pub subject: Box<Reference>,

    /// Encounter associated with MedicationStatement
    pub encounter: Option<Box<Reference>>,

    /// The date/time or interval when the medication is/was/will be taken
    #[serde(rename = "effectiveDateTime")]
    pub effective_date_time: Option<String>,

    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    #[serde(rename = "effectiveTiming")]
    pub effective_timing: Option<Timing>,

    /// When the usage was asserted?
    #[serde(rename = "dateAsserted")]
    pub date_asserted: Option<String>,

    /// Person or organization that provided the information about the taking of this medication
    #[serde(rename = "informationSource")]
    pub information_source: Option<Vec<Box<Reference>>>,

    /// Link to information used to derive the MedicationStatement
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<Box<Reference>>>,

    /// Reason for why the medication is being/was taken
    pub reason: Option<Vec<CodeableReference>>,

    /// Further information about the usage
    pub note: Option<Vec<Annotation>>,

    /// Link to information relevant to the usage of a medication
    #[serde(rename = "relatedClinicalInformation")]
    pub related_clinical_information: Option<Vec<Box<Reference>>>,

    /// Full representation of the dosage instructions
    #[serde(rename = "renderedDosageInstruction")]
    pub rendered_dosage_instruction: Option<String>,

    /// Details of how medication is/was taken or should be taken
    pub dosage: Option<Vec<Dosage>>,

    /// Indicates whether the medication is or is not being consumed or administered
    pub adherence: Option<MedicationStatementAdherence>,
}
