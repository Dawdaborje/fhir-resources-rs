//! FHIR R5 MedicationRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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

    /// Intended performer of dispense
    pub dispenser: Option<Box<Reference>>,

    /// Additional information for the dispenser
    #[serde(rename = "dispenserInstruction")]
    pub dispenser_instruction: Option<Vec<Annotation>>,

    /// Type of adherence packaging to use for the dispense
    #[serde(rename = "doseAdministrationAid")]
    pub dose_administration_aid: Option<CodeableConcept>,
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
    pub allowed: serde_json::Value,

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

    /// A plan or request that is fulfilled in whole or in part by this medication request
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Reference to an order/prescription that is being replaced by this MedicationRequest
    #[serde(rename = "priorPrescription")]
    pub prior_prescription: Option<Box<Reference>>,

    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// active | on-hold | ended | stopped | completed | cancelled | entered-in-error | draft | unknown
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// When the status was changed
    #[serde(rename = "statusChanged")]
    pub status_changed: Option<String>,

    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// Grouping or category of medication request
    pub category: Option<Vec<CodeableConcept>>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if patient is to stop taking or not to start taking the medication
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// Medication to be taken
    pub medication: CodeableReference,

    /// Individual or group for whom the medication has been requested
    pub subject: Box<Reference>,

    /// The person or organization who provided the information about this request, if the source is someone other than the requestor
    #[serde(rename = "informationSource")]
    pub information_source: Option<Vec<Box<Reference>>>,

    /// Encounter created as part of encounter/admission/stay
    pub encounter: Option<Box<Reference>>,

    /// Information to support fulfilling of the medication
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,

    /// When request was initially authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Who/What requested the Request
    pub requester: Option<Box<Reference>>,

    /// Reported rather than primary record
    pub reported: Option<bool>,

    /// Desired kind of performer of the medication administration
    #[serde(rename = "performerType")]
    pub performer_type: Option<CodeableConcept>,

    /// Intended performer of administration
    pub performer: Option<Vec<Box<Reference>>>,

    /// Intended type of device for the administration
    pub device: Option<Vec<CodeableReference>>,

    /// Person who entered the request
    pub recorder: Option<Box<Reference>>,

    /// Reason or indication for ordering or not ordering the medication
    pub reason: Option<Vec<CodeableReference>>,

    /// Overall pattern of medication administration
    #[serde(rename = "courseOfTherapyType")]
    pub course_of_therapy_type: Option<CodeableConcept>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Information about the prescription
    pub note: Option<Vec<Annotation>>,

    /// Full representation of the dosage instructions
    #[serde(rename = "renderedDosageInstruction")]
    pub rendered_dosage_instruction: Option<String>,

    /// Period over which the medication is to be taken
    #[serde(rename = "effectiveDosePeriod")]
    pub effective_dose_period: Option<Period>,

    /// Specific instructions for how the medication should be taken
    #[serde(rename = "dosageInstruction")]
    pub dosage_instruction: Option<Vec<Dosage>>,

    /// Medication supply authorization
    #[serde(rename = "dispenseRequest")]
    pub dispense_request: Option<MedicationRequestDispenseRequest>,

    /// Any restrictions on medication substitution
    pub substitution: Option<MedicationRequestSubstitution>,

    /// A list of events of interest in the lifecycle
    #[serde(rename = "eventHistory")]
    pub event_history: Option<Vec<Box<Reference>>>,
}
