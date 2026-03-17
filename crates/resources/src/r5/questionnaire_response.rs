//! FHIR R5 QuestionnaireResponse Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Groups and questions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific item from Questionnaire
    #[serde(rename = "linkId")]
    pub link_id: String,

    /// ElementDefinition - details for the item
    pub definition: Option<String>,

    /// Name for group or question text
    pub text: Option<String>,

    /// The response(s) to the question
    pub answer: Option<Vec<QuestionnaireResponseItemAnswer>>,

    /// Child items of group item
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}

/// The response(s) to the question
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponseItemAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Single-valued answer to the question
    pub value: serde_json::Value,

    /// Child items of question
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}

/// A structured set of questions and their answers. The questions are ordered and grouped into coherent subsets, corresponding to the structure of the grouping of the questionnaire being responded to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireResponse {
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

    /// Business identifier for this set of answers
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Request fulfilled by this QuestionnaireResponse
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// Canonical URL of Questionnaire being answered
    pub questionnaire: String,

    /// in-progress | completed | amended | entered-in-error | stopped
    pub status: String,

    /// The subject of the questions
    pub subject: Option<Box<Reference>>,

    /// Encounter the questionnaire response is part of
    pub encounter: Option<Box<Reference>>,

    /// Date the answers were gathered
    pub authored: Option<String>,

    /// The individual or device that received and recorded the answers
    pub author: Option<Box<Reference>>,

    /// The individual or device that answered the questions
    pub source: Option<Box<Reference>>,

    /// Groups and questions
    pub item: Option<Vec<QuestionnaireResponseItem>>,
}
