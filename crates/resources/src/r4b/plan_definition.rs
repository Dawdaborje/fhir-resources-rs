//! FHIR R4B PlanDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// What the plan is trying to accomplish
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionGoal {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// E.g. Treatment, dietary, behavioral
    pub category: Option<CodeableConcept>,

    /// Code or text describing the goal
    pub description: CodeableConcept,

    /// high-priority | medium-priority | low-priority
    pub priority: Option<CodeableConcept>,

    /// When goal pursuit begins
    pub start: Option<CodeableConcept>,

    /// What does the goal address
    pub addresses: Option<Vec<CodeableConcept>>,

    /// Supporting documentation for the goal
    pub documentation: Option<Vec<RelatedArtifact>>,

    /// Target outcome for the goal
    pub target: Option<Vec<PlanDefinitionGoalTarget>>,
}

/// Target outcome for the goal
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionGoalTarget {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The parameter whose value is to be tracked
    pub measure: Option<CodeableConcept>,

    /// The target value to be achieved
    pub detail: Option<serde_json::Value>,

    /// Reach goal within
    pub due: Option<Duration>,
}

/// Action defined by the plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<String>,

    /// User-visible title
    pub title: Option<String>,

    /// Brief description of the action
    pub description: Option<String>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    #[serde(rename = "textEquivalent")]
    pub text_equivalent: Option<String>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<Vec<CodeableConcept>>,

    /// Why the action should be performed
    pub reason: Option<Vec<CodeableConcept>>,

    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<RelatedArtifact>>,

    /// What goals this action supports
    #[serde(rename = "goalId")]
    pub goal_id: Option<Vec<String>>,

    /// Type of individual the action is focused on
    pub subject: Option<serde_json::Value>,

    /// When the action should be triggered
    pub trigger: Option<Vec<TriggerDefinition>>,

    /// Whether or not the action is applicable
    pub condition: Option<Vec<PlanDefinitionActionCondition>>,

    /// Input data requirements
    pub input: Option<Vec<DataRequirement>>,

    /// Output data definition
    pub output: Option<Vec<DataRequirement>>,

    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<PlanDefinitionActionRelatedAction>>,

    /// When the action should take place
    pub timing: Option<serde_json::Value>,

    /// Who should participate in the action
    pub participant: Option<Vec<PlanDefinitionActionParticipant>>,

    /// create | update | remove | fire-event
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// visual-group | logical-group | sentence-group
    #[serde(rename = "groupingBehavior")]
    pub grouping_behavior: Option<String>,

    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    #[serde(rename = "selectionBehavior")]
    pub selection_behavior: Option<String>,

    /// must | could | must-unless-documented
    #[serde(rename = "requiredBehavior")]
    pub required_behavior: Option<String>,

    /// yes | no
    #[serde(rename = "precheckBehavior")]
    pub precheck_behavior: Option<String>,

    /// single | multiple
    #[serde(rename = "cardinalityBehavior")]
    pub cardinality_behavior: Option<String>,

    /// Description of the activity to be performed
    pub definition: Option<serde_json::Value>,

    /// Transform to apply the template
    pub transform: Option<String>,

    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<PlanDefinitionActionDynamicValue>>,

    /// A sub-action
    pub action: Option<Vec<PlanDefinitionAction>>,
}

/// Whether or not the action is applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionCondition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// applicability | start | stop
    pub kind: String,

    /// Boolean-valued expression
    pub expression: Option<Expression>,
}

/// Relationship to another action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What action is this related to
    #[serde(rename = "actionId")]
    pub action_id: String,

    /// before-start | before | before-end | concurrent-with-start | concurrent | concurrent-with-end | after-start | after | after-end
    pub relationship: String,

    /// Time offset for the relationship
    pub offset: Option<serde_json::Value>,
}

/// Who should participate in the action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionParticipant {
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

    /// E.g. Nurse, Surgeon, Parent
    pub role: Option<CodeableConcept>,
}

/// Dynamic aspects of the definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinitionActionDynamicValue {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The path to the element to be set dynamically
    pub path: Option<String>,

    /// An expression that provides the dynamic value for the customization
    pub expression: Option<Expression>,
}

/// This resource allows for the definition of various types of plans as a sharable, consumable, and executable artifact. The resource is general enough to support the description of a broad range of c...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanDefinition {
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

    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the plan definition
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the plan definition
    pub version: Option<String>,

    /// Name for this plan definition (computer friendly)
    pub name: Option<String>,

    /// Name for this plan definition (human friendly)
    pub title: Option<String>,

    /// Subordinate title of the plan definition
    pub subtitle: Option<String>,

    /// order-set | clinical-protocol | eca-rule | workflow-definition
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Type of individual the plan definition is focused on
    pub subject: Option<serde_json::Value>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the plan definition
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for plan definition (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this plan definition is defined
    pub purpose: Option<String>,

    /// Describes the clinical usage of the plan
    pub usage: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// When the plan definition was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the plan definition was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the plan definition is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// E.g. Education, Treatment, Assessment
    pub topic: Option<Vec<CodeableConcept>>,

    /// Who authored the content
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the content
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the content
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the content
    pub endorser: Option<Vec<ContactDetail>>,

    /// Additional documentation, citations
    #[serde(rename = "relatedArtifact")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,

    /// Logic used by the plan definition
    pub library: Option<Vec<String>>,

    /// What the plan is trying to accomplish
    pub goal: Option<Vec<PlanDefinitionGoal>>,

    /// Action defined by the plan
    pub action: Option<Vec<PlanDefinitionAction>>,
}
