//! FHIR R5 ClaimResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Event information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseEvent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Specific event
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Occurance date or period
    #[serde(rename = "whenDateTime")]
    pub when_date_time: String,

    #[serde(rename = "whenPeriod")]
    pub when_period: Period,
}

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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Adjudication for claim details
    pub detail: Option<Vec<ClaimResponseItemDetail>>,
}

/// Adjudication results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseItemReviewOutcome {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Result of the adjudication
    pub decision: Option<CodeableConcept>,

    /// Reason for result of the adjudication
    pub reason: Option<Vec<CodeableConcept>>,

    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<String>,

    /// Preauthorization reference effective period
    #[serde(rename = "preAuthPeriod")]
    pub pre_auth_period: Option<Period>,
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
    pub quantity: Option<Quantity>,
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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Detail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Detail level adjudication details
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Subdetail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Authorized providers
    pub provider: Option<Vec<Box<Reference>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Request or Referral for Service
    pub request: Option<Vec<Box<Reference>>>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Program the product or service is provided under
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,

    /// Date or dates of service or product delivery
    #[serde(rename = "servicedDate")]
    pub serviced_date: Option<String>,

    #[serde(rename = "servicedPeriod")]
    pub serviced_period: Option<Period>,

    /// Place of service or where product was supplied
    #[serde(rename = "locationCodeableConcept")]
    pub location_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "locationAddress")]
    pub location_address: Option<Address>,

    #[serde(rename = "locationReference")]
    pub location_reference: Option<Box<Reference>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total tax
    pub tax: Option<Money>,

    /// Total item cost
    pub net: Option<Money>,

    /// Anatomical location
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<ClaimResponseAddItemBodySite>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

    /// Insurer added line details
    pub detail: Option<Vec<ClaimResponseAddItemDetail>>,
}

/// Anatomical location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponseAddItemBodySite {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Location
    pub site: Vec<CodeableReference>,

    /// Sub-location
    #[serde(rename = "subSite")]
    pub sub_site: Option<Vec<CodeableConcept>>,
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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total tax
    pub tax: Option<Money>,

    /// Total item cost
    pub net: Option<Money>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items detail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items detail adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,

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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Price scaling factor
    pub factor: Option<f64>,

    /// Total tax
    pub tax: Option<Money>,

    /// Total item cost
    pub net: Option<Money>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Added items subdetail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcome>,

    /// Added items subdetail adjudication
    pub adjudication: Option<Vec<ClaimResponseItemAdjudication>>,
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

    /// Note purpose
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

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

    /// FHIRPath of element(s) related to issue
    pub expression: Option<Vec<String>>,
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

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

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
    pub insurer: Option<Box<Reference>>,

    /// Party responsible for the claim
    pub requestor: Option<Box<Reference>>,

    /// Id of resource triggering adjudication
    pub request: Option<Box<Reference>>,

    /// queued | complete | error | partial
    pub outcome: String,

    /// Result of the adjudication
    pub decision: Option<CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<String>,

    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<String>,

    /// Preauthorization reference effective period
    #[serde(rename = "preAuthPeriod")]
    pub pre_auth_period: Option<Period>,

    /// Event information
    pub event: Option<Vec<ClaimResponseEvent>>,

    /// Party to be paid any benefits payable
    #[serde(rename = "payeeType")]
    pub payee_type: Option<CodeableConcept>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Package billing code
    #[serde(rename = "diagnosisRelatedGroup")]
    pub diagnosis_related_group: Option<CodeableConcept>,

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
