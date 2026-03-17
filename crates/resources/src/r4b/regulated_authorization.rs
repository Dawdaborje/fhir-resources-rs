//! FHIR R4B RegulatedAuthorization Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// The case or regulatory procedure for granting or amending a regulated authorization. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see l...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegulatedAuthorizationCase {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifier by which this case can be referenced
    pub identifier: Option<Box<Identifier>>,

    /// The defining type of case
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The status associated with the case
    pub status: Option<CodeableConcept>,

    /// Relevant date for this case
    #[serde(rename = "datePeriod")]
    pub date_period: Option<Period>,

    #[serde(rename = "dateDateTime")]
    pub date_date_time: Option<String>,

    /// Applications submitted to obtain a regulated authorization. Steps within the longer running case or procedure
    pub application: Option<Vec<RegulatedAuthorizationCase>>,
}

/// Regulatory approval, clearance or licencing related to a regulated product, treatment, facility or activity that is cited in a guidance, regulation, rule or legislative act. An example is Market Au...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegulatedAuthorization {
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

    /// Business identifier for the authorization, typically assigned by the authorizing body
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The product type, treatment, facility or activity that is being authorized
    pub subject: Option<Vec<Box<Reference>>>,

    /// Overall type of this authorization, for example drug marketing approval, orphan drug designation
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// General textual supporting information
    pub description: Option<String>,

    /// The territory in which the authorization has been granted
    pub region: Option<Vec<CodeableConcept>>,

    /// The status that is authorised e.g. approved. Intermediate states can be tracked with cases and applications
    pub status: Option<CodeableConcept>,

    /// The date at which the current status was assigned
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// The time period in which the regulatory approval etc. is in effect, e.g. a Marketing Authorization includes the date of authorization and/or expiration date
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,

    /// Condition for which the use of the regulated product applies
    pub indication: Option<CodeableReference>,

    /// The intended use of the product, e.g. prevention, treatment
    #[serde(rename = "intendedUse")]
    pub intended_use: Option<CodeableConcept>,

    /// The legal/regulatory framework or reasons under which this authorization is granted
    pub basis: Option<Vec<CodeableConcept>>,

    /// The organization that has been granted this authorization, by the regulator
    pub holder: Option<Box<Reference>>,

    /// The regulatory authority or authorizing body granting the authorization
    pub regulator: Option<Box<Reference>>,

    /// The case or regulatory procedure for granting or amending a regulated authorization. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see l...
    pub case: Option<RegulatedAuthorizationCase>,
}
