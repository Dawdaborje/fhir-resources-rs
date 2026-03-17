//! FHIR R4 CoverageEligibilityRequest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Supporting information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestSupportingInfo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Information instance identifier
    pub sequence: i32,

    /// Data to be provided
    pub information: Box<Reference>,

    /// Applies to all items
    #[serde(rename = "appliesToAll")]
    pub applies_to_all: Option<bool>,
}

/// Patient insurance information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestInsurance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Applicable coverage
    pub focal: Option<bool>,

    /// Insurance information
    pub coverage: Box<Reference>,

    /// Additional provider contract number
    #[serde(rename = "businessArrangement")]
    pub business_arrangement: Option<String>,
}

/// Item to be evaluated for eligibiity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Applicable exception or supporting information
    #[serde(rename = "supportingInfoSequence")]
    pub supporting_info_sequence: Option<Vec<i32>>,

    /// Benefit classification
    pub category: Option<CodeableConcept>,

    /// Billing, service, product, or drug code
    #[serde(rename = "productOrService")]
    pub product_or_service: Option<CodeableConcept>,

    /// Product or service billing modifiers
    pub modifier: Option<Vec<CodeableConcept>>,

    /// Perfoming practitioner
    pub provider: Option<Box<Reference>>,

    /// Count of products or services
    pub quantity: Option<Quantity>,

    /// Fee, charge or cost per item
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Servicing facility
    pub facility: Option<Box<Reference>>,

    /// Applicable diagnosis
    pub diagnosis: Option<Vec<CoverageEligibilityRequestItemDiagnosis>>,

    /// Product or service details
    pub detail: Option<Vec<Box<Reference>>>,
}

/// Applicable diagnosis
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequestItemDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Nature of illness or problem
    #[serde(rename = "diagnosisCodeableConcept")]
    pub diagnosis_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "diagnosisReference")]
    pub diagnosis_reference: Option<Box<Reference>>,
}

/// The CoverageEligibilityRequest provides patient and insurance coverage information to an insurer for them to respond, in the form of an CoverageEligibilityResponse, with information regarding wheth...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageEligibilityRequest {
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

    /// Desired processing priority
    pub priority: Option<CodeableConcept>,

    /// auth-requirements | benefits | discovery | validation
    pub purpose: Vec<String>,

    /// Intended recipient of products and services
    pub patient: Box<Reference>,

    /// Estimated date or dates of service
    #[serde(rename = "servicedDate")]
    pub serviced_date: Option<String>,

    #[serde(rename = "servicedPeriod")]
    pub serviced_period: Option<Period>,

    /// Creation date
    pub created: String,

    /// Author
    pub enterer: Option<Box<Reference>>,

    /// Party responsible for the request
    pub provider: Option<Box<Reference>>,

    /// Coverage issuer
    pub insurer: Box<Reference>,

    /// Servicing facility
    pub facility: Option<Box<Reference>>,

    /// Supporting information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<CoverageEligibilityRequestSupportingInfo>>,

    /// Patient insurance information
    pub insurance: Option<Vec<CoverageEligibilityRequestInsurance>>,

    /// Item to be evaluated for eligibiity
    pub item: Option<Vec<CoverageEligibilityRequestItem>>,
}
