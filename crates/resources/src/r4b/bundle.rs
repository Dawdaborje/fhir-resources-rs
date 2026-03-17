//! FHIR R4B Bundle Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Links related to this Bundle
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleLink {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// See http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
    pub relation: String,

    /// Reference details for the link
    pub url: String,
}

/// Entry in the bundle - will have a resource or information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntry {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Links related to this entry
    pub link: Option<Vec<BundleLink>>,

    /// URI for resource (Absolute URL server address or URI for UUID/OID)
    #[serde(rename = "fullUrl")]
    pub full_url: Option<String>,

    /// A resource in the bundle
    pub resource: Option<serde_json::Value>,

    /// Search related information
    pub search: Option<BundleEntrySearch>,

    /// Additional execution information (transaction/batch/history)
    pub request: Option<BundleEntryRequest>,

    /// Results of execution (transaction/batch/history)
    pub response: Option<BundleEntryResponse>,
}

/// Search related information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntrySearch {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// match | include | outcome - why this is in the result set
    pub mode: Option<String>,

    /// Search ranking (between 0 and 1)
    pub score: Option<f64>,
}

/// Additional execution information (transaction/batch/history)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntryRequest {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// GET | HEAD | POST | PUT | DELETE | PATCH
    pub method: String,

    /// URL for HTTP equivalent of this entry
    pub url: String,

    /// For managing cache currency
    #[serde(rename = "ifNoneMatch")]
    pub if_none_match: Option<String>,

    /// For managing cache currency
    #[serde(rename = "ifModifiedSince")]
    pub if_modified_since: Option<String>,

    /// For managing update contention
    #[serde(rename = "ifMatch")]
    pub if_match: Option<String>,

    /// For conditional creates
    #[serde(rename = "ifNoneExist")]
    pub if_none_exist: Option<String>,
}

/// Results of execution (transaction/batch/history)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BundleEntryResponse {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Status response code (text optional)
    pub status: String,

    /// The location (if the operation returns a location)
    pub location: Option<String>,

    /// The Etag for the resource (if relevant)
    pub etag: Option<String>,

    /// Server's date time modified
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,

    /// OperationOutcome with hints and warnings (for batch/transaction)
    pub outcome: Option<serde_json::Value>,
}

/// A container for a collection of resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bundle {
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

    /// Persistent identifier for the bundle
    pub identifier: Option<Box<Identifier>>,

    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection
    #[serde(rename = "type")]
    pub r#type: String,

    /// When the bundle was assembled
    pub timestamp: Option<String>,

    /// If search, the total number of matches
    pub total: Option<u32>,

    /// Links related to this Bundle
    pub link: Option<Vec<BundleLink>>,

    /// Entry in the bundle - will have a resource or information
    pub entry: Option<Vec<BundleEntry>>,

    /// Digital Signature
    pub signature: Option<Signature>,
}
