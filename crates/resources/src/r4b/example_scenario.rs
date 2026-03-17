//! FHIR R4B ExampleScenario Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Actor participating in the resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioActor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// ID or acronym of the actor
    #[serde(rename = "actorId")]
    pub actor_id: String,

    /// person | entity
    #[serde(rename = "type")]
    pub r#type: String,

    /// The name of the actor as shown in the page
    pub name: Option<String>,

    /// The description of the actor
    pub description: Option<String>,
}

/// Each resource and each version that is present in the workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The id of the resource for referencing
    #[serde(rename = "resourceId")]
    pub resource_id: String,

    /// The type of the resource
    #[serde(rename = "resourceType")]
    pub resource_type: String,

    /// A short name for the resource instance
    pub name: Option<String>,

    /// Human-friendly description of the resource instance
    pub description: Option<String>,

    /// A specific version of the resource
    pub version: Option<Vec<ExampleScenarioInstanceVersion>>,

    /// Resources contained in the instance
    #[serde(rename = "containedInstance")]
    pub contained_instance: Option<Vec<ExampleScenarioInstanceContainedInstance>>,
}

/// A specific version of the resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstanceVersion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The identifier of a specific version of a resource
    #[serde(rename = "versionId")]
    pub version_id: String,

    /// The description of the resource version
    pub description: String,
}

/// Resources contained in the instance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioInstanceContainedInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Each resource contained in the instance
    #[serde(rename = "resourceId")]
    pub resource_id: String,

    /// A specific version of a resource contained in the instance
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
}

/// Each major process - a group of operations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcess {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The diagram title of the group of operations
    pub title: String,

    /// A longer description of the group of operations
    pub description: Option<String>,

    /// Description of initial status before the process starts
    #[serde(rename = "preConditions")]
    pub pre_conditions: Option<String>,

    /// Description of final status after the process ends
    #[serde(rename = "postConditions")]
    pub post_conditions: Option<String>,

    /// Each step of the process
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

/// Each step of the process
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStep {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Nested process
    pub process: Option<Vec<ExampleScenarioProcess>>,

    /// If there is a pause in the flow
    pub pause: Option<bool>,

    /// Each interaction or action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    pub alternative: Option<Vec<ExampleScenarioProcessStepAlternative>>,
}

/// Each interaction or action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStepOperation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The sequential number of the interaction
    pub number: String,

    /// The type of operation - CRUD
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// The human-friendly name of the interaction
    pub name: Option<String>,

    /// Who starts the transaction
    pub initiator: Option<String>,

    /// Who receives the transaction
    pub receiver: Option<String>,

    /// A comment to be inserted in the diagram
    pub description: Option<String>,

    /// Whether the initiator is deactivated right after the transaction
    #[serde(rename = "initiatorActive")]
    pub initiator_active: Option<bool>,

    /// Whether the receiver is deactivated right after the transaction
    #[serde(rename = "receiverActive")]
    pub receiver_active: Option<bool>,

    /// Each resource instance used by the initiator
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Each resource instance used by the responder
    pub response: Option<ExampleScenarioInstanceContainedInstance>,
}

/// Alternate non-typical step action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenarioProcessStepAlternative {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for alternative
    pub title: String,

    /// A human-readable description of each option
    pub description: Option<String>,

    /// What happens in each alternative option
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

/// Example of workflow instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExampleScenario {
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

    /// Canonical identifier for this example scenario, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Additional identifier for the example scenario
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the example scenario
    pub version: Option<String>,

    /// Name for this example scenario (computer friendly)
    pub name: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for example scenario (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<String>,

    /// Actor participating in the resource
    pub actor: Option<Vec<ExampleScenarioActor>>,

    /// Each resource and each version that is present in the workflow
    pub instance: Option<Vec<ExampleScenarioInstance>>,

    /// Each major process - a group of operations
    pub process: Option<Vec<ExampleScenarioProcess>>,

    /// Another nested workflow
    pub workflow: Option<Vec<String>>,
}
