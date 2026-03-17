//! FHIR R4B PaymentReconciliation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Settlement particulars
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Business identifier of the payment detail
    pub identifier: Option<Box<Identifier>>,

    /// Business identifier of the prior payment detail
    pub predecessor: Option<Box<Identifier>>,

    /// Category of payment
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Request giving rise to the payment
    pub request: Option<Box<Reference>>,

    /// Submitter of the request
    pub submitter: Option<Box<Reference>>,

    /// Response committing to a payment
    pub response: Option<Box<Reference>>,

    /// Date of commitment to pay
    pub date: Option<String>,

    /// Contact for the response
    pub responsible: Option<Box<Reference>>,

    /// Recipient of the payment
    pub payee: Option<Box<Reference>>,

    /// Amount allocated to this payable
    pub amount: Option<Money>,
}

/// Note concerning processing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// display | print | printoper
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Note explanatory text
    pub text: Option<String>,
}

/// This resource provides the details including amount of a payment and allocates the payment items being paid.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliation {
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

    /// Business Identifier for a payment reconciliation
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Period covered
    pub period: Option<Period>,

    /// Creation date
    pub created: String,

    /// Party generating payment
    #[serde(rename = "paymentIssuer")]
    pub payment_issuer: Option<Box<Reference>>,

    /// Reference to requesting resource
    pub request: Option<Box<Reference>>,

    /// Responsible practitioner
    pub requestor: Option<Box<Reference>>,

    /// queued | complete | error | partial
    pub outcome: Option<String>,

    /// Disposition message
    pub disposition: Option<String>,

    /// When payment issued
    #[serde(rename = "paymentDate")]
    pub payment_date: String,

    /// Total amount of Payment
    #[serde(rename = "paymentAmount")]
    pub payment_amount: Money,

    /// Business identifier for the payment
    #[serde(rename = "paymentIdentifier")]
    pub payment_identifier: Option<Box<Identifier>>,

    /// Settlement particulars
    pub detail: Option<Vec<PaymentReconciliationDetail>>,

    /// Printed form identifier
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,

    /// Note concerning processing
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<PaymentReconciliationProcessNote>>,
}
