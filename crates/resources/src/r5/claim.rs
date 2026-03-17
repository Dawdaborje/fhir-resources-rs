//! FHIR R5 Claim Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Prior or corollary claims
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRelated {
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

/// Recipient of benefits payable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimPayee {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Category of recipient
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Recipient reference
    pub party: Option<Box<Reference>>,
}

/// Event information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimEvent {
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

/// Members of the care team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimCareTeam {
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
pub struct ClaimSupportingInfo {
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
    pub reason: Option<CodeableConcept>,
}

/// Pertinent diagnosis information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimDiagnosis {
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
pub struct ClaimProcedure {
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
pub struct ClaimInsurance {
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

    /// Pre-assigned Claim number
    pub identifier: Option<Box<Identifier>>,

    /// Insurance information
    pub coverage: Box<Reference>,

    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<String>,

    /// Prior authorization reference number
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<Vec<String>>,

    /// Adjudication results
    #[serde(rename = "claimResponse")]
    pub claim_response: Option<Box<Reference>>,
}

/// Details of the event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimAccident {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// When the incident occurred
    pub date: String,

    /// The nature of the accident
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Where the event occurred
    pub location: Option<serde_json::Value>,
}

/// Product or service provided
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item instance identifier
    pub sequence: i32,

    /// Number for tracking
    #[serde(rename = "traceNumber")]
    pub trace_number: Option<Vec<Box<Identifier>>>,

    /// Applicable careTeam members
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
    pub body_site: Option<Vec<ClaimItemBodySite>>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Product or service provided
    pub detail: Option<Vec<ClaimItemDetail>>,
}

/// Anatomical location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemBodySite {
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

/// Product or service provided
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item instance identifier
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

    /// Product or service provided
    #[serde(rename = "subDetail")]
    pub sub_detail: Option<Vec<ClaimItemDetailSubDetail>>,
}

/// Product or service provided
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimItemDetailSubDetail {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Item instance identifier
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
}

/// A provider issued list of professional services and products which have been provided, or are to be provided, to a patient which is sent to an insurer for reimbursement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
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

    /// Business Identifier for claim
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

    /// Resource creation date
    pub created: String,

    /// Author of the claim
    pub enterer: Option<Box<Reference>>,

    /// Target
    pub insurer: Option<Box<Reference>>,

    /// Party responsible for the claim
    pub provider: Option<Box<Reference>>,

    /// Desired processing urgency
    pub priority: Option<CodeableConcept>,

    /// For whom to reserve funds
    #[serde(rename = "fundsReserve")]
    pub funds_reserve: Option<CodeableConcept>,

    /// Prior or corollary claims
    pub related: Option<Vec<ClaimRelated>>,

    /// Prescription authorizing services and products
    pub prescription: Option<Box<Reference>>,

    /// Original prescription if superseded by fulfiller
    #[serde(rename = "originalPrescription")]
    pub original_prescription: Option<Box<Reference>>,

    /// Recipient of benefits payable
    pub payee: Option<ClaimPayee>,

    /// Treatment referral
    pub referral: Option<Box<Reference>>,

    /// Encounters associated with the listed treatments
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Servicing facility
    pub facility: Option<Box<Reference>>,

    /// Package billing code
    #[serde(rename = "diagnosisRelatedGroup")]
    pub diagnosis_related_group: Option<CodeableConcept>,

    /// Event information
    pub event: Option<Vec<ClaimEvent>>,

    /// Members of the care team
    #[serde(rename = "careTeam")]
    pub care_team: Option<Vec<ClaimCareTeam>>,

    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<ClaimSupportingInfo>>,

    /// Pertinent diagnosis information
    pub diagnosis: Option<Vec<ClaimDiagnosis>>,

    /// Clinical procedures performed
    pub procedure: Option<Vec<ClaimProcedure>>,

    /// Patient insurance information
    pub insurance: Option<Vec<ClaimInsurance>>,

    /// Details of the event
    pub accident: Option<ClaimAccident>,

    /// Paid by the patient
    #[serde(rename = "patientPaid")]
    pub patient_paid: Option<Money>,

    /// Product or service provided
    pub item: Option<Vec<ClaimItem>>,

    /// Total claim cost
    pub total: Option<Money>,
}
