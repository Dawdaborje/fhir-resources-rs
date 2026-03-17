//! FHIR R5 Substance Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Composition information about the substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceIngredient {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Optional amount (concentration)
    pub quantity: Option<Ratio>,

    /// A component of the substance
    pub substance: serde_json::Value,
}

/// A homogeneous material with a definite composition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Substance {
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

    /// Unique identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Is this an instance of a substance or a kind of one
    pub instance: bool,

    /// active | inactive | entered-in-error
    pub status: Option<String>,

    /// What class/type of substance this is
    pub category: Option<Vec<CodeableConcept>>,

    /// What substance this is
    pub code: CodeableReference,

    /// Textual description of the substance, comments
    pub description: Option<String>,

    /// When no longer valid to use
    pub expiry: Option<String>,

    /// Amount of substance in the package
    pub quantity: Option<Quantity>,

    /// Composition information about the substance
    pub ingredient: Option<Vec<SubstanceIngredient>>,
}
