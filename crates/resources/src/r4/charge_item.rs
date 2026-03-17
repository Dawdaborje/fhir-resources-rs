//! FHIR R4 ChargeItem Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Who performed charged service
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What type of performance was done
    pub function: Option<CodeableConcept>,

    /// Individual who was performing
    pub actor: Box<Reference>,
}

/// The resource ChargeItem describes the provision of healthcare provider products for a certain patient, therefore referring not only to the product, but containing in addition details of the provisi...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItem {
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

    /// Business Identifier for item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Defining information about the code of this charge item
    #[serde(rename = "definitionUri")]
    pub definition_uri: Option<Vec<String>>,

    /// Resource defining the code of this ChargeItem
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<Vec<String>>,

    /// planned | billable | not-billable | aborted | billed | entered-in-error | unknown
    pub status: String,

    /// Part of referenced ChargeItem
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// A code that identifies the charge, like a billing code
    pub code: CodeableConcept,

    /// Individual service was done for/to
    pub subject: Box<Reference>,

    /// Encounter / Episode associated with event
    pub context: Option<Box<Reference>>,

    /// When the charged service was applied
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// Who performed charged service
    pub performer: Option<Vec<ChargeItemPerformer>>,

    /// Organization providing the charged service
    #[serde(rename = "performingOrganization")]
    pub performing_organization: Option<Box<Reference>>,

    /// Organization requesting the charged service
    #[serde(rename = "requestingOrganization")]
    pub requesting_organization: Option<Box<Reference>>,

    /// Organization that has ownership of the (potential, future) revenue
    #[serde(rename = "costCenter")]
    pub cost_center: Option<Box<Reference>>,

    /// Quantity of which the charge item has been serviced
    pub quantity: Option<Quantity>,

    /// Anatomical location, if relevant
    pub bodysite: Option<Vec<CodeableConcept>>,

    /// Factor overriding the associated rules
    #[serde(rename = "factorOverride")]
    pub factor_override: Option<f64>,

    /// Price overriding the associated rules
    #[serde(rename = "priceOverride")]
    pub price_override: Option<Money>,

    /// Reason for overriding the list price/factor
    #[serde(rename = "overrideReason")]
    pub override_reason: Option<String>,

    /// Individual who was entering
    pub enterer: Option<Box<Reference>>,

    /// Date the charge item was entered
    #[serde(rename = "enteredDate")]
    pub entered_date: Option<String>,

    /// Why was the charged service rendered?
    pub reason: Option<Vec<CodeableConcept>>,

    /// Which rendered service is being charged?
    pub service: Option<Vec<Box<Reference>>>,

    /// Product charged
    #[serde(rename = "productReference")]
    pub product_reference: Option<Box<Reference>>,

    #[serde(rename = "productCodeableConcept")]
    pub product_codeable_concept: Option<CodeableConcept>,

    /// Account to place this charge
    pub account: Option<Vec<Box<Reference>>>,

    /// Comments made about the ChargeItem
    pub note: Option<Vec<Annotation>>,

    /// Further information supporting this charge
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<Vec<Box<Reference>>>,
}
