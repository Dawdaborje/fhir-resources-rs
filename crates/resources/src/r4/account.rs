//! FHIR R4 Account Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
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

    /// Reference to a parent Account
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,
}
