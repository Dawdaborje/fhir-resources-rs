//! FHIR R5 Account Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The party(s), such as insurances, that may contribute to the payment of this account
    pub coverage: Box<Reference>,

    /// The priority of the coverage in the context of this account
    pub priority: Option<i32>,
}

/// The parties ultimately responsible for balancing the Account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountGuarantor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Responsible entity
    pub party: Box<Reference>,

    /// Credit or other hold applied
    #[serde(rename = "onHold")]
    pub on_hold: Option<bool>,

    /// Guarantee account during
    pub period: Option<Period>,
}

/// The list of diagnoses relevant to this account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountDiagnosis {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Ranking of the diagnosis (for each type)
    pub sequence: Option<i32>,

    /// The diagnosis relevant to the account
    pub condition: CodeableReference,

    /// Date of the diagnosis (when coded diagnosis)
    #[serde(rename = "dateOfDiagnosis")]
    pub date_of_diagnosis: Option<String>,

    /// Type that this diagnosis has relevant to the account (e.g. admission, billing, discharge …)
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Diagnosis present on Admission
    #[serde(rename = "onAdmission")]
    pub on_admission: Option<bool>,

    /// Package Code specific for billing
    #[serde(rename = "packageCode")]
    pub package_code: Option<Vec<CodeableConcept>>,
}

/// The list of procedures relevant to this account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Ranking of the procedure (for each type)
    pub sequence: Option<i32>,

    /// The procedure relevant to the account
    pub code: CodeableReference,

    /// Date of the procedure (when coded procedure)
    #[serde(rename = "dateOfService")]
    pub date_of_service: Option<String>,

    /// How this procedure value should be used in charging the account
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Package Code specific for billing
    #[serde(rename = "packageCode")]
    pub package_code: Option<Vec<CodeableConcept>>,

    /// Any devices that were associated with the procedure
    pub device: Option<Vec<Box<Reference>>>,
}

/// Other associated accounts related to this account
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRelatedAccount {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Relationship of the associated Account
    pub relationship: Option<CodeableConcept>,

    /// Reference to an associated Account
    pub account: Box<Reference>,
}

/// Calculated account balance(s)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Who is expected to pay this part of the balance
    pub aggregate: Option<CodeableConcept>,

    /// current | 30 | 60 | 90 | 120
    pub term: Option<CodeableConcept>,

    /// Estimated balance
    pub estimate: Option<bool>,

    /// Calculated amount
    pub amount: Money,
}

/// A financial tool for tracking value accrued for a particular purpose. In the healthcare field, used to track charges for a patient, cost centers, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
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

    /// Account number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | inactive | entered-in-error | on-hold | unknown
    pub status: String,

    /// Tracks the lifecycle of the account through the billing process
    #[serde(rename = "billingStatus")]
    pub billing_status: Option<CodeableConcept>,

    /// E.g. patient, expense, depreciation
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Human-readable label
    pub name: Option<String>,

    /// The entity that caused the expenses
    pub subject: Option<Vec<Box<Reference>>>,

    /// Transaction window
    #[serde(rename = "servicePeriod")]
    pub service_period: Option<Period>,

    /// The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account
    pub coverage: Option<Vec<AccountCoverage>>,

    /// Entity managing the Account
    pub owner: Option<Box<Reference>>,

    /// Explanation of purpose/use
    pub description: Option<String>,

    /// The parties ultimately responsible for balancing the Account
    pub guarantor: Option<Vec<AccountGuarantor>>,

    /// The list of diagnoses relevant to this account
    pub diagnosis: Option<Vec<AccountDiagnosis>>,

    /// The list of procedures relevant to this account
    pub procedure: Option<Vec<AccountProcedure>>,

    /// Other associated accounts related to this account
    #[serde(rename = "relatedAccount")]
    pub related_account: Option<Vec<AccountRelatedAccount>>,

    /// The base or default currency
    pub currency: Option<CodeableConcept>,

    /// Calculated account balance(s)
    pub balance: Option<Vec<AccountBalance>>,

    /// Time the balance amount was calculated
    #[serde(rename = "calculatedAt")]
    pub calculated_at: Option<String>,
}
