//! FHIR R4 ChargeItemDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Whether or not the billing code is applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinitionApplicability {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Natural language description of the condition
    pub description: Option<String>,

    /// Language of the expression
    pub language: Option<String>,

    /// Boolean-valued expression
    pub expression: Option<String>,
}

/// Group of properties which are applicable under the same conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinitionPropertyGroup {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Conditions under which the priceComponent is applicable
    pub applicability: Option<Vec<ChargeItemDefinitionApplicability>>,

    /// Components of total line item price
    #[serde(rename = "priceComponent")]
    pub price_component: Option<Vec<ChargeItemDefinitionPropertyGroupPriceComponent>>,
}

/// Components of total line item price
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinitionPropertyGroupPriceComponent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// base | surcharge | deduction | discount | tax | informational
    #[serde(rename = "type")]
    pub r#type: String,

    /// Code identifying the specific component
    pub code: Option<CodeableConcept>,

    /// Factor used for calculating this component
    pub factor: Option<f64>,

    /// Monetary amount associated with this component
    pub amount: Option<Money>,
}

/// The ChargeItemDefinition resource provides the properties that apply to the (billing) codes necessary to calculate costs and prices. The properties may differ largely depending on type and realm, t...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeItemDefinition {
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

    /// Canonical identifier for this charge item definition, represented as a URI (globally unique)
    pub url: String,

    /// Additional identifier for the charge item definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the charge item definition
    pub version: Option<String>,

    /// Name for this charge item definition (human friendly)
    pub title: Option<String>,

    /// Underlying externally-defined charge item definition
    #[serde(rename = "derivedFromUri")]
    pub derived_from_uri: Option<Vec<String>>,

    /// A larger definition of which this particular definition is a component or step
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<String>>,

    /// Completed or terminated request(s) whose function is taken by this new request
    pub replaces: Option<Vec<String>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the charge item definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for charge item definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the charge item definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the charge item definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the charge item definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Billing codes or product types this definition applies to
    pub code: Option<CodeableConcept>,

    /// Instances this definition applies to
    pub instance: Option<Vec<Box<Reference>>>,

    /// Whether or not the billing code is applicable
    pub applicability: Option<Vec<ChargeItemDefinitionApplicability>>,

    /// Group of properties which are applicable under the same conditions
    #[serde(rename = "propertyGroup")]
    pub property_group: Option<Vec<ChargeItemDefinitionPropertyGroup>>,
}
