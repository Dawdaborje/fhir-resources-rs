//! FHIR R5 Transport Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Constraints on fulfillment transports
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportRestriction {
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

/// Information used to perform transport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportInput {
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

    /// Content to use in performing the transport
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

/// Information produced as part of transport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportOutput {
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

/// Record of transport.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
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

    /// External identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Formal definition of transport
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<String>,

    /// Formal definition of transport
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Request fulfilled by this transport
    #[serde(rename = "basedOn")]
    pub based_on: Option<Vec<Box<Reference>>>,

    /// Requisition or grouper id
    #[serde(rename = "groupIdentifier")]
    pub group_identifier: Option<Box<Identifier>>,

    /// Part of referenced event
    #[serde(rename = "partOf")]
    pub part_of: Option<Vec<Box<Reference>>>,

    /// in-progress | completed | abandoned | cancelled | planned | entered-in-error
    pub status: Option<String>,

    /// Reason for current status
    #[serde(rename = "statusReason")]
    pub status_reason: Option<CodeableConcept>,

    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    pub intent: String,

    /// routine | urgent | asap | stat
    pub priority: Option<String>,

    /// Transport Type
    pub code: Option<CodeableConcept>,

    /// Human-readable explanation of transport
    pub description: Option<String>,

    /// What transport is acting on
    pub focus: Option<Box<Reference>>,

    /// Beneficiary of the Transport
    #[serde(rename = "for")]
    pub r#for: Option<Box<Reference>>,

    /// Healthcare event during which this transport originated
    pub encounter: Option<Box<Reference>>,

    /// Completion time of the event (the occurrence)
    #[serde(rename = "completionTime")]
    pub completion_time: Option<String>,

    /// Transport Creation Date
    #[serde(rename = "authoredOn")]
    pub authored_on: Option<String>,

    /// Transport Last Modified Date
    #[serde(rename = "lastModified")]
    pub last_modified: Option<String>,

    /// Who is asking for transport to be done
    pub requester: Option<Box<Reference>>,

    /// Requested performer
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,

    /// Responsible individual
    pub owner: Option<Box<Reference>>,

    /// Where transport occurs
    pub location: Option<Box<Reference>>,

    /// Associated insurance coverage
    pub insurance: Option<Vec<Box<Reference>>>,

    /// Comments made about the transport
    pub note: Option<Vec<Annotation>>,

    /// Key events in history of the Transport
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,

    /// Constraints on fulfillment transports
    pub restriction: Option<TransportRestriction>,

    /// Information used to perform transport
    pub input: Option<Vec<TransportInput>>,

    /// Information produced as part of transport
    pub output: Option<Vec<TransportOutput>>,

    /// The desired location
    #[serde(rename = "requestedLocation")]
    pub requested_location: Box<Reference>,

    /// The entity current location
    #[serde(rename = "currentLocation")]
    pub current_location: Box<Reference>,

    /// Why transport is needed
    pub reason: Option<CodeableReference>,

    /// Parent (or preceding) transport
    pub history: Option<Box<Reference>>,
}
