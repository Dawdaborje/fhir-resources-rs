//! FHIR R4B ActivityDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Who should participate in the action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinitionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// patient | practitioner | related-person | device
    #[serde(rename = "type")]
    pub r#type: String,

    /// E.g. Nurse, Surgeon, Parent, etc.
    pub role: Option<CodeableConcept>,
}

/// Dynamic aspects of the definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinitionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The path to the element to be set dynamically
    pub path: String,

    /// An expression that provides the dynamic value for the customization
    pub expression: Expression,
}

/// This resource allows for the definition of some activity to be performed, independent of a particular patient, practitioner, or other performance context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityDefinition {
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

    /// Canonical identifier for this activity definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the activity definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the activity definition
    pub version: Option<String>,

    /// Name for this activity definition (computer friendly)
    pub name: Option<String>,

    /// Name for this activity definition (human friendly)
    pub title: Option<String>,

    /// Subordinate title of the activity definition
    pub subtitle: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Type of individual the activity definition is intended for
    #[serde(rename = "subjectCodeableConcept")]
    pub subject_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "subjectReference")]
    pub subject_reference: Option<Box<Reference>>,

    #[serde(rename = "subjectCanonical")]
    pub subject_canonical: Option<String>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the activity definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for activity definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this activity definition is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the activity definition
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the activity definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the activity definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the activity definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment, etc.
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

    /// Logic used by the activity definition
    pub library: Option<Vec<String>>,

    /// Kind of resource
    pub kind: Option<String>,

    /// What profile the resource needs to conform to
    pub profile: Option<String>,

    /// Detail type of activity
    pub code: Option<CodeableConcept>,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: Option<String>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if the activity should not be performed
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// When activity is to occur
    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,

    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<String>,

    #[serde(rename = "timingAge")]
    pub timing_age: Option<Age>,

    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,

    #[serde(rename = "timingRange")]
    pub timing_range: Option<Range>,

    #[serde(rename = "timingDuration")]
    pub timing_duration: Option<Duration>,

    /// Where it should happen
    pub location: Option<Box<Reference>>,

    /// Who should participate in the action
    pub participant: Option<Vec<ActivityDefinitionParticipant>>,

    /// What's administered/supplied
    #[serde(rename = "productReference")]
    pub product_reference: Option<Box<Reference>>,

    #[serde(rename = "productCodeableConcept")]
    pub product_codeable_concept: Option<CodeableConcept>,

    /// How much is administered/consumed/supplied
    pub quantity: Option<Quantity>,

    /// Detailed dosage instructions
    pub dosage: Option<Vec<Dosage>>,

    /// What part of body to perform on
    #[serde(rename = "bodySite")]
    pub body_site: Option<Vec<CodeableConcept>>,

    /// What specimens are required to perform this action
    #[serde(rename = "specimenRequirement")]
    pub specimen_requirement: Option<Vec<Box<Reference>>>,

    /// What observations are required to perform this action
    #[serde(rename = "observationRequirement")]
    pub observation_requirement: Option<Vec<Box<Reference>>>,

    /// What observations must be produced by this action
    #[serde(rename = "observationResultRequirement")]
    pub observation_result_requirement: Option<Vec<Box<Reference>>>,

    /// Transform to apply the template
    pub transform: Option<String>,

    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<ActivityDefinitionDynamicValue>>,
}
