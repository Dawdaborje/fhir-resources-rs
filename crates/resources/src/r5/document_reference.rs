//! FHIR R5 DocumentReference Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Attests to accuracy of the document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceAttester {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// personal | professional | legal | official
    pub mode: CodeableConcept,

    /// When the document was attested
    pub time: Option<String>,

    /// Who attested the document
    pub party: Option<Box<Reference>>,
}

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

    /// The relationship type with another document
    pub code: CodeableConcept,

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

    /// Content profile rules for the document
    pub profile: Option<Vec<DocumentReferenceContentProfile>>,
}

/// Content profile rules for the document
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentReferenceContentProfile {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code|uri|canonical
    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,

    #[serde(rename = "valueUri")]
    pub value_uri: String,

    #[serde(rename = "valueCanonical")]
    pub value_canonical: String,
}

/// A reference to a document of any kind for any purpose. While the term “document” implies a more narrow focus, for this resource this “document” encompasses *any* serialized object with a mi...
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

    /// Business identifiers for the document
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// An explicitly assigned identifer of a variation of the content in the DocumentReference
    pub version: Option<String>,

    /// Procedure that caused this media to be created
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// current | superseded | entered-in-error
    pub status: String,

    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    #[serde(rename = "docStatus")]
    pub doc_status: Option<String>,

    /// Imaging modality used
    pub modality: Option<Vec<CodeableConcept>>,

    /// Kind of document (LOINC if possible)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Categorization of document
    pub category: Option<Vec<CodeableConcept>>,

    /// Who/what is the subject of the document
    pub subject: Option<Box<Reference>>,

    /// Context of the document content
    pub context: Option<Vec<Box<Reference>>>,

    /// Main clinical acts documented
    pub event: Option<Vec<CodeableReference>>,

    /// Body part included
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableReference>>,

    /// Kind of facility where patient was seen
    #[serde(rename = "facilityType")]
    pub facility_type: Option<CodeableConcept>,

    /// Additional details about where the content was created (e.g. clinical specialty)
    #[serde(rename = "practiceSetting")]
    pub practice_setting: Option<CodeableConcept>,

    /// Time of service that is being documented
    pub period: Option<Period>,

    /// When this document reference was created
    pub date: Option<String>,

    /// Who and/or what authored the document
    pub author: Option<Vec<Box<Reference>>>,

    /// Attests to accuracy of the document
    pub attester: Option<Vec<DocumentReferenceAttester>>,

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
}
