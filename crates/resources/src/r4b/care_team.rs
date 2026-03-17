//! FHIR R4B CareTeam Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Members of the team
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CareTeamParticipant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Type of involvement
    pub role: Option<Vec<CodeableConcept>>,

    /// Who is involved
    pub member: Option<Box<Reference>>,

    /// Organization of the practitioner
    #[serde(rename = "onBehalfOf")]
    pub on_behalf_of: Option<Box<Reference>>,

    /// Time period of participant
    pub period: Option<Period>,
}

/// The Care Team includes all the people and organizations who plan to participate in the coordination and delivery of care for a patient.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CareTeam {
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

    /// External Ids for this team
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// proposed | active | suspended | inactive | entered-in-error
    pub status: Option<String>,

    /// Type of team
    pub category: Option<Vec<CodeableConcept>>,

    /// Name of the team, such as crisis assessment team
    pub name: Option<String>,

    /// Who care team is for
    pub subject: Option<Box<Reference>>,

    /// Encounter created as part of
    pub encounter: Option<Box<Reference>>,

    /// Time period team covers
    pub period: Option<Period>,

    /// Members of the team
    pub participant: Option<Vec<CareTeamParticipant>>,

    /// Why the care team exists
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why the care team exists
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Organization responsible for the care team
    #[serde(rename = "managingOrganization")]
    pub managing_organization: Option<Vec<Box<Reference>>>,

    /// A contact detail for the care team (that applies to all members)
    pub telecom: Option<Vec<ContactPoint>>,

    /// Comments made about the CareTeam
    pub note: Option<Vec<Annotation>>,
}
