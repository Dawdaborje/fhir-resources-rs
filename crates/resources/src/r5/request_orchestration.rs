//! FHIR R5 RequestOrchestration Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Proposed actions, if any
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Pointer to specific item from the PlanDefinition
    #[serde(rename = "linkId")]
    pub link_id: Option<String>,

    /// User-visible prefix for the action (e.g. 1. or A.)
    pub prefix: Option<String>,

    /// User-visible title
    pub title: Option<String>,

    /// Short description of the action
    pub description: Option<String>,

    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    #[serde(rename = "textEquivalent")]
    pub text_equivalent: Option<String>,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Code representing the meaning of the action or sub-actions
    pub code: Option<Vec<CodeableConcept>>,

    /// Supporting documentation for the intended performer of the action
    pub documentation: Option<Vec<RelatedArtifact>>,

    /// What goals
    pub goal: Option<Vec<Box<Reference>>>,

    /// Whether or not the action is applicable
    pub condition: Option<Vec<RequestOrchestrationActionCondition>>,

    /// Input data requirements
    pub input: Option<Vec<RequestOrchestrationActionInput>>,

    /// Output data definition
    pub output: Option<Vec<RequestOrchestrationActionOutput>>,

    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<RequestOrchestrationActionRelatedAction>>,

    /// When the action should take place
    #[serde(rename = "timingDateTime")]
    pub timing_date_time: Option<String>,

    #[serde(rename = "timingAge")]
    pub timing_age: Option<Age>,

    #[serde(rename = "timingPeriod")]
    pub timing_period: Option<Period>,

    #[serde(rename = "timingDuration")]
    pub timing_duration: Option<Duration>,

    #[serde(rename = "timingRange")]
    pub timing_range: Option<Range>,

    #[serde(rename = "timingTiming")]
    pub timing_timing: Option<Timing>,

    /// Where it should happen
    pub location: Option<CodeableReference>,

    /// Who should perform the action
    pub participant: Option<Vec<RequestOrchestrationActionParticipant>>,

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

    /// The target of the action
    pub resource: Option<Box<Reference>>,

    /// Description of the activity to be performed
    #[serde(rename = "definitionCanonical")]
    pub definition_canonical: Option<String>,

    #[serde(rename = "definitionUri")]
    pub definition_uri: Option<String>,

    /// Transform to apply the template
    pub transform: Option<String>,

    /// Dynamic aspects of the definition
    #[serde(rename = "dynamicValue")]
    pub dynamic_value: Option<Vec<RequestOrchestrationActionDynamicValue>>,

    /// Sub action
    pub action: Option<Vec<RequestOrchestrationAction>>,
}

/// Whether or not the action is applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionCondition {
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

/// Input data requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionInput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// User-visible title
    pub title: Option<String>,

    /// What data is provided
    pub requirement: Option<DataRequirement>,

    /// What data is provided
    #[serde(rename = "relatedData")]
    pub related_data: Option<String>,
}

/// Output data definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionOutput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// User-visible title
    pub title: Option<String>,

    /// What data is provided
    pub requirement: Option<DataRequirement>,

    /// What data is provided
    #[serde(rename = "relatedData")]
    pub related_data: Option<String>,
}

/// Relationship to another action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What action this is related to
    #[serde(rename = "targetId")]
    pub target_id: String,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    pub relationship: String,

    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[serde(rename = "endRelationship")]
    pub end_relationship: Option<String>,

    /// Time offset for the relationship
    #[serde(rename = "offsetDuration")]
    pub offset_duration: Option<Duration>,

    #[serde(rename = "offsetRange")]
    pub offset_range: Option<Range>,
}

/// Who should perform the action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// Who or what can participate
    #[serde(rename = "typeCanonical")]
    pub type_canonical: Option<String>,

    /// Who or what can participate
    #[serde(rename = "typeReference")]
    pub type_reference: Option<Box<Reference>>,

    /// E.g. Nurse, Surgeon, Parent, etc
    pub role: Option<CodeableConcept>,

    /// E.g. Author, Reviewer, Witness, etc
    pub function: Option<CodeableConcept>,

    /// Who/what is participating?
    #[serde(rename = "actorCanonical")]
    pub actor_canonical: Option<String>,

    #[serde(rename = "actorReference")]
    pub actor_reference: Option<Box<Reference>>,
}

/// Dynamic aspects of the definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestrationActionDynamicValue {
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

/// A set of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestOrchestration {
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

    /// Business identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Instantiates FHIR protocol or definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Vec<String>>,

    /// Instantiates external protocol or definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<Vec<String>>,

    /// Fulfills plan, proposal, or order
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Request(s) replaced by this request
    pub replaces: Option<Vec<Box<Reference>>>,

    /// Composite request this is part of
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    pub status: String,

    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// What's being requested/ordered
    pub code: Option<CodeableConcept>,

    /// Who the request orchestration is about
    pub subject: Option<Box<Reference>>,

    /// Created as part of
    pub encounter: Option<Box<Reference>>,

    /// When the request orchestration was authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Device or practitioner that authored the request orchestration
    pub author: Option<Box<Reference>>,

    /// Why the request orchestration is needed
    pub reason: Option<Vec<CodeableReference>>,

    /// What goals
    pub goal: Option<Vec<Box<Reference>>>,

    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,

    /// Proposed actions, if any
    pub action: Option<Vec<RequestOrchestrationAction>>,
}
