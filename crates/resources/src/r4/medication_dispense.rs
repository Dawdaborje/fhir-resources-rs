//! FHIR R4 MedicationDispense Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Who performed event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Who performed the dispense and what they did
    pub function: Option<CodeableConcept>,

    /// Individual who was performing
    pub actor: Box<Reference>,
}

/// Whether a substitution was performed on the dispense
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispenseSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether a substitution was or was not performed on the dispense
    #[serde(rename = "wasSubstituted")]
    pub was_substituted: bool,

    /// Code signifying whether a different drug was dispensed from what was prescribed
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Why was substitution made
    pub reason: Option<Vec<CodeableConcept>>,

    /// Who is responsible for the substitution
    #[serde(rename = "responsibleParty")]
    pub responsible_party: Option<Vec<Box<Reference>>>,
}

/// Indicates that a medication product is to be or has been dispensed for a named person/patient. This includes a description of the medication product (supply) provided and the instructions for admin...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationDispense {
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

    /// Event that dispense is part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    pub status: String,

    /// Why a dispense was not performed
    #[serde(rename = "statusReason")]
    pub status_reason: Option<serde_json::Value>,

    /// Type of medication dispense
    pub category: Option<CodeableConcept>,

    /// What medication was supplied
    pub medication: serde_json::Value,

    /// Who the dispense is for
    pub subject: Option<Box<Reference>>,

    /// Encounter / Episode associated with event
    pub context: Option<Box<Reference>>,

    /// Information that supports the dispensing of the medication
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// Who performed event
    pub performer: Option<Vec<MedicationDispensePerformer>>,

    /// Where the dispense occurred
    pub location: Option<Box<Reference>>,

    /// Medication order that authorizes the dispense
    #[serde(rename = "authorizingPrescription")]
    pub authorizing_prescription: Option<Vec<Box<Reference>>>,

    /// Trial fill, partial fill, emergency fill, etc.
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Amount dispensed
    pub quantity: Option<Quantity>,

    /// Amount of medication expressed as a timing amount
    #[serde(rename = "daysSupply")]
    pub days_supply: Option<Quantity>,

    /// When product was packaged and reviewed
    #[serde(rename = "whenPrepared")]
    pub when_prepared: Option<String>,

    /// When product was given out
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<String>,

    /// Where the medication was sent
    pub destination: Option<Box<Reference>>,

    /// Who collected the medication
    pub receiver: Option<Vec<Box<Reference>>>,

    /// Information about the dispense
    pub note: Option<Vec<Annotation>>,

    /// How the medication is to be used by the patient or administered by the caregiver
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,

    /// Whether a substitution was performed on the dispense
    pub substitution: Option<MedicationDispenseSubstitution>,

    /// Clinical issue with action
    #[serde(rename = "detectedIssue")]
    pub detected_issue: Option<Vec<Box<Reference>>>,

    /// A list of relevant lifecycle events
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Box<Reference>>>,
}
