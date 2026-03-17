//! FHIR R5 PaymentReconciliation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Settlement particulars
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentReconciliationAllocation {
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

    /// Subject of the payment
    pub target: Option<Box<Reference>>,

    /// Sub-element of the subject
    #[serde(rename = "targetItemString")]
    pub target_item_string: Option<String>,

    #[serde(rename = "targetItemIdentifier")]
    pub target_item_identifier: Option<Box<Identifier>>,

    #[serde(rename = "targetItemPositiveInt")]
    pub target_item_positive_int: Option<i32>,

    /// Applied-to encounter
    pub encounter: Option<Box<Reference>>,

    /// Applied-to account
    pub account: Option<Box<Reference>>,

    /// Category of payment
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

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

    /// Category of payment
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Workflow originating payment
    pub kind: Option<CodeableConcept>,

    /// Period covered
    pub period: Option<Period>,

    /// Creation date
    pub created: String,

    /// Who entered the payment
    pub enterer: Option<Box<Reference>>,

    /// Nature of the source
    #[serde(rename = "issuerType")]
    pub issuer_type: Option<CodeableConcept>,

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
    pub date: String,

    /// Where payment collected
    pub location: Option<Box<Reference>>,

    /// Payment instrument
    pub method: Option<CodeableConcept>,

    /// Type of card
    #[serde(rename = "cardBrand")]
    pub card_brand: Option<String>,

    /// Digits for verification
    #[serde(rename = "accountNumber")]
    pub account_number: Option<String>,

    /// Expiration year-month
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,

    /// Processor name
    pub processor: Option<String>,

    /// Check number or payment reference
    #[serde(rename = "referenceNumber")]
    pub reference_number: Option<String>,

    /// Authorization number
    pub authorization: Option<String>,

    /// Amount offered by the issuer
    #[serde(rename = "tenderedAmount")]
    pub tendered_amount: Option<Money>,

    /// Amount returned by the receiver
    #[serde(rename = "returnedAmount")]
    pub returned_amount: Option<Money>,

    /// Total amount of Payment
    pub amount: Money,

    /// Business identifier for the payment
    #[serde(rename = "paymentIdentifier")]
    pub payment_identifier: Option<Box<Identifier>>,

    /// Settlement particulars
    pub allocation: Option<Vec<PaymentReconciliationAllocation>>,

    /// Printed form identifier
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,

    /// Note concerning processing
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<PaymentReconciliationProcessNote>>,
}
