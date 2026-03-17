//! FHIR R5 BiologicallyDerivedProduct Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// How this product was collected
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductCollection {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Individual performing collection
    pub collector: Option<Box<Reference>>,

    /// The patient who underwent the medical procedure to collect the product or the organization that facilitated the collection
    pub source: Option<Box<Reference>>,

    /// Time of product collection
    pub collected: Option<serde_json::Value>,
}

/// A property that is specific to this BiologicallyDerviedProduct instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code that specifies the property
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Property values
    pub value: serde_json::Value,
}

/// A biological material originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProduct {
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

    /// organ | tissue | fluid | cells | biologicalAgent
    #[serde(rename = "productCategory")]
    pub product_category: Option<Coding>,

    /// A code that identifies the kind of this biologically derived product
    #[serde(rename = "productCode")]
    pub product_code: Option<CodeableConcept>,

    /// The parent biologically-derived product
    pub parent: Option<Vec<Box<Reference>>>,

    /// Request to obtain and/or infuse this product
    pub request: Option<Vec<Box<Reference>>>,

    /// Instance identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[serde(rename = "biologicalSourceEvent")]
    pub biological_source_event: Option<Box<Identifier>>,

    /// Processing facilities responsible for the labeling and distribution of this biologically derived product
    #[serde(rename = "processingFacility")]
    pub processing_facility: Option<Vec<Box<Reference>>>,

    /// A unique identifier for an aliquot of a product
    pub division: Option<String>,

    /// available | unavailable
    #[serde(rename = "productStatus")]
    pub product_status: Option<Coding>,

    /// Date, and where relevant time, of expiration
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,

    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Product storage temperature requirements
    #[serde(rename = "storageTempRequirements")]
    pub storage_temp_requirements: Option<Range>,

    /// A property that is specific to this BiologicallyDerviedProduct instance
    pub property: Option<Vec<BiologicallyDerivedProductProperty>>,
}
