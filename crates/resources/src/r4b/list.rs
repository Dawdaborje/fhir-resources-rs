//! FHIR R4B List Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Entries in the list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListEntry {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Status/Workflow information about this item
    pub flag: Option<CodeableConcept>,

    /// If this item is actually marked as deleted
    pub deleted: Option<bool>,

    /// When item added to list
    pub date: Option<String>,

    /// Actual entry
    pub item: Box<Reference>,
}

/// A list is a curated collection of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct List {
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

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// current | retired | entered-in-error
    pub status: String,

    /// working | snapshot | changes
    pub mode: String,

    /// Descriptive name for the list
    pub title: Option<String>,

    /// What the purpose of this list is
    pub code: Option<CodeableConcept>,

    /// If all resources have the same subject
    pub subject: Option<Box<Reference>>,

    /// Context in which list created
    pub encounter: Option<Box<Reference>>,

    /// When the list was prepared
    pub date: Option<String>,

    /// Who and/or what defined the list contents (aka Author)
    pub source: Option<Box<Reference>>,

    /// What order the list has
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,

    /// Comments about the list
    pub note: Option<Vec<Annotation>>,

    /// Entries in the list
    pub entry: Option<Vec<ListEntry>>,

    /// Why list is empty
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,
}
