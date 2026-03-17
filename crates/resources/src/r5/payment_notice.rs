//! FHIR R5 PaymentNotice Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// This resource provides the status of the payment for goods and services rendered, and the request and response resource references.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentNotice {
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

    /// Business Identifier for the payment notice
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Request reference
    pub request: Option<Box<Reference>>,

    /// Response reference
    pub response: Option<Box<Reference>>,

    /// Creation date
    pub created: String,

    /// Responsible practitioner
    pub reporter: Option<Box<Reference>>,

    /// Payment reference
    pub payment: Option<Box<Reference>>,

    /// Payment or clearing date
    #[serde(rename = "paymentDate")]
    pub payment_date: Option<String>,

    /// Party being paid
    pub payee: Option<Box<Reference>>,

    /// Party being notified
    pub recipient: Box<Reference>,

    /// Monetary amount of the payment
    pub amount: Money,

    /// Issued or cleared Status of the payment
    #[serde(rename = "paymentStatus")]
    pub payment_status: Option<CodeableConcept>,
}
