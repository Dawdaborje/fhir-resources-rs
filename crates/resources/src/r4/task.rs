//! FHIR R4 Task Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Constraints on fulfillment tasks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskRestriction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How many times to repeat
    pub repetitions: Option<i32>,

    /// When fulfillment sought
    pub period: Option<Period>,

    /// For whom is fulfillment sought?
    pub recipient: Option<Vec<Box<Reference>>>,
}

/// Information used to perform task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for the input
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Content to use in performing the task
    pub value: serde_json::Value,
}

/// Information produced as part of task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskOutput {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Label for output
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Result of output
    pub value: serde_json::Value,
}

/// A task to be performed.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
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

    /// Task Instance Identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Formal definition of task
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<String>,

    /// Formal definition of task
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Request fulfilled by this task
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Requisition or grouper id
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// Composite task
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// draft | requested | received | accepted | +
    pub status: String,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// E.g. "Specimen collected", "IV prepped"
    #[serde(rename = "businessStatus")]
    pub business_status: Option<CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Task Type
    pub code: Option<CodeableConcept>,

    /// Human-readable explanation of task
    pub description: Option<String>,

    /// What task is acting on
    pub focus: Option<Box<Reference>>,

    /// Beneficiary of the Task
    #[serde(rename = "for")]
    pub r#for: Option<Box<Reference>>,

    /// Healthcare event during which this task originated
    pub encounter: Option<Box<Reference>>,

    /// Start and end time of execution
    #[serde(rename = "executionPeriod")]
    pub execution_period: Option<Period>,

    /// Task Creation Date
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Task Last Modified Date
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,

    /// Who is asking for task to be done
    pub requester: Option<Box<Reference>>,

    /// Requested performer
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,

    /// Responsible individual
    pub owner: Option<Box<Reference>>,

    /// Where task occurs
    pub location: Option<Box<Reference>>,

    /// Why task is needed
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<CodeableConcept>,

    /// Why task is needed
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Box<Reference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Comments made about the task
    pub note: Option<Vec<Annotation>>,

    /// Key events in history of the Task
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,

    /// Constraints on fulfillment tasks
    pub restriction: Option<TaskRestriction>,

    /// Information used to perform task
    pub input: Option<Vec<TaskInput>>,

    /// Information produced as part of task
    pub output: Option<Vec<TaskOutput>>,
}
