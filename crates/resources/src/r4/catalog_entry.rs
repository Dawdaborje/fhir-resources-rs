//! FHIR R4 CatalogEntry Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// An item that this catalog entry is related to
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntryRelatedEntry {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// triggers | is-replaced-by
    pub relationtype: String,

    /// The reference to the related item
    pub item: Box<Reference>,
}

/// Catalog entries are wrappers that contextualize items included in a catalog.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogEntry {
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

    /// Unique identifier of the catalog item
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The type of item - medication, device, service, protocol or other
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Whether the entry represents an orderable item
    pub orderable: bool,

    /// The item that is being defined
    #[serde(rename = "referencedItem")]
    pub referenced_item: Box<Reference>,

    /// Any additional identifier(s) for the catalog item, in the same granularity or concept
    #[serde(rename = "additionalIdentifier")]
    pub additional_identifier: Option<Vec<Box<Identifier>>>,

    /// Classification (category or class) of the item entry
    pub classification: Option<Vec<CodeableConcept>>,

    /// draft | active | retired | unknown
    pub status: Option<String>,

    /// The time period in which this catalog entry is expected to be active
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,

    /// The date until which this catalog entry is expected to be active
    #[serde(rename = "validTo")]
    pub valid_to: Option<String>,

    /// When was this catalog last updated
    #[serde(rename = "lastUpdated")]
    pub last_updated: Option<String>,

    /// Additional characteristics of the catalog entry
    #[serde(rename = "additionalCharacteristic")]
    pub additional_characteristic: Option<Vec<CodeableConcept>>,

    /// Additional classification of the catalog entry
    #[serde(rename = "additionalClassification")]
    pub additional_classification: Option<Vec<CodeableConcept>>,

    /// An item that this catalog entry is related to
    #[serde(rename = "relatedEntry")]
    pub related_entry: Option<Vec<CatalogEntryRelatedEntry>>,
}
