//! FHIR R4B Composition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Attests to accuracy of composition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionAttester {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// personal | professional | legal | official
    pub mode: String,

    /// When the composition was attested
    pub time: Option<String>,

    /// Who attested the composition
    pub party: Option<Box<Reference>>,
}

/// Relationships to other compositions/documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionRelatesTo {
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
    #[serde(rename = "targetIdentifier")]
    pub target_identifier: Box<Identifier>,

    #[serde(rename = "targetReference")]
    pub target_reference: Box<Reference>,
}

/// The clinical service(s) being documented
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionEvent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code(s) that apply to the event being documented
    pub code: Option<Vec<CodeableConcept>>,

    /// The period covered by the documentation
    pub period: Option<Period>,

    /// The event(s) being documented
    pub detail: Option<Vec<Box<Reference>>>,
}

/// Composition is broken into sections
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompositionSection {
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
    pub code: Option<CodeableConcept>,

    /// Who and/or what authored the section
    pub author: Option<Vec<Box<Reference>>>,

    /// Who/what the section is about, when it is not about the subject of composition
    pub focus: Option<Box<Reference>>,

    /// Text summary of the section, for human interpretation
    pub text: Option<Narrative>,

    /// working | snapshot | changes
    pub mode: Option<String>,

    /// Order of section entries
    #[serde(rename = "orderedBy")]
    pub ordered_by: Option<CodeableConcept>,

    /// A reference to data that supports this section
    pub entry: Option<Vec<Box<Reference>>>,

    /// Why the section is empty
    #[serde(rename = "emptyReason")]
    pub empty_reason: Option<CodeableConcept>,

    /// Nested Section
    pub section: Option<Vec<CompositionSection>>,
}

/// A set of healthcare-related information that is assembled together into a single logical package that provides a single coherent statement of meaning, establishes its own context and that has clini...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Composition {
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

    /// Version-independent identifier for the Composition
    pub identifier: Option<Box<Identifier>>,

    /// preliminary | final | amended | entered-in-error
    pub status: String,

    /// Kind of composition (LOINC if possible)
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Categorization of Composition
    pub category: Option<Vec<CodeableConcept>>,

    /// Who and/or what the composition is about
    pub subject: Option<Box<Reference>>,

    /// Context of the Composition
    pub encounter: Option<Box<Reference>>,

    /// Composition editing time
    pub date: String,

    /// Who and/or what authored the composition
    pub author: Vec<Box<Reference>>,

    /// Human Readable name/title
    pub title: String,

    /// As defined by affinity domain
    pub confidentiality: Option<String>,

    /// Attests to accuracy of composition
    pub attester: Option<Vec<CompositionAttester>>,

    /// Organization which maintains the composition
    pub custodian: Option<Box<Reference>>,

    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<CompositionRelatesTo>>,

    /// The clinical service(s) being documented
    pub event: Option<Vec<CompositionEvent>>,

    /// Composition is broken into sections
    pub section: Option<Vec<CompositionSection>>,
}
