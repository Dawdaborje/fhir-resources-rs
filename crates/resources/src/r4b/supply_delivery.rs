//! FHIR R4B SupplyDelivery Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The item that is delivered or supplied
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDeliverySuppliedItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Amount dispensed
    pub quantity: Option<Quantity>,

    /// Medication, Substance, or Device supplied
    pub item: Option<serde_json::Value>,
}

/// Record of delivery of what is supplied.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupplyDelivery {
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

    /// Fulfills plan, proposal or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// in-progress | completed | abandoned | entered-in-error
    pub status: Option<String>,

    /// Patient for whom the item is supplied
    pub patient: Option<Box<Reference>>,

    /// Category of dispense event
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The item that is delivered or supplied
    #[serde(rename = "suppliedItem")]
    pub supplied_item: Option<SupplyDeliverySuppliedItem>,

    /// When event occurred
    pub occurrence: Option<serde_json::Value>,

    /// Dispenser
    pub supplier: Option<Box<Reference>>,

    /// Where the Supply was sent
    pub destination: Option<Box<Reference>>,

    /// Who collected the Supply
    pub receiver: Option<Vec<Box<Reference>>>,
}
