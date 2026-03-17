//! FHIR R4 Invoice Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Participant in creation of this Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of involvement in creation of this Invoice
    pub role: Option<CodeableConcept>,

    /// Individual who was involved
    pub actor: Box<Reference>,
}

/// Line items of this Invoice
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Sequence number of line item
    pub sequence: Option<i32>,

    /// Reference to ChargeItem containing details of this line item or an inline billing code
    #[serde(rename = "chargeItem")]
    pub charge_item: serde_json::Value,

    /// Components of total line item price
    #[serde(rename = "priceComponent")]
    pub price_component: Option<Vec<InvoiceLineItemPriceComponent>>,
}

/// Components of total line item price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLineItemPriceComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// base | surcharge | deduction | discount | tax | informational
    #[serde(rename = "type")]
    pub r#type: String,

    /// Code identifying the specific component
    pub code: Option<CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<f64>,

    /// Monetary amount associated with this component
    pub amount: Option<Money>,
}

/// Invoice containing collected ChargeItems from an Account with calculated individual and total price for Billing purpose.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
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

    /// draft | issued | balanced | cancelled | entered-in-error
    pub status: String,

    /// Reason for cancellation of this Invoice
    #[serde(rename = "cancelledReason")]
    pub cancelled_reason: Option<String>,

    /// Type of Invoice
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Recipient(s) of goods and services
    pub subject: Option<Box<Reference>>,

    /// Recipient of this invoice
    pub recipient: Option<Box<Reference>>,

    /// Invoice date / posting date
    pub date: Option<String>,

    /// Participant in creation of this Invoice
    pub participant: Option<Vec<InvoiceParticipant>>,

    /// Issuing Organization of Invoice
    pub issuer: Option<Box<Reference>>,

    /// Account that is being balanced
    pub account: Option<Box<Reference>>,

    /// Line items of this Invoice
    #[serde(rename = "lineItem")]
    pub line_item: Option<Vec<InvoiceLineItem>>,

    /// Components of Invoice total
    #[serde(rename = "totalPriceComponent")]
    pub total_price_component: Option<Vec<InvoiceLineItemPriceComponent>>,

    /// Net total of this Invoice
    #[serde(rename = "totalNet")]
    pub total_net: Option<Money>,

    /// Gross total of this Invoice
    #[serde(rename = "totalGross")]
    pub total_gross: Option<Money>,

    /// Payment details
    #[serde(rename = "paymentTerms")]
    pub payment_terms: Option<String>,

    /// Comments made about the invoice
    pub note: Option<Vec<Annotation>>,
}
