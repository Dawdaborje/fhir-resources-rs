//! FHIR R4B ResearchDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The ResearchDefinition resource describes the conditional state (population and any exposures being compared within the population) and outcome (if specified) that the knowledge (evidence, assertio...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchDefinition {
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

    /// Canonical identifier for this research definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the research definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the research definition
    pub version: Option<String>,

    /// Name for this research definition (computer friendly)
    pub name: Option<String>,

    /// Name for this research definition (human friendly)
    pub title: Option<String>,

    /// Title for use in informal contexts
    #[serde(rename = "shortTitle")]
    pub short_title: Option<String>,

    /// Subordinate title of the ResearchDefinition
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Box<Reference>>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the research definition
    pub description: Option<String>,

    /// Used for footnotes or explanatory notes
    pub comment: Option<Vec<String>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for research definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this research definition is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the ResearchDefinition
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the research definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the research definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the research definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// The category of the ResearchDefinition, such as Education, Treatment, Assessment, etc.
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations, etc.
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Logic used by the ResearchDefinition
    pub library: Option<Vec<String>>,

    /// What population?
    pub population: Box<Reference>,

    /// What exposure?
    pub exposure: Option<Box<Reference>>,

    /// What alternative exposure state?
    #[serde(rename = "exposureAlternative")]
    pub exposure_alternative: Option<Box<Reference>>,

    /// What outcome?
    pub outcome: Option<Box<Reference>>,
}
