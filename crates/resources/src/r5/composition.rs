//! FHIR R5 Composition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
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
    pub mode: CodeableConcept,

    /// When the composition was attested
    pub time: Option<String>,

    /// Who attested the composition
    pub party: Option<Box<Reference>>,
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

    /// The period covered by the documentation
    pub period: Option<Period>,

    /// The event(s) being documented, as code(s), reference(s), or both
    pub detail: Option<Vec<CodeableReference>>,
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

    /// Canonical identifier for this Composition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Version-independent identifier for the Composition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// An explicitly assigned identifer of a variation of the content in the Composition
    pub version: Option<String>,

    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    pub status: String,

    /// Kind of composition (LOINC if possible)
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Categorization of Composition
    pub category: Option<Vec<CodeableConcept>>,

    /// Who and/or what the composition is about
    pub subject: Option<Vec<Box<Reference>>>,

    /// Context of the Composition
    pub encounter: Option<Box<Reference>>,

    /// Composition editing time
    pub date: String,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Who and/or what authored the composition
    pub author: Vec<Box<Reference>>,

    /// Name for this Composition (computer friendly)
    pub name: Option<String>,

    /// Human Readable name/title
    pub title: String,

    /// For any additional notes
    pub note: Option<Vec<Annotation>>,

    /// Attests to accuracy of composition
    pub attester: Option<Vec<CompositionAttester>>,

    /// Organization which maintains the composition
    pub custodian: Option<Box<Reference>>,

    /// Relationships to other compositions/documents
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<RelatedArtifact>>,

    /// The clinical service(s) being documented
    pub event: Option<Vec<CompositionEvent>>,

    /// Composition is broken into sections
    pub section: Option<Vec<CompositionSection>>,
}
