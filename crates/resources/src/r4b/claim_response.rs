//! FHIR R4B ClaimResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Adjudication for claim line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Claim item instance identifier
    #[serde(rename = "itemSequence")]
    pub item_sequence: i32,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Adjudication details
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Adjudication for claim details
    pub detail: Option<Vec<ClaimResponseItemDetail>>,
}

/// Adjudication details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemAdjudication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of adjudication information
    pub category: CodeableConcept,

    /// Explanation of adjudication outcome
    pub reason: Option<CodeableConcept>,

    /// Monetary amount
    pub amount: Option<Money>,

    /// Non-monetary value
    pub value: Option<f64>,
}

/// Adjudication for claim details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Claim detail instance identifier
    #[serde(rename = "detailSequence")]
    pub detail_sequence: i32,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Detail level adjudication details
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Adjudication for claim sub-details
    #[serde(rename = "subDetail")]
    pub sub_detail: Option<Vec<ClaimResponseItemDetailSubDetail>>,
}

/// Adjudication for claim sub-details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Claim sub-detail instance identifier
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: i32,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
}

/// Insurer added line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item sequence number
    #[serde(rename = "itemSequence")]
    pub item_sequence: Option<Vec<i32>>,

    /// Detail sequence number
    #[serde(rename = "detailSequence")]
    pub detail_sequence: Option<Vec<i32>>,

    /// Subdetail sequence number
    #[serde(rename = "subdetailSequence")]
    pub subdetail_sequence: Option<Vec<i32>>,

    /// Authorized providers
    pub provider: Option<Vec<Box<Reference>>>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Program the product or service is provided under
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,

    /// Date or dates of service or product delivery
    pub serviced: Option<serde_json::Value>,

    /// Place of service or where product was supplied
    pub location: Option<serde_json::Value>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total item cost
    pub net: Option<Money>,

    /// Anatomical location
    #[serde(rename = "bodySite")]
    pub body_site: Option<CodeableConcept>,

    /// Anatomical sub-location
    #[serde(rename = "subSite")]
    pub sub_site: Option<Vec<CodeableConcept>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items adjudication
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Insurer added line details
    pub detail: Option<Vec<ClaimResponseAddItemDetail>>,
}

/// Insurer added line details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total item cost
    pub net: Option<Money>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items detail adjudication
    pub adjudication: Vec<ClaimResponseItemAdjudication>,

    /// Insurer added line items
    #[serde(rename = "subDetail")]
    pub sub_detail: Option<Vec<ClaimResponseAddItemDetailSubDetail>>,
}

/// Insurer added line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: CodeableConcept,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total item cost
    pub net: Option<Money>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items detail adjudication
    pub adjudication: Vec<ClaimResponseItemAdjudication>,
}

/// Adjudication totals
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseTotal {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of adjudication information
    pub category: CodeableConcept,

    /// Financial total for the category
    pub amount: Money,
}

/// Payment Details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponsePayment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Partial or complete payment
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Payment adjustment for non-claim issues
    pub adjustment: Option<Money>,

    /// Explanation for the adjustment
    #[serde(rename = "adjustmentReason")]
    pub adjustment_reason: Option<CodeableConcept>,

    /// Expected date of payment
    pub date: Option<String>,

    /// Payable amount after adjustment
    pub amount: Money,

    /// Business identifier for the payment
    pub identifier: Option<Box<Identifier>>,
}

/// Note concerning adjudication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseProcessNote {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Note instance identifier
    pub number: Option<i32>,

    /// display | print | printoper
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Note explanatory text
    pub text: String,

    /// Language of the text
    pub language: Option<CodeableConcept>,
}

/// Patient insurance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Insurance instance identifier
    pub sequence: i32,

    /// Coverage to be used for adjudication
    pub focal: bool,

    /// Insurance information
    pub coverage: Box<Reference>,

    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<String>,

    /// Adjudication results
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Box<Reference>>,
}

/// Processing errors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item sequence number
    #[serde(rename = "itemSequence")]
    pub item_sequence: Option<i32>,

    /// Detail sequence number
    #[serde(rename = "detailSequence")]
    pub detail_sequence: Option<i32>,

    /// Subdetail sequence number
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: Option<i32>,

    /// Error code detailing processing issues
    pub code: CodeableConcept,
}

/// This resource provides the adjudication details from the processing of a Claim resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
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

    /// Business Identifier for a claim response
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// More granular claim type
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// More granular claim type
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,

    /// claim | preauthorization | predetermination
    #[serde(rename = "use")]
    pub r#use: String,

    /// The recipient of the products and services
    pub patient: Box<Reference>,

    /// Response creation date
    pub created: String,

    /// Party responsible for reimbursement
    pub insurer: Box<Reference>,

    /// Party responsible for the claim
    pub requestor: Option<Box<Reference>>,

    /// Id of resource triggering adjudication
    pub request: Option<Box<Reference>>,

    /// queued | complete | error | partial
    pub outcome: String,

    /// Disposition Message
    pub disposition: Option<String>,

    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<String>,

    /// Preauthorization reference effective period
    #[serde(rename = "preAuthPeriod")]
    pub pre_auth_period: Option<Period>,

    /// Party to be paid any benefits payable
    #[serde(rename = "payeeType")]
    pub payee_type: Option<CodeableConcept>,

    /// Adjudication for claim line items
    pub item: Option<Vec<ClaimResponseItem>>,

    /// Insurer added line items
    #[serde(rename = "addItem")]
    pub add_item: Option<Vec<ClaimResponseAddItem>>,

    /// Header-level adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Adjudication totals
    pub total: Option<Vec<ClaimResponseTotal>>,

    /// Payment Details
    pub payment: Option<ClaimResponsePayment>,

    /// Funds reserved status
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,

    /// Printed form identifier
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,

    /// Printed reference or actual form
    pub form: Option<Attachment>,

    /// Note concerning adjudication
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<ClaimResponseProcessNote>>,

    /// Request for additional information
    #[serde(rename = "communicationRequest")]
    pub communication_request: Option<Vec<Box<Reference>>>,

    /// Patient insurance information
    pub insurance: Option<Vec<ClaimResponseInsurance>>,

    /// Processing errors
    pub error: Option<Vec<ClaimResponseError>>,
}
