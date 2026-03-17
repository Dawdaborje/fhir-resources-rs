//! FHIR R4 DocumentManifest Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Related things
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentManifestRelated {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifiers of things that are related
    pub identifier: Option<Box<Identifier>>,

    /// Related Resource
    #[serde(rename = "ref")]
    pub r#ref: Option<Box<Reference>>,
}

/// A collection of documents compiled for a purpose together with metadata that applies to the collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentManifest {
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

    /// Unique Identifier for the set of documents
    #[serde(rename = "masterIdentifier")]
    pub master_identifier: Option<Box<Identifier>>,

    /// Other identifiers for the manifest
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// current | superseded | entered-in-error
    pub status: String,

    /// Kind of document set
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The subject of the set of documents
    pub subject: Option<Box<Reference>>,

    /// When this document manifest created
    pub created: Option<String>,

    /// Who and/or what authored the DocumentManifest
    pub author: Option<Vec<Box<Reference>>>,

    /// Intended to get notified about this set of documents
    pub recipient: Option<Vec<Box<Reference>>>,

    /// The source system/application/software
    pub source: Option<String>,

    /// Human-readable description (title)
    pub description: Option<String>,

    /// Items in manifest
    pub content: Vec<Box<Reference>>,

    /// Related things
    pub related: Option<Vec<DocumentManifestRelated>>,
}
