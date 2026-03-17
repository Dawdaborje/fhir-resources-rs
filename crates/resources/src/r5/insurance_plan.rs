//! FHIR R5 InsurancePlan Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Coverage details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of coverage
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// What networks provide coverage
    pub network: Option<Vec<Box<Reference>>>,

    /// List of benefits
    pub benefit: Vec<InsurancePlanCoverageBenefit>,
}

/// List of benefits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of benefit
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Referral requirements
    pub requirement: Option<String>,

    /// Benefit limits
    pub limit: Option<Vec<InsurancePlanCoverageBenefitLimit>>,
}

/// Benefit limits
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanCoverageBenefitLimit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Maximum value allowed
    pub value: Option<Quantity>,

    /// Benefit limit details
    pub code: Option<CodeableConcept>,
}

/// Plan details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlan {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Business Identifier for Product
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Type of plan
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Where product applies
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Box<Reference>>>,

    /// What networks provide coverage
    pub network: Option<Vec<Box<Reference>>>,

    /// Overall costs
    #[serde(rename = "generalCost")]
    pub general_cost: Option<Vec<InsurancePlanPlanGeneralCost>>,

    /// Specific costs
    #[serde(rename = "specificCost")]
    pub specific_cost: Option<Vec<InsurancePlanPlanSpecificCost>>,
}

/// Overall costs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanGeneralCost {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of cost
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Number of enrollees
    #[serde(rename = "groupSize")]
    pub group_size: Option<i32>,

    /// Cost value
    pub cost: Option<Money>,

    /// Additional cost information
    pub comment: Option<String>,
}

/// Specific costs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCost {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// General category of benefit
    pub category: CodeableConcept,

    /// Benefits list
    pub benefit: Option<Vec<InsurancePlanPlanSpecificCostBenefit>>,
}

/// Benefits list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefit {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of specific benefit
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// List of the costs
    pub cost: Option<Vec<InsurancePlanPlanSpecificCostBenefitCost>>,
}

/// List of the costs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlanPlanSpecificCostBenefitCost {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of cost
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// in-network | out-of-network | other
    pub applicability: Option<CodeableConcept>,

    /// Additional information about the cost
    pub qualifiers: Option<Vec<CodeableConcept>>,

    /// The actual cost value
    pub value: Option<Quantity>,
}

/// Details of a Health Insurance product/plan provided by an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsurancePlan {
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

    /// Business Identifier for Product
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | active | retired | unknown
    pub status: Option<String>,

    /// Kind of product
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Official name
    pub name: Option<String>,

    /// Alternate names
    pub alias: Option<Vec<String>>,

    /// When the product is available
    pub period: Option<Period>,

    /// Product issuer
    #[serde(rename = "ownedBy")]
    pub owned_by: Option<Box<Reference>>,

    /// Product administrator
    #[serde(rename = "administeredBy")]
    pub administered_by: Option<Box<Reference>>,

    /// Where product applies
    #[serde(rename = "coverageArea")]
    pub coverage_area: Option<Vec<Box<Reference>>>,

    /// Official contact details relevant to the health insurance plan/product
    pub contact: Option<Vec<ExtendedContactDetail>>,

    /// Technical endpoint
    pub endpoint: Option<Vec<Box<Reference>>>,

    /// What networks are Included
    pub network: Option<Vec<Box<Reference>>>,

    /// Coverage details
    pub coverage: Option<Vec<InsurancePlanCoverage>>,

    /// Plan details
    pub plan: Option<Vec<InsurancePlanPlan>>,
}
