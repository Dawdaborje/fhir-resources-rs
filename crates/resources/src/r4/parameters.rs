//! FHIR R4 Parameters Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Operation Parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParametersParameter {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Name from the definition
    pub name: String,

    /// If parameter is a data type
    #[serde(rename = "valueBase64Binary")]
    pub value_base64binary: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueCanonical")]
    pub value_canonical: Option<String>,

    #[serde(rename = "valueCode")]
    pub value_code: Option<String>,

    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: Option<String>,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: Option<f64>,

    #[serde(rename = "valueId")]
    pub value_id: Option<String>,

    #[serde(rename = "valueInstant")]
    pub value_instant: Option<String>,

    #[serde(rename = "valueInteger")]
    pub value_integer: Option<i32>,

    #[serde(rename = "valueMarkdown")]
    pub value_markdown: Option<String>,

    #[serde(rename = "valueOid")]
    pub value_oid: Option<String>,

    #[serde(rename = "valuePositiveInt")]
    pub value_positive_int: Option<i32>,

    #[serde(rename = "valueString")]
    pub value_string: Option<String>,

    #[serde(rename = "valueTime")]
    pub value_time: Option<String>,

    #[serde(rename = "valueUnsignedInt")]
    pub value_unsigned_int: Option<u32>,

    #[serde(rename = "valueUri")]
    pub value_uri: Option<String>,

    #[serde(rename = "valueUrl")]
    pub value_url: Option<String>,

    #[serde(rename = "valueUuid")]
    pub value_uuid: Option<String>,

    #[serde(rename = "valueAddress")]
    pub value_address: Option<Address>,

    #[serde(rename = "valueAge")]
    pub value_age: Option<Age>,

    #[serde(rename = "valueAnnotation")]
    pub value_annotation: Option<Annotation>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,

    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueCoding")]
    pub value_coding: Option<Coding>,

    #[serde(rename = "valueContactPoint")]
    pub value_contact_point: Option<ContactPoint>,

    #[serde(rename = "valueCount")]
    pub value_count: Option<Count>,

    #[serde(rename = "valueDistance")]
    pub value_distance: Option<Distance>,

    #[serde(rename = "valueDuration")]
    pub value_duration: Option<Duration>,

    #[serde(rename = "valueHumanName")]
    pub value_human_name: Option<HumanName>,

    #[serde(rename = "valueIdentifier")]
    pub value_identifier: Option<Box<Identifier>>,

    #[serde(rename = "valueMoney")]
    pub value_money: Option<Money>,

    #[serde(rename = "valuePeriod")]
    pub value_period: Option<Period>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueRange")]
    pub value_range: Option<Range>,

    #[serde(rename = "valueRatio")]
    pub value_ratio: Option<Ratio>,

    #[serde(rename = "valueReference")]
    pub value_reference: Option<Box<Reference>>,

    #[serde(rename = "valueSampledData")]
    pub value_sampled_data: Option<SampledData>,

    #[serde(rename = "valueSignature")]
    pub value_signature: Option<Signature>,

    #[serde(rename = "valueTiming")]
    pub value_timing: Option<Timing>,

    #[serde(rename = "valueContactDetail")]
    pub value_contact_detail: Option<ContactDetail>,

    #[serde(rename = "valueContributor")]
    pub value_contributor: Option<Contributor>,

    #[serde(rename = "valueDataRequirement")]
    pub value_data_requirement: Option<DataRequirement>,

    #[serde(rename = "valueExpression")]
    pub value_expression: Option<Expression>,

    #[serde(rename = "valueParameterDefinition")]
    pub value_parameter_definition: Option<ParameterDefinition>,

    #[serde(rename = "valueRelatedArtifact")]
    pub value_related_artifact: Option<RelatedArtifact>,

    #[serde(rename = "valueTriggerDefinition")]
    pub value_trigger_definition: Option<TriggerDefinition>,

    #[serde(rename = "valueUsageContext")]
    pub value_usage_context: Option<UsageContext>,

    #[serde(rename = "valueDosage")]
    pub value_dosage: Option<Dosage>,

    #[serde(rename = "valueMeta")]
    pub value_meta: Option<Meta>,

    /// If parameter is a whole resource
    pub resource: Option<serde_json::Value>,

    /// Named part of a multi-part parameter
    pub part: Option<Vec<ParametersParameter>>,
}

/// This resource is a non-persisted resource used to pass information into and back from an [operation](operations.html). It has no other use, and there is no RESTful endpoint associated with it.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters {
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

    /// Operation Parameter
    pub parameter: Option<Vec<ParametersParameter>>,
}
