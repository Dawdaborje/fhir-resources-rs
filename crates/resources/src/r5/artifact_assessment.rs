//! FHIR R5 ArtifactAssessment Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Comment, classifier, or rating content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactAssessmentContent {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// comment | classifier | rating | container | response | change-request
    #[serde(rename = "informationType")]
    pub information_type: Option<String>,

    /// Brief summary of the content
    pub summary: Option<String>,

    /// What type of content
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Rating, classifier, or assessment
    pub classifier: Option<Vec<CodeableConcept>>,

    /// Quantitative rating
    pub quantity: Option<Quantity>,

    /// Who authored the content
    pub author: Option<Box<Reference>>,

    /// What the comment is directed to
    pub path: Option<Vec<String>>,

    /// Additional information
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Acceptable to publicly share the resource content
    #[serde(rename = "freeToShare")]
    pub free_to_share: Option<bool>,

    /// Contained content
    pub component: Option<Vec<ArtifactAssessmentContent>>,
}

/// This Resource provides one or more comments, classifiers or ratings about a Resource and supports attribution and rights management metadata for the added content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactAssessment {
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

    /// Additional identifier for the artifact assessment
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// A short title for the assessment for use in displaying and selecting
    pub title: Option<String>,

    /// How to cite the comment or rating
    #[serde(rename = "citeAsReference")]
    pub cite_as_reference: Option<Box<Reference>>,

    #[serde(rename = "citeAsMarkdown")]
    pub cite_as_markdown: Option<String>,

    /// Date last changed
    pub date: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the artifact assessment was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the artifact assessment was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// The artifact assessed, commented upon or rated
    #[serde(rename = "artifactReference")]
    pub artifact_reference: Box<Reference>,

    #[serde(rename = "artifactCanonical")]
    pub artifact_canonical: String,

    #[serde(rename = "artifactUri")]
    pub artifact_uri: String,

    /// Comment, classifier, or rating content
    pub content: Option<Vec<ArtifactAssessmentContent>>,

    /// submitted | triaged | waiting-for-input | resolved-no-change | resolved-change-required | deferred | duplicate | applied | published | entered-in-error
    #[serde(rename = "workflowStatus")]
    pub workflow_status: Option<String>,

    /// unresolved | not-persuasive | persuasive | persuasive-with-modification | not-persuasive-with-modification
    pub disposition: Option<String>,
}
