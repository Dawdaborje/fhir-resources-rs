//! FHIR R5 Location Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The absolute geographic location
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationPosition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Longitude with WGS84 datum
    pub longitude: f64,

    /// Latitude with WGS84 datum
    pub latitude: f64,

    /// Altitude with WGS84 datum
    pub altitude: Option<f64>,
}

/// Details and position information for a place where services are provided and resources and participants may be stored, found, contained, or accommodated.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
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

    /// Unique code or number identifying the location to its users
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// active | suspended | inactive
    pub status: Option<String>,

    /// The operational status of the location (typically only for a bed/room)
    #[serde(rename = "operationalStatus")]
    pub operational_status: Option<Coding>,

    /// Name of the location as used by humans
    pub name: Option<String>,

    /// A list of alternate names that the location is known as, or was known as, in the past
    pub alias: Option<Vec<String>>,

    /// Additional details about the location that could be displayed as further information to identify the location beyond its name
    pub description: Option<String>,

    /// instance | kind
    pub mode: Option<String>,

    /// Type of function performed
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Official contact details for the location
    pub contact: Option<Vec<ExtendedContactDetail>>,

    /// Physical location
    pub address: Option<Address>,

    /// Physical form of the location
    pub form: Option<CodeableConcept>,

    /// The absolute geographic location
    pub position: Option<LocationPosition>,

    /// Organization responsible for provisioning and upkeep
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Box<Reference>>,

    /// Another Location this one is physically a part of
    #[serde(rename = "partOf")]
    pub part_of: Option<Box<Reference>>,

    /// Collection of characteristics (attributes)
    pub characteristic: Option<Vec<CodeableConcept>>,

    /// What days/times during a week is this location usually open (including exceptions)
    #[serde(rename = "hoursOfOperation")]
    pub hours_of_operation: Option<Vec<Availability>>,

    /// Connection details of a virtual service (e.g. conference call)
    #[serde(rename = "virtualService")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,

    /// Technical endpoints providing access to services operated for the location
    pub endpoint: Option<Vec<Box<Reference>>>,
}
