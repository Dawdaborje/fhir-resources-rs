//! FHIR R4 SupplyRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Ordered item details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyRequestParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item detail
    pub code: Option<CodeableConcept>,

    /// Value of detail
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,
}

/// A record of a request for a medication, substance or device used in the healthcare setting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyRequest {
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

    /// Business Identifier for SupplyRequest
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | active | suspended +
    pub status: Option<String>,

    /// The kind of supply (central, non-stock, etc.)
    pub category: Option<CodeableConcept>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Medication, Substance, or Device requested to be supplied
    #[serde(rename = "itemCodeableConcept")]
    pub item_codeable_concept: CodeableConcept,

    #[serde(rename = "itemReference")]
    pub item_reference: Box<Reference>,

    /// The requested amount of the item indicated
    pub quantity: Quantity,

    /// Ordered item details
    pub parameter: Option<Vec<SupplyRequestParameter>>,

    /// When the request should be fulfilled
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// When the request was made
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Individual making the request
    pub requester: Option<Box<Reference>>,

    /// Who is intended to fulfill the request
    pub supplier: Option<Vec<Box<Reference>>>,

    /// The reason why the supply item was requested
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// The reason why the supply item was requested
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// The origin of the supply
    #[serde(rename = "deliverFrom")]
    pub deliver_from: Option<Box<Reference>>,

    /// The destination of the supply
    #[serde(rename = "deliverTo")]
    pub deliver_to: Option<Box<Reference>>,
}
