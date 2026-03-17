//! FHIR R4 Coverage Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Additional coverage classifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageClass {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of class such as 'group' or 'plan'
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Value associated with the type
    pub value: String,

    /// Human readable description of the type and value
    pub name: Option<String>,
}

/// Patient payments for services/products
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageCostToBeneficiary {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Cost category
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The amount or percentage due from the beneficiary
    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueMoney")]
    pub value_money: Money,

    /// Exceptions for patient payments
    pub exception: Option<Vec<CoverageCostToBeneficiaryException>>,
}

/// Exceptions for patient payments
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverageCostToBeneficiaryException {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Exception category
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The effective period of the exception
    pub period: Option<Period>,
}

/// Financial instrument which may be used to reimburse or pay for health care products and services. Includes both insurance and self-payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coverage {
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

    /// Business Identifier for the coverage
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | cancelled | draft | entered-in-error
    pub status: String,

    /// Coverage category such as medical or accident
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Owner of the policy
    #[serde(rename = "policyHolder")]
    pub policy_holder: Option<Box<Reference>>,

    /// Subscriber to the policy
    pub subscriber: Option<Box<Reference>>,

    /// ID assigned to the subscriber
    #[serde(rename = "subscriberId")]
    pub subscriber_id: Option<String>,

    /// Plan beneficiary
    pub beneficiary: Box<Reference>,

    /// Dependent number
    pub dependent: Option<String>,

    /// Beneficiary relationship to the subscriber
    pub relationship: Option<CodeableConcept>,

    /// Coverage start and end dates
    pub period: Option<Period>,

    /// Issuer of the policy
    pub payor: Vec<Box<Reference>>,

    /// Additional coverage classifications
    pub class: Option<Vec<CoverageClass>>,

    /// Relative order of the coverage
    pub order: Option<i32>,

    /// Insurer network
    pub network: Option<String>,

    /// Patient payments for services/products
    #[serde(rename = "costToBeneficiary")]
    pub cost_to_beneficiary: Option<Vec<CoverageCostToBeneficiary>>,

    /// Reimbursement to insurer
    pub subrogation: Option<bool>,

    /// Contract details
    pub contract: Option<Vec<Box<Reference>>>,
}
