//! FHIR R4B CoverageEligibilityResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Patient insurance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Insurance information
    pub coverage: Box<Reference>,

    /// Coverage inforce indicator
    pub inforce: Option<bool>,

    /// When the benefits are applicable
    #[serde(rename = "benefitPeriod")]
    pub benefit_period: Option<Period>,

    /// Benefits and authorization details
    pub item: Option<Vec<CoverageEligibilityResponseInsuranceItem>>,
}

/// Benefits and authorization details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsuranceItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Benefit classification
    pub category: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// Product or service billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Performing practitioner
    pub provider: Option<Box<Reference>>,

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
    pub benefit: Option<Vec<CoverageEligibilityResponseInsuranceItemBenefit>>,

    /// Authorization required flag
    #[serde(rename = "authorizationRequired")]
    pub authorization_required: Option<bool>,

    /// Type of required supporting materials
    #[serde(rename = "authorizationSupporting")]
    pub authorization_supporting: Option<Vec<CodeableConcept>>,

    /// Preauthorization requirements endpoint
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: Option<String>,
}

/// Benefit Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseInsuranceItemBenefit {
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

/// Processing errors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponseError {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Error code detailing processing issues
    pub code: CodeableConcept,
}

/// This resource provides eligibility and plan details from the processing of an CoverageEligibilityRequest resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityResponse {
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

    /// Business Identifier for coverage eligiblity request
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<String>,

    /// Intended recipient of products and services
    pub patient: Box<Reference>,

    /// Estimated date or dates of service
    pub serviced: Option<serde_json::Value>,

    /// Response creation date
    pub created: String,

    /// Party responsible for the request
    pub requestor: Option<Box<Reference>>,

    /// Eligibility request reference
    pub request: Box<Reference>,

    /// queued | complete | error | partial
    pub outcome: String,

    /// Disposition Message
    pub disposition: Option<String>,

    /// Coverage issuer
    pub insurer: Box<Reference>,

    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityResponseInsurance>>,

    /// Preauthorization reference
    #[serde(rename = "preAuthRef")]
    pub pre_auth_ref: Option<String>,

    /// Printed form identifier
    pub form: Option<CodeableConcept>,

    /// Processing errors
    pub error: Option<Vec<CoverageEligibilityResponseError>>,
}
