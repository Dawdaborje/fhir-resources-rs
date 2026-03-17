//! FHIR R4 Immunization Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Who performed event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What type of performance was done
    pub function: Option<CodeableConcept>,

    /// Individual or organization who was performing
    pub actor: Box<Reference>,
}

/// Educational material presented to patient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationEducation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Educational material document identifier
    #[serde(rename = "documentType")]
    pub document_type: Option<String>,

    /// Educational material reference pointer
    pub reference: Option<String>,

    /// Educational material publication date
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,

    /// Educational material presentation date
    #[serde(rename = "presentationDate")]
    pub presentation_date: Option<String>,
}

/// Details of a reaction that follows immunization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationReaction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// When reaction started
    pub date: Option<String>,

    /// Additional information on reaction
    pub detail: Option<Box<Reference>>,

    /// Indicates self-reported reaction
    pub reported: Option<bool>,
}

/// Protocol followed by the provider
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImmunizationProtocolApplied {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name of vaccine series
    pub series: Option<String>,

    /// Who is responsible for publishing the recommendations
    pub authority: Option<Box<Reference>>,

    /// Vaccine preventatable disease being targetted
    #[serde(rename = "targetDisease")]
    pub target_disease: Option<Vec<CodeableConcept>>,

    /// Dose number within series
    #[serde(rename = "doseNumberPositiveInt")]
    pub dose_number_positive_int: i32,

    #[serde(rename = "doseNumberString")]
    pub dose_number_string: String,

    /// Recommended number of doses for immunity
    #[serde(rename = "seriesDosesPositiveInt")]
    pub series_doses_positive_int: Option<i32>,

    #[serde(rename = "seriesDosesString")]
    pub series_doses_string: Option<String>,
}

/// Describes the event of a patient being administered a vaccine or a record of an immunization as reported by a patient, a clinician or another party.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Immunization {
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

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// completed | entered-in-error | not-done
    pub status: String,

    /// Reason not done
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// Vaccine product administered
    #[serde(rename = "vaccineCode")]
    pub vaccine_code: CodeableConcept,

    /// Who was immunized
    pub patient: Box<Reference>,

    /// Encounter immunization was part of
    pub encounter: Option<Box<Reference>>,

    /// Vaccine administration date
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: String,

    #[serde(rename = "occurrenceString")]
    pub occurrence_string: String,

    /// When the immunization was first captured in the subject's record
    pub recorded: Option<String>,

    /// Indicates context the data was recorded in
    #[serde(rename = "primarySource")]
    pub primary_source: Option<bool>,

    /// Indicates the source of a secondarily reported record
    #[serde(rename = "reportOrigin")]
    pub report_origin: Option<CodeableConcept>,

    /// Where immunization occurred
    pub location: Option<Box<Reference>>,

    /// Vaccine manufacturer
    pub manufacturer: Option<Box<Reference>>,

    /// Vaccine lot number
    #[serde(rename = "lotNumber")]
    pub lot_number: Option<String>,

    /// Vaccine expiration date
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,

    /// Body site vaccine was administered
    pub site: Option<CodeableConcept>,

    /// How vaccine entered body
    pub route: Option<CodeableConcept>,

    /// Amount of vaccine administered
    #[serde(rename = "doseQuantity")]
    pub dose_quantity: Option<Quantity>,

    /// Who performed event
    pub performer: Option<Vec<ImmunizationPerformer>>,

    /// Additional immunization notes
    pub note: Option<Vec<Annotation>>,

    /// Why immunization occurred
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why immunization occurred
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Dose potency
    #[serde(rename = "isSubpotent")]
    pub is_subpotent: Option<bool>,

    /// Reason for being subpotent
    #[serde(rename = "subpotentReason")]
    pub subpotent_reason: Option<Vec<CodeableConcept>>,

    /// Educational material presented to patient
    pub education: Option<Vec<ImmunizationEducation>>,

    /// Patient eligibility for a vaccination program
    #[serde(rename = "programEligibility")]
    pub program_eligibility: Option<Vec<CodeableConcept>>,

    /// Funding source for the vaccine
    #[serde(rename = "fundingSource")]
    pub funding_source: Option<CodeableConcept>,

    /// Details of a reaction that follows immunization
    pub reaction: Option<Vec<ImmunizationReaction>>,

    /// Protocol followed by the provider
    #[serde(rename = "protocolApplied")]
    pub protocol_applied: Option<Vec<ImmunizationProtocolApplied>>,
}
