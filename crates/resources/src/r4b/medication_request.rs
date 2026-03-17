//! FHIR R4B MedicationRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Medication supply authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequest {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// First fill details
    #[serde(rename = "initialFill")]
    pub initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,

    /// Minimum period of time between dispenses
    #[serde(rename = "dispenseInterval")]
    pub dispense_interval: Option<Duration>,

    /// Time period supply is authorized for
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,

    /// Number of refills authorized
    #[serde(rename = "numberOfRepeatsAllowed")]
    pub number_of_repeats_allowed: Option<u32>,

    /// Amount of medication to supply per dispense
    pub quantity: Option<Quantity>,

    /// Number of days supply per dispense
    #[serde(rename = "expectedSupplyDuration")]
    pub expected_supply_duration: Option<Duration>,

    /// Intended dispenser
    pub performer: Option<Box<Reference>>,
}

/// First fill details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestDispenseRequestInitialFill {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// First fill quantity
    pub quantity: Option<Quantity>,

    /// First fill duration
    pub duration: Option<Duration>,
}

/// Any restrictions on medication substitution
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequestSubstitution {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Whether substitution is allowed or not
    #[serde(rename = "allowedBoolean")]
    pub allowed_boolean: bool,

    #[serde(rename = "allowedCodeableConcept")]
    pub allowed_codeable_concept: CodeableConcept,

    /// Why should (not) substitution be made
    pub reason: Option<CodeableConcept>,
}

/// An order or request for both supply of the medication and the instructions for administration of the medication to a patient. The resource is called "MedicationRequest" rather than "MedicationPresc...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicationRequest {
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

    /// External ids for this request
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | on-hold | cancelled | completed | entered-in-error | stopped | draft | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// Type of medication usage
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if request is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// Reported rather than primary record
    #[serde(rename = "reportedBoolean")]
    pub reported_boolean: Option<bool>,

    #[serde(rename = "reportedReference")]
    pub reported_reference: Option<Box<Reference>>,

    /// Medication to be taken
    #[serde(rename = "medicationCodeableConcept")]
    pub medication_codeable_concept: CodeableConcept,

    #[serde(rename = "medicationReference")]
    pub medication_reference: Box<Reference>,

    /// Who or group medication request is for
    pub subject: Box<Reference>,

    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<Box<Reference>>,

    /// Information to support ordering of the medication
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// When request was initially authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Who/What requested the Request
    pub requester: Option<Box<Reference>>,

    /// Intended performer of administration
    pub performer: Option<Box<Reference>>,

    /// Desired kind of performer of the medication administration
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,

    /// Person who entered the request
    pub recorder: Option<Box<Reference>>,

    /// Reason or indication for ordering or not ordering the medication
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Condition or observation that supports why the prescription is being written
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// What request fulfills
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// Overall pattern of medication administration
    #[serde(rename = "courseOfTherapyType")]
    pub course_of_therapy_type: Option<CodeableConcept>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Information about the prescription
    pub note: Option<Vec<Annotation>>,

    /// How the medication should be taken
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,

    /// Medication supply authorization
    #[serde(rename = "dispenseRequest")]
    pub dispense_request: Option<MedicationRequestDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,

    /// An order/prescription that is being replaced
    #[serde(rename = "priorPrescription")]
    pub prior_prescription: Option<Box<Reference>>,

    /// Clinical Issue with action
    #[serde(rename = "detectedIssue")]
    pub detected_issue: Option<Vec<Box<Reference>>>,

    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Box<Reference>>>,
}
