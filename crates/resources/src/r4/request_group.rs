//! FHIR R4 RequestGroup Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Proposed actions, if any
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestGroupAction {
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

    /// Whether or not the action is applicable
    pub condition: Option<Vec<RequestGroupActionCondition>>,

    /// Relationship to another action
    #[serde(rename = "relatedAction")]
    pub related_action: Option<Vec<RequestGroupActionRelatedAction>>,

    /// When the action should take place
    pub timing: Option<serde_json::Value>,

    /// Who should perform the action
    pub participant: Option<Vec<Box<Reference>>>,

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

    /// Sub action
    pub action: Option<Vec<RequestGroupAction>>,
}

/// Whether or not the action is applicable
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestGroupActionCondition {
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
pub struct RequestGroupActionRelatedAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// What action this is related to
    #[serde(rename = "actionId")]
    pub action_id: String,

    /// before-start | before | before-end | concurrent-with-start | concurrent | concurrent-with-end | after-start | after | after-end
    pub relationship: String,

    /// Time offset for the relationship
    pub offset: Option<serde_json::Value>,
}

/// A group of related requests that can be used to capture intended activities that have inter-dependencies such as "give this medication after that one".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestGroup {
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

    /// Who the request group is about
    pub subject: Option<Box<Reference>>,

    /// Created as part of
    pub encounter: Option<Box<Reference>>,

    /// When the request group was authored
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Device or practitioner that authored the request group
    pub author: Option<Box<Reference>>,

    /// Why the request group is needed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why the request group is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Additional notes about the response
    pub note: Option<Vec<Annotation>>,

    /// Proposed actions, if any
    pub action: Option<Vec<RequestGroupAction>>,
}
