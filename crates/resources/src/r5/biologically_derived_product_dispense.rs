//! FHIR R5 BiologicallyDerivedProductDispense Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Indicates who or what performed an action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductDispensePerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifies the function of the performer during the dispense
    pub function: Option<CodeableConcept>,

    /// Who performed the action
    pub actor: Box<Reference>,
}

/// A record of dispensation of a biologically derived product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductDispense {
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

    /// Business identifier for this dispense
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The order or request that this dispense is fulfilling
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Short description
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// preparation | in-progress | allocated | issued | unfulfilled | returned | entered-in-error | unknown
    pub status: String,

    /// Relationship between the donor and intended recipient
    #[serde(rename = "originRelationshipType")]
    pub origin_relationship_type: Option<CodeableConcept>,

    /// The BiologicallyDerivedProduct that is dispensed
    pub product: Box<Reference>,

    /// The intended recipient of the dispensed product
    pub patient: Box<Reference>,

    /// Indicates the type of matching associated with the dispense
    #[serde(rename = "matchStatus")]
    pub match_status: Option<CodeableConcept>,

    /// Indicates who or what performed an action
    pub performer: Option<Vec<BiologicallyDerivedProductDispensePerformer>>,

    /// Where the dispense occurred
    pub location: Option<Box<Reference>>,

    /// Amount dispensed
    pub quantity: Option<Quantity>,

    /// When product was selected/matched
    #[serde(rename = "preparedDate")]
    pub prepared_date: Option<String>,

    /// When the product was dispatched
    #[serde(rename = "whenHandedOver")]
    pub when_handed_over: Option<String>,

    /// Where the product was dispatched to
    pub destination: Option<Box<Reference>>,

    /// Additional notes
    pub note: Option<Vec<Annotation>>,

    /// Specific instructions for use
    #[serde(rename = "usageInstruction")]
    pub usage_instruction: Option<String>,
}
