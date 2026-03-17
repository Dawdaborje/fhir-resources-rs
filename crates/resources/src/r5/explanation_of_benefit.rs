//! FHIR R5 ExplanationOfBenefit Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Prior or corollary claims
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitRelated {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to the related claim
    pub claim: Option<Box<Reference>>,

    /// How the reference claim is related
    pub relationship: Option<CodeableConcept>,

    /// File or case reference
    pub reference: Option<Box<Identifier>>,
}

/// Event information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitEvent {
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
    pub when: serde_json::Value,
}

/// Recipient of benefits payable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitPayee {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category of recipient
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Recipient reference
    pub party: Option<Box<Reference>>,
}

/// Care Team members
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitCareTeam {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Order of care team
    pub sequence: i32,

    /// Practitioner or organization
    pub provider: Box<Reference>,

    /// Indicator of the lead practitioner
    pub responsible: Option<bool>,

    /// Function within the team
    pub role: Option<CodeableConcept>,

    /// Practitioner or provider specialization
    pub specialty: Option<CodeableConcept>,
}

/// Supporting information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Information instance identifier
    pub sequence: i32,

    /// Classification of the supplied information
    pub category: CodeableConcept,

    /// Type of information
    pub code: Option<CodeableConcept>,

    /// When it occurred
    pub timing: Option<serde_json::Value>,

    /// Data to be provided
    pub value: Option<serde_json::Value>,

    /// Explanation for the information
    pub reason: Option<Coding>,
}

/// Pertinent diagnosis information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Diagnosis instance identifier
    pub sequence: i32,

    /// Nature of illness or problem
    pub diagnosis: serde_json::Value,

    /// Timing or nature of the diagnosis
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Present on admission
    #[serde(rename = "onAdmission")]
    pub on_admission: Option<CodeableConcept>,
}

/// Clinical procedures performed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Procedure instance identifier
    pub sequence: i32,

    /// Category of Procedure
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// When the procedure was performed
    pub date: Option<String>,

    /// Specific clinical procedure
    pub procedure: serde_json::Value,

    /// Unique device identifier
    pub udi: Option<Vec<Box<Reference>>>,
}

/// Patient insurance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coverage to be used for adjudication
    pub focal: bool,

    /// Insurance information
    pub coverage: Box<Reference>,

    /// Prior authorization reference number
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<Vec<String>>,
}

/// Details of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAccident {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// When the incident occurred
    pub date: Option<String>,

    /// The nature of the accident
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Where the event occurred
    pub location: Option<serde_json::Value>,
}

/// Product or service provided
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item instance identifier
    pub sequence: i32,

    /// Applicable care team members
    #[serde(rename = "careTeamSequence")]
    pub care_team_sequence: Option<Vec<i32>>,

    /// Applicable diagnoses
    #[serde(rename = "diagnosisSequence")]
    pub diagnosis_sequence: Option<Vec<i32>>,

    /// Applicable procedures
    #[serde(rename = "procedureSequence")]
    pub procedure_sequence: Option<Vec<i32>>,

    /// Applicable exception and supporting information
    #[serde(rename = "informationSequence")]
    pub information_sequence: Option<Vec<i32>>,

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Benefit classification
    pub category: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Request or Referral for Service
    pub request: Option<Vec<Box<Reference>>>,

    /// Product or service billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Program the product or service is provided under
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,

    /// Date or dates of service or product delivery
    pub serviced: Option<serde_json::Value>,

    /// Place of service or where product was supplied
    pub location: Option<serde_json::Value>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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

    /// Unique device identifier
    pub udi: Option<Vec<Box<Reference>>>,

    /// Anatomical location
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<ExplanationOfBenefitItemBodySite>>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,

    /// Additional items
    pub detail: Option<Vec<ExplanationOfBenefitItemDetail>>,
}

/// Anatomical location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemBodySite {
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

/// Adjudication results
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemReviewOutcome {
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
pub struct ExplanationOfBenefitItemAdjudication {
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

    /// Non-monitary value
    pub quantity: Option<Quantity>,
}

/// Additional items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Product or service provided
    pub sequence: i32,

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Benefit classification
    pub category: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Program the product or service is provided under
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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

    /// Unique device identifier
    pub udi: Option<Vec<Box<Reference>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Detail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Detail level adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,

    /// Additional items
    #[serde(rename = "subDetail")]
    pub sub_detail: Option<Vec<ExplanationOfBenefitItemDetailSubDetail>>,
}

/// Additional items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Product or service provided
    pub sequence: i32,

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Revenue or cost center code
    pub revenue: Option<CodeableConcept>,

    /// Benefit classification
    pub category: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// End of a range of codes
    #[serde(rename = "productOrServiceEnd")]
    pub product_or_service_end: Option<CodeableConcept>,

    /// Service/Product billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Program the product or service is provided under
    #[serde(rename = "programCode")]
    pub program_code: Option<Vec<CodeableConcept>>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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

    /// Unique device identifier
    pub udi: Option<Vec<Box<Reference>>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Subdetail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Subdetail level adjudication details
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
}

/// Insurer added line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItem {
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
    #[serde(rename = "subDetailSequence")]
    pub sub_detail_sequence: Option<Vec<i32>>,

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
    pub serviced: Option<serde_json::Value>,

    /// Place of service or where product was supplied
    pub location: Option<serde_json::Value>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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
    pub body_site: Option<Vec<ExplanationOfBenefitAddItemBodySite>>,

    /// Applicable note numbers
    #[serde(rename = "noteNumber")]
    pub note_number: Option<Vec<i32>>,

    /// Additem level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,

    /// Insurer added line items
    pub detail: Option<Vec<ExplanationOfBenefitAddItemDetail>>,
}

/// Anatomical location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemBodySite {
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

/// Insurer added line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemDetail {
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

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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

    /// Additem detail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,

    /// Insurer added line items
    #[serde(rename = "subDetail")]
    pub sub_detail: Option<Vec<ExplanationOfBenefitAddItemDetailSubDetail>>,
}

/// Insurer added line items
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitAddItemDetailSubDetail {
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

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

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

    /// Additem subdetail level adjudication results
    #[serde(rename = "reviewOutcome")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcome>,

    /// Added items adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,
}

/// Adjudication totals
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitTotal {
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
pub struct ExplanationOfBenefitPayment {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Partial or complete payment
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Payment adjustment for non-claim issues
    pub adjustment: Option<Money>,

    /// Explanation for the variance
    #[serde(rename = "adjustmentReason")]
    pub adjustment_reason: Option<CodeableConcept>,

    /// Expected date of payment
    pub date: Option<String>,

    /// Payable amount after adjustment
    pub amount: Option<Money>,

    /// Business identifier for the payment
    pub identifier: Option<Box<Identifier>>,
}

/// Note concerning adjudication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitProcessNote {
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
    pub text: Option<String>,

    /// Language of the text
    pub language: Option<CodeableConcept>,
}

/// Balance by Benefit Category
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitBenefitBalance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Benefit classification
    pub category: CodeableConcept,

    /// Excluded from the plan
    pub excluded: Option<bool>,

    /// Short name for the benefit
    pub name: Option<String>,

    /// Description of the benefit or services covered
    pub description: Option<String>,

    /// In or out of network
    pub network: Option<CodeableConcept>,

    /// Individual or family
    pub unit: Option<CodeableConcept>,

    /// Annual or lifetime
    pub term: Option<CodeableConcept>,

    /// Benefit Summary
    pub financial: Option<Vec<ExplanationOfBenefitBenefitBalanceFinancial>>,
}

/// Benefit Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefitBenefitBalanceFinancial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Benefit classification
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Benefits allowed
    pub allowed: Option<serde_json::Value>,

    /// Benefits used
    pub used: Option<serde_json::Value>,
}

/// This resource provides: the claim details; adjudication details from the processing of a Claim; and optionally account balance information, for informing the subscriber of the benefits provided.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExplanationOfBenefit {
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

    /// Business Identifier for the resource
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Category or discipline
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

    /// Relevant time frame for the claim
    #[serde(rename = "billablePeriod")]
    pub billable_period: Option<Period>,

    /// Response creation date
    pub created: String,

    /// Author of the claim
    pub enterer: Option<Box<Reference>>,

    /// Party responsible for reimbursement
    pub insurer: Option<Box<Reference>>,

    /// Party responsible for the claim
    pub provider: Option<Box<Reference>>,

    /// Desired processing urgency
    pub priority: Option<CodeableConcept>,

    /// For whom to reserve funds
    #[serde(rename = "fundsReserveRequested")]
    pub funds_reserve_requested: Option<CodeableConcept>,

    /// Funds reserved status
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,

    /// Prior or corollary claims
    pub related: Option<Vec<ExplanationOfBenefitRelated>>,

    /// Prescription authorizing services or products
    pub prescription: Option<Box<Reference>>,

    /// Original prescription if superceded by fulfiller
    #[serde(rename = "originalPrescription")]
    pub original_prescription: Option<Box<Reference>>,

    /// Event information
    pub event: Option<Vec<ExplanationOfBenefitEvent>>,

    /// Recipient of benefits payable
    pub payee: Option<ExplanationOfBenefitPayee>,

    /// Treatment Referral
    pub referral: Option<Box<Reference>>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Servicing Facility
    pub facility: Option<Box<Reference>>,

    /// Claim reference
    pub claim: Option<Box<Reference>>,

    /// Claim response reference
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Box<Reference>>,

    /// queued | complete | error | partial
    pub outcome: String,

    /// Result of the adjudication
    pub decision: Option<CodeableConcept>,

    /// Disposition Message
    pub disposition: Option<String>,

    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<Vec<String>>,

    /// Preauthorization in-effect period
    #[serde(rename = "preAuthRefPeriod")]
    pub pre_auth_ref_period: Option<Vec<Period>>,

    /// Package billing code
    #[serde(rename = "diagnosisRelatedGroup")]
    pub diagnosis_related_group: Option<CodeableConcept>,

    /// Care Team members
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<ExplanationOfBenefitCareTeam>>,

    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<ExplanationOfBenefitSupportingInfo>>,

    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ExplanationOfBenefitDiagnosis>>,

    /// Clinical procedures performed
    pub procedure: Option<Vec<ExplanationOfBenefitProcedure>>,

    /// Precedence (primary, secondary, etc.)
    pub precedence: Option<i32>,

    /// Patient insurance information
    pub insurance: Option<Vec<ExplanationOfBenefitInsurance>>,

    /// Details of the event
    pub accident: Option<ExplanationOfBenefitAccident>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

    /// Product or service provided
    pub item: Option<Vec<ExplanationOfBenefitItem>>,

    /// Insurer added line items
    #[serde(rename = "addItem")]
    pub add_item: Option<Vec<ExplanationOfBenefitAddItem>>,

    /// Header-level adjudication
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudication>>,

    /// Adjudication totals
    pub total: Option<Vec<ExplanationOfBenefitTotal>>,

    /// Payment Details
    pub payment: Option<ExplanationOfBenefitPayment>,

    /// Printed form identifier
    #[serde(rename = "formCode")]
    pub form_code: Option<CodeableConcept>,

    /// Printed reference or actual form
    pub form: Option<Attachment>,

    /// Note concerning adjudication
    #[serde(rename = "processNote")]
    pub process_note: Option<Vec<ExplanationOfBenefitProcessNote>>,

    /// When the benefits are applicable
    #[serde(rename = "benefitPeriod")]
    pub benefit_period: Option<Period>,

    /// Balance by Benefit Category
    #[serde(rename = "benefitBalance")]
    pub benefit_balance: Option<Vec<ExplanationOfBenefitBenefitBalance>>,
}
