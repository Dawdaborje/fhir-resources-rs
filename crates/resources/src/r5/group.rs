//! FHIR R5 Group Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Include / Exclude group members by Trait
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupCharacteristic {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Kind of characteristic
    pub code: CodeableConcept,

    /// Value held by characteristic
    pub value: serde_json::Value,

    /// Group includes or excludes
    pub exclude: bool,

    /// Period over which characteristic is tested
    pub period: Option<Period>,
}

/// Who or what is in group
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupMember {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Reference to the group member
    pub entity: Box<Reference>,

    /// Period member belonged to the group
    pub period: Option<Period>,

    /// If member is no longer in group
    pub inactive: Option<bool>,
}

/// Represents a defined collection of entities that may be discussed or acted upon collectively but which are not expected to act collectively, and are not formally or legally recognized; i.e. a colle...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
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

    /// Business Identifier for this Group
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Whether this group's record is in active use
    pub active: Option<bool>,

    /// person | animal | practitioner | device | careteam | healthcareservice | location | organization | relatedperson | specimen
    #[serde(rename = "type")]
    pub r#type: String,

    /// definitional | enumerated
    pub membership: String,

    /// Kind of Group members
    pub code: Option<CodeableConcept>,

    /// Label for Group
    pub name: Option<String>,

    /// Natural language description of the group
    pub description: Option<String>,

    /// Number of members
    pub quantity: Option<u32>,

    /// Entity that is the custodian of the Group's definition
    #[serde(rename = "managingEntity")]
    pub managing_entity: Option<Box<Reference>>,

    /// Include / Exclude group members by Trait
    pub characteristic: Option<Vec<GroupCharacteristic>>,

    /// Who or what is in group
    pub member: Option<Vec<GroupMember>>,
}
