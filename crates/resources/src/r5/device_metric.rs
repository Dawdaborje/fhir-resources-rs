//! FHIR R5 DeviceMetric Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Describes the calibrations that have been performed or that are required to be performed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetricCalibration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// unspecified | offset | gain | two-point
    #[serde(rename = "type")]
    pub r#type: Option<String>,

    /// not-calibrated | calibration-required | calibrated | unspecified
    pub state: Option<String>,

    /// Describes the time last calibration has been performed
    pub time: Option<String>,
}

/// Describes a measurement, calculation or setting capability of a device. The DeviceMetric resource is derived from the ISO/IEEE 11073-10201 Domain Information Model standard, but is more widely appl...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceMetric {
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

    /// Instance identifier
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Identity of metric, for example Heart Rate or PEEP Setting
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Unit of Measure for the Metric
    pub unit: Option<CodeableConcept>,

    /// Describes the link to the Device
    pub device: Box<Reference>,

    /// on | off | standby | entered-in-error
    #[serde(rename = "operationalStatus")]
    pub operational_status: Option<String>,

    /// Color name (from CSS4) or #RRGGBB code
    pub color: Option<String>,

    /// measurement | setting | calculation | unspecified
    pub category: String,

    /// Indicates how often the metric is taken or recorded
    #[serde(rename = "measurementFrequency")]
    pub measurement_frequency: Option<Quantity>,

    /// Describes the calibrations that have been performed or that are required to be performed
    pub calibration: Option<Vec<DeviceMetricCalibration>>,
}
