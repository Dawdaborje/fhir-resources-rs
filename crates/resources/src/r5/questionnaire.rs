//! FHIR R5 Questionnaire Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Questions and sections within the Questionnaire
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Unique id for item in questionnaire
    #[serde(rename = "linkId")]
    pub link_id: String,

    /// ElementDefinition - details for the item
    pub definition: Option<String>,

    /// Corresponding concept for this item in a terminology
    pub code: Option<Vec<Coding>>,

    /// E.g. "1(a)", "2.5.3"
    pub prefix: Option<String>,

    /// Primary text for the item
    pub text: Option<String>,

    /// group | display | boolean | decimal | integer | date | dateTime +
    #[serde(rename = "type")]
    pub r#type: String,

    /// Only allow data when
    #[serde(rename = "enableWhen")]
    pub enable_when: Option<Vec<QuestionnaireItemEnableWhen>>,

    /// all | any
    #[serde(rename = "enableBehavior")]
    pub enable_behavior: Option<String>,

    /// hidden | protected
    #[serde(rename = "disabledDisplay")]
    pub disabled_display: Option<String>,

    /// Whether the item must be included in data results
    pub required: Option<bool>,

    /// Whether the item may repeat
    pub repeats: Option<bool>,

    /// Don't allow human editing
    #[serde(rename = "readOnly")]
    pub read_only: Option<bool>,

    /// No more than these many characters
    #[serde(rename = "maxLength")]
    pub max_length: Option<i32>,

    /// optionsOnly | optionsOrType | optionsOrString
    #[serde(rename = "answerConstraint")]
    pub answer_constraint: Option<String>,

    /// ValueSet containing permitted answers
    #[serde(rename = "answerValueSet")]
    pub answer_value_set: Option<String>,

    /// Permitted answer
    #[serde(rename = "answerOption")]
    pub answer_option: Option<Vec<QuestionnaireItemAnswerOption>>,

    /// Initial value(s) when item is first rendered
    pub initial: Option<Vec<QuestionnaireItemInitial>>,

    /// Nested questionnaire items
    pub item: Option<Vec<QuestionnaireItem>>,
}

/// Only allow data when
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemEnableWhen {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The linkId of question that determines whether item is enabled/disabled
    pub question: String,

    /// exists | = | != | > | < | >= | <=
    pub operator: String,

    /// Value for question comparison based on operator
    pub answer: serde_json::Value,
}

/// Permitted answer
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemAnswerOption {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Answer value
    pub value: serde_json::Value,

    /// Whether option is selected by default
    #[serde(rename = "initialSelected")]
    pub initial_selected: Option<bool>,
}

/// Initial value(s) when item is first rendered
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionnaireItemInitial {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Actual value for initializing the question
    pub value: serde_json::Value,
}

/// A structured set of questions intended to guide the collection of answers from end-users. Questionnaires provide detailed control over order, presentation, phraseology and grouping to allow coheren...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Questionnaire {
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

    /// Canonical identifier for this questionnaire, represented as an absolute URI (globally unique)
    pub url: Option<String>,

    /// Business identifier for questionnaire
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the questionnaire
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this questionnaire (computer friendly)
    pub name: Option<String>,

    /// Name for this questionnaire (human friendly)
    pub title: Option<String>,

    /// Based on Questionnaire
    #[serde(rename = "derivedFrom")]
    pub derived_from: Option<Vec<String>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Resource that can be subject of QuestionnaireResponse
    #[serde(rename = "subjectType")]
    pub subject_type: Option<Vec<String>>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the questionnaire
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for questionnaire (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this questionnaire is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// When the questionnaire was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the questionnaire was last reviewed by the publisher
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the questionnaire is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Concept that represents the overall questionnaire
    pub code: Option<Vec<Coding>>,

    /// Questions and sections within the Questionnaire
    pub item: Option<Vec<QuestionnaireItem>>,
}
