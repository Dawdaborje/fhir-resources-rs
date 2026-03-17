//! FHIR R4B EvidenceReport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Focus of the report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubject {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Characteristic
    pub characteristic: Option<Vec<EvidenceReportSubjectCharacteristic>>,

    /// Footnotes and/or explanatory notes
    pub note: Option<Vec<Annotation>>,
}

/// Characteristic
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSubjectCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Characteristic code
    pub code: CodeableConcept,

    /// Characteristic value
    pub value: serde_json::Value,

    /// Is used to express not the characteristic
    pub exclude: Option<bool>,

    /// Timeframe for the characteristic
    pub period: Option<Period>,
}

/// Relationships to other compositions/documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// replaces | amends | appends | transforms | replacedWith | amendedWith | appendedWith | transformedWith
    pub code: String,

    /// Target of the relationship
    pub target: serde_json::Value,
}

/// Composition is broken into sections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReportSection {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for section (e.g. for ToC)
    pub title: Option<String>,

    /// Classification of section (recommended)
    pub focus: Option<CodeableConcept>,

    /// Classification of section by Resource
    #[serde(rename = "focusReference")]
    pub focus_reference: Option<Box<Reference>>,

    /// Who and/or what authored the section
    pub author: Option<Vec<Box<Reference>>>,

    /// Text summary of the section, for human interpretation
    pub text: Option<Narrative>,

    /// working | snapshot | changes
    pub mode: Option<String>,

    /// Order of section entries
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,

    /// Extensible classifiers as content
    #[serde(rename = "entryClassifier")]
    pub entry_classifier: Option<Vec<CodeableConcept>>,

    /// Reference to resources as content
    #[serde(rename = "entryReference")]
    pub entry_reference: Option<Vec<Box<Reference>>>,

    /// Quantity as content
    #[serde(rename = "entryQuantity")]
    pub entry_quantity: Option<Vec<Quantity>>,

    /// Why the section is empty
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,

    /// Nested Section
    pub section: Option<Vec<EvidenceReportSection>>,
}

/// The EvidenceReport Resource is a specialized container for a collection of resources and codable concepts, adapted to support compositions of Evidence, EvidenceVariable, and Citation resources and ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvidenceReport {
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

    /// Canonical identifier for this EvidenceReport, represented as a globally unique URI
    pub url: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Unique identifier for the evidence report
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Identifiers for articles that may relate to more than one evidence report
    #[serde(rename = "relatedIdentifier")]
    pub related_identifier: Option<Vec<Box<Identifier>>>,

    /// Citation for this report
    #[serde(rename = "citeAs")]
    pub cite_as: Option<serde_json::Value>,

    /// Kind of report
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Used for footnotes and annotations
    pub note: Option<Vec<Annotation>>,

    /// Link, description or reference to artifact associated with the report
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Focus of the report
    pub subject: EvidenceReportSubject,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<EvidenceReportRelatesTo>>,

    /// Composition is broken into sections
    pub section: Option<Vec<EvidenceReportSection>>,
}
