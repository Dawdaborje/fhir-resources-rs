//! FHIR R4 DocumentReference Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Relationships to other documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// replaces | transforms | signs | appends
    pub code: String,

    /// Target of the relationship
    pub target: Box<Reference>,
}

/// Document referenced
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Where to access the document
    pub attachment: Attachment,

    /// Format/content rules for the document
    pub format: Option<Coding>,
}

/// Clinical context of document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContext {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Context of the document content
    pub encounter: Option<Vec<Box<Reference>>>,

    /// Main clinical acts documented
    pub event: Option<Vec<CodeableConcept>>,

    /// Time of service that is being documented
    pub period: Option<Period>,

    /// Kind of facility where patient was seen
    #[serde(rename = "facilityType")]
    pub facility_type: Option<CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical specialty)
    #[serde(rename = "practiceSetting")]
    pub practice_setting: Option<CodeableConcept>,

    /// Patient demographics from source
    #[serde(rename = "sourcePatientInfo")]
    pub source_patient_info: Option<Box<Reference>>,

    /// Related identifiers or resources
    pub related: Option<Vec<Box<Reference>>>,
}

/// A reference to a document of any kind for any purpose. Provides metadata about the document so that the document can be discovered and managed. The scope of a document is any seralized object with ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReference {
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

    /// Master Version Specific Identifier
    #[serde(rename = "masterIdentifier")]
    pub master_identifier: Option<Box<Identifier>>,

    /// Other identifiers for the document
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// current | superseded | entered-in-error
    pub status: String,

    /// preliminary | final | amended | entered-in-error
    #[serde(rename = "docStatus")]
    pub doc_status: Option<String>,

    /// Kind of document (LOINC if possible)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Categorization of document
    pub category: Option<Vec<CodeableConcept>>,

    /// Who/what is the subject of the document
    pub subject: Option<Box<Reference>>,

    /// When this document reference was created
    pub date: Option<String>,

    /// Who and/or what authored the document
    pub author: Option<Vec<Box<Reference>>>,

    /// Who/what authenticated the document
    pub authenticator: Option<Box<Reference>>,

    /// Organization which maintains the document
    pub custodian: Option<Box<Reference>>,

    /// Relationships to other documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<DocumentReferenceRelatesTo>>,

    /// Human-readable description
    pub description: Option<String>,

    /// Document security-tags
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<CodeableConcept>>,

    /// Document referenced
    pub content: Vec<DocumentReferenceContent>,

    /// Clinical context of document
    pub context: Option<DocumentReferenceContext>,
}
