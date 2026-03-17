//! FHIR R4 SubstanceReferenceInformation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGene {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "geneSequenceOrigin")]
    pub gene_sequence_origin: Option<CodeableConcept>,

    /// Todo
    pub gene: Option<CodeableConcept>,

    /// Todo
    pub source: Option<Vec<Box<Reference>>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationGeneElement {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Todo
    pub element: Option<Box<Identifier>>,

    /// Todo
    pub source: Option<Vec<Box<Reference>>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationClassification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    pub domain: Option<CodeableConcept>,

    /// Todo
    pub classification: Option<CodeableConcept>,

    /// Todo
    pub subtype: Option<Vec<CodeableConcept>>,

    /// Todo
    pub source: Option<Vec<Box<Reference>>>,
}

/// Todo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformationTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Todo
    pub target: Option<Box<Identifier>>,

    /// Todo
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Todo
    pub interaction: Option<CodeableConcept>,

    /// Todo
    pub organism: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "organismType")]
    pub organism_type: Option<CodeableConcept>,

    /// Todo
    #[serde(rename = "amountQuantity")]
    pub amount_quantity: Option<Quantity>,

    #[serde(rename = "amountRange")]
    pub amount_range: Option<Range>,

    #[serde(rename = "amountString")]
    pub amount_string: Option<String>,

    /// Todo
    #[serde(rename = "amountType")]
    pub amount_type: Option<CodeableConcept>,

    /// Todo
    pub source: Option<Vec<Box<Reference>>>,
}

/// Todo.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceReferenceInformation {
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

    /// Todo
    pub comment: Option<String>,

    /// Todo
    pub gene: Option<Vec<SubstanceReferenceInformationGene>>,

    /// Todo
    #[serde(rename = "geneElement")]
    pub gene_element: Option<Vec<SubstanceReferenceInformationGeneElement>>,

    /// Todo
    pub classification: Option<Vec<SubstanceReferenceInformationClassification>>,

    /// Todo
    pub target: Option<Vec<SubstanceReferenceInformationTarget>>,
}
