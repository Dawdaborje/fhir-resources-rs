//! FHIR R5 ExampleScenario Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Individual involved in exchange
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
    pub key: String,

    /// person | system
    #[serde(rename = "type")]
    pub r#type: String,

    /// Label for actor when rendering
    pub title: String,

    /// Details about actor
    pub description: Option<String>,
}

/// Data used in the scenario
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

    /// ID or acronym of the instance
    pub key: String,

    /// Data structure for example
    #[serde(rename = "structureType")]
    pub structure_type: Coding,

    /// E.g. 4.0.1
    #[serde(rename = "structureVersion")]
    pub structure_version: Option<String>,

    /// Rules instance adheres to
    #[serde(rename = "structureProfileCanonical")]
    pub structure_profile_canonical: Option<String>,

    #[serde(rename = "structureProfileUri")]
    pub structure_profile_uri: Option<String>,

    /// Label for instance
    pub title: String,

    /// Human-friendly description of the instance
    pub description: Option<String>,

    /// Example instance data
    pub content: Option<Box<Reference>>,

    /// Snapshot of instance that changes
    pub version: Option<Vec<ExampleScenarioInstanceVersion>>,

    /// Resources contained in the instance
    #[serde(rename = "containedInstance")]
    pub contained_instance: Option<Vec<ExampleScenarioInstanceContainedInstance>>,
}

/// Snapshot of instance that changes
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

    /// ID or acronym of the version
    pub key: String,

    /// Label for instance version
    pub title: String,

    /// Details about version
    pub description: Option<String>,

    /// Example instance version data
    pub content: Option<Box<Reference>>,
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

    /// Key of contained instance
    #[serde(rename = "instanceReference")]
    pub instance_reference: String,

    /// Key of contained instance version
    #[serde(rename = "versionReference")]
    pub version_reference: Option<String>,
}

/// Major process within scenario
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

    /// Label for procss
    pub title: String,

    /// Human-friendly description of the process
    pub description: Option<String>,

    /// Status before process starts
    #[serde(rename = "preConditions")]
    pub pre_conditions: Option<String>,

    /// Status after successful completion
    #[serde(rename = "postConditions")]
    pub post_conditions: Option<String>,

    /// Event within of the process
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

/// Event within of the process
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

    /// Sequential number of the step
    pub number: Option<String>,

    /// Step is nested process
    pub process: Option<ExampleScenarioProcess>,

    /// Step is nested workflow
    pub workflow: Option<String>,

    /// Step is simple action
    pub operation: Option<ExampleScenarioProcessStepOperation>,

    /// Alternate non-typical step action
    pub alternative: Option<Vec<ExampleScenarioProcessStepAlternative>>,

    /// Pause in the flow?
    pub pause: Option<bool>,
}

/// Step is simple action
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

    /// Kind of action
    #[serde(rename = "type")]
    pub r#type: Option<Coding>,

    /// Label for step
    pub title: String,

    /// Who starts the operation
    pub initiator: Option<String>,

    /// Who receives the operation
    pub receiver: Option<String>,

    /// Human-friendly description of the operation
    pub description: Option<String>,

    /// Initiator stays active?
    #[serde(rename = "initiatorActive")]
    pub initiator_active: Option<bool>,

    /// Receiver stays active?
    #[serde(rename = "receiverActive")]
    pub receiver_active: Option<bool>,

    /// Instance transmitted on invocation
    pub request: Option<ExampleScenarioInstanceContainedInstance>,

    /// Instance transmitted on invocation response
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

    /// Human-readable description of option
    pub description: Option<String>,

    /// Alternative action(s)
    pub step: Option<Vec<ExampleScenarioProcessStep>>,
}

/// A walkthrough of a workflow showing the interaction between systems and the instances shared, possibly including the evolution of instances over time.
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

    /// How to compare versions
    #[serde(rename = "versionAlgorithmString")]
    pub version_algorithm_string: Option<String>,

    #[serde(rename = "versionAlgorithmCoding")]
    pub version_algorithm_coding: Option<Coding>,

    /// To be removed?
    pub name: Option<String>,

    /// Name for this example scenario (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the ExampleScenario
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for example scenario (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// The purpose of the example, e.g. to illustrate a scenario
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// Individual involved in exchange
    pub actor: Option<Vec<ExampleScenarioActor>>,

    /// Data used in the scenario
    pub instance: Option<Vec<ExampleScenarioInstance>>,

    /// Major process within scenario
    pub process: Option<Vec<ExampleScenarioProcess>>,
}
