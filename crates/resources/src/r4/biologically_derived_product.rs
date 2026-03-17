//! FHIR R4 BiologicallyDerivedProduct Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
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

    /// Who is product from
    pub source: Option<Box<Reference>>,

    /// Time of product collection
    #[serde(rename = "collectedDateTime")]
    pub collected_date_time: Option<String>,

    #[serde(rename = "collectedPeriod")]
    pub collected_period: Option<Period>,
}

/// Any processing of the product during collection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductProcessing {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of of processing
    pub description: Option<String>,

    /// Procesing code
    pub procedure: Option<CodeableConcept>,

    /// Substance added during processing
    pub additive: Option<Box<Reference>>,

    /// Time of processing
    #[serde(rename = "timeDateTime")]
    pub time_date_time: Option<String>,

    #[serde(rename = "timePeriod")]
    pub time_period: Option<Period>,
}

/// Any manipulation of product post-collection
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductManipulation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of manipulation
    pub description: Option<String>,

    /// Time of manipulation
    #[serde(rename = "timeDateTime")]
    pub time_date_time: Option<String>,

    #[serde(rename = "timePeriod")]
    pub time_period: Option<Period>,
}

/// Product storage
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiologicallyDerivedProductStorage {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of storage
    pub description: Option<String>,

    /// Storage temperature
    pub temperature: Option<f64>,

    /// farenheit | celsius | kelvin
    pub scale: Option<String>,

    /// Storage timeperiod
    pub duration: Option<Period>,
}

/// A material substance originating from a biological entity intended to be transplanted or infused into another (possibly the same) biological entity.
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

    /// External ids for this item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// organ | tissue | fluid | cells | biologicalAgent
    #[serde(rename = "productCategory")]
    pub product_category: Option<String>,

    /// What this biologically derived product is
    #[serde(rename = "productCode")]
    pub product_code: Option<CodeableConcept>,

    /// available | unavailable
    pub status: Option<String>,

    /// Procedure request
    pub request: Option<Vec<Box<Reference>>>,

    /// The amount of this biologically derived product
    pub quantity: Option<i32>,

    /// BiologicallyDerivedProduct parent
    pub parent: Option<Vec<Box<Reference>>>,

    /// How this product was collected
    pub collection: Option<BiologicallyDerivedProductCollection>,

    /// Any processing of the product during collection
    pub processing: Option<Vec<BiologicallyDerivedProductProcessing>>,

    /// Any manipulation of product post-collection
    pub manipulation: Option<BiologicallyDerivedProductManipulation>,

    /// Product storage
    pub storage: Option<Vec<BiologicallyDerivedProductStorage>>,
}
