//! FHIR R5 Task Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Who or what performed the task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskPerformer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of performance
    pub function: Option<CodeableConcept>,

    /// Who performed the task
    pub actor: Box<Reference>,
}

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

    /// When fulfillment is sought
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
    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: String,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueCanonical")]
    pub value_canonical: String,

    #[serde(rename = "valueCode")]
    pub value_code: String,

    #[serde(rename = "valueDate")]
    pub value_date: String,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,

    #[serde(rename = "valueId")]
    pub value_id: String,

    #[serde(rename = "valueInstant")]
    pub value_instant: String,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueInteger64")]
    pub value_integer64: i64,

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: String,

    #[serde(rename = "valueOid")]
    pub value_oid: String,

    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: i32,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueTime")]
    pub value_time: String,

    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: u32,

    #[serde(rename = "valueUri")]
    pub value_uri: String,

    #[serde(rename = "valueUrl")]
    pub value_url: String,

    #[serde(rename = "valueUuid")]
    pub value_uuid: String,

    #[serde(rename = "valueAddress")]
    pub value_address: Address,

    #[serde(rename = "valueAge")]
    pub value_age: Age,

    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueCodeableReference")]
    pub value_codeable_reference: CodeableReference,

    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,

    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: ContactPoint,

    #[serde(rename = "valueCount")]
    pub value_count: Count,

    #[serde(rename = "valueDistance")]
    pub value_distance: Distance,

    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,

    #[serde(rename = "valueHumanName")]
    pub value_human_name: HumanName,

    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Box<Identifier>,

    #[serde(rename = "valueMoney")]
    pub value_money: Money,

    #[serde(rename = "valuePeriod")]
    pub value_period: Period,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,

    #[serde(rename = "valueRatioRange")]
    pub value_ratio_range: RatioRange,

    #[serde(rename = "valueReference")]
    pub value_reference: Box<Reference>,

    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: SampledData,

    #[serde(rename = "valueSignature")]
    pub value_signature: Signature,

    #[serde(rename = "valueTiming")]
    pub value_timing: Timing,

    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: ContactDetail,

    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: DataRequirement,

    #[serde(rename = "valueExpression")]
    pub value_expression: Expression,

    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: ParameterDefinition,

    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: RelatedArtifact,

    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: TriggerDefinition,

    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: UsageContext,

    #[serde(rename = "valueAvailability")]
    pub value_availability: Availability,

    #[serde(rename = "valueExtendedContactDetail")]
    pub value_extended_contact_detail: ExtendedContactDetail,

    #[serde(rename = "valueDosage")]
    pub value_dosage: Dosage,

    #[serde(rename = "valueMeta")]
    pub value_meta: Meta,
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
    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: String,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueCanonical")]
    pub value_canonical: String,

    #[serde(rename = "valueCode")]
    pub value_code: String,

    #[serde(rename = "valueDate")]
    pub value_date: String,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,

    #[serde(rename = "valueId")]
    pub value_id: String,

    #[serde(rename = "valueInstant")]
    pub value_instant: String,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueInteger64")]
    pub value_integer64: i64,

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: String,

    #[serde(rename = "valueOid")]
    pub value_oid: String,

    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: i32,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueTime")]
    pub value_time: String,

    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: u32,

    #[serde(rename = "valueUri")]
    pub value_uri: String,

    #[serde(rename = "valueUrl")]
    pub value_url: String,

    #[serde(rename = "valueUuid")]
    pub value_uuid: String,

    #[serde(rename = "valueAddress")]
    pub value_address: Address,

    #[serde(rename = "valueAge")]
    pub value_age: Age,

    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Annotation,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: CodeableConcept,

    #[serde(rename = "valueCodeableReference")]
    pub value_codeable_reference: CodeableReference,

    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,

    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: ContactPoint,

    #[serde(rename = "valueCount")]
    pub value_count: Count,

    #[serde(rename = "valueDistance")]
    pub value_distance: Distance,

    #[serde(rename = "valueDuration")]
    pub value_duration: Duration,

    #[serde(rename = "valueHumanName")]
    pub value_human_name: HumanName,

    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Box<Identifier>,

    #[serde(rename = "valueMoney")]
    pub value_money: Money,

    #[serde(rename = "valuePeriod")]
    pub value_period: Period,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueRange")]
    pub value_range: Range,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Ratio,

    #[serde(rename = "valueRatioRange")]
    pub value_ratio_range: RatioRange,

    #[serde(rename = "valueReference")]
    pub value_reference: Box<Reference>,

    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: SampledData,

    #[serde(rename = "valueSignature")]
    pub value_signature: Signature,

    #[serde(rename = "valueTiming")]
    pub value_timing: Timing,

    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: ContactDetail,

    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: DataRequirement,

    #[serde(rename = "valueExpression")]
    pub value_expression: Expression,

    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: ParameterDefinition,

    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: RelatedArtifact,

    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: TriggerDefinition,

    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: UsageContext,

    #[serde(rename = "valueAvailability")]
    pub value_availability: Availability,

    #[serde(rename = "valueExtendedContactDetail")]
    pub value_extended_contact_detail: ExtendedContactDetail,

    #[serde(rename = "valueDosage")]
    pub value_dosage: Dosage,

    #[serde(rename = "valueMeta")]
    pub value_meta: Meta,
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
    pub status_reason: Option<CodeableReference>,

    /// E.g. "Specimen collected", "IV prepped"
    #[serde(rename = "businessStatus")]
    pub business_status: Option<CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// True if Task is prohibiting action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

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

    /// When the task should be performed
    #[serde(rename = "requestedPeriod")]
    pub requested_period: Option<Period>,

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

    /// Who should perform Task
    #[serde(rename = "requestedPerformer")]
    pub requested_performer: Option<Vec<CodeableReference>>,

    /// Responsible individual
    pub owner: Option<Box<Reference>>,

    /// Who or what performed the task
    pub performer: Option<Vec<TaskPerformer>>,

    /// Where task occurs
    pub location: Option<Box<Reference>>,

    /// Why task is needed
    pub reason: Option<Vec<CodeableReference>>,

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
