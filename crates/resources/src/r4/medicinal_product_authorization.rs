//! FHIR R4 MedicinalProductAuthorization Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Authorization in areas within a country
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductAuthorizationJurisdictionalAuthorization {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The assigned number for the marketing authorization
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Country of authorization
    pub country: Option<CodeableConcept>,

    /// Jurisdiction within a country
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// The legal status of supply in a jurisdiction or region
    #[serde(rename = "legalStatusOfSupply")]
    pub legal_status_of_supply: Option<CodeableConcept>,

    /// The start and expected end date of the authorization
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,
}

/// The regulatory procedure for granting or amending a marketing authorization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductAuthorizationProcedure {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Identifier for this procedure
    pub identifier: Option<Box<Identifier>>,

    /// Type of procedure
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Date of procedure
    pub date: Option<serde_json::Value>,

    /// Applcations submitted to obtain a marketing authorization
    pub application: Option<Vec<MedicinalProductAuthorizationProcedure>>,
}

/// The regulatory authorization of a medicinal product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductAuthorization {
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

    /// Business identifier for the marketing authorization, as assigned by a regulator
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The medicinal product that is being authorized
    pub subject: Option<Box<Reference>>,

    /// The country in which the marketing authorization has been granted
    pub country: Option<Vec<CodeableConcept>>,

    /// Jurisdiction within a country
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// The status of the marketing authorization
    pub status: Option<CodeableConcept>,

    /// The date at which the given status has become applicable
    #[serde(rename = "statusDate")]
    pub status_date: Option<String>,

    /// The date when a suspended the marketing or the marketing authorization of the product is anticipated to be restored
    #[serde(rename = "restoreDate")]
    pub restore_date: Option<String>,

    /// The beginning of the time period in which the marketing authorization is in the specific status shall be specified A complete date consisting of day, month and year shall be specified using the ISO...
    #[serde(rename = "validityPeriod")]
    pub validity_period: Option<Period>,

    /// A period of time after authorization before generic product applicatiosn can be submitted
    #[serde(rename = "dataExclusivityPeriod")]
    pub data_exclusivity_period: Option<Period>,

    /// The date when the first authorization was granted by a Medicines Regulatory Agency
    #[serde(rename = "dateOfFirstAuthorization")]
    pub date_of_first_authorization: Option<String>,

    /// Date of first marketing authorization for a company's new medicinal product in any country in the World
    #[serde(rename = "internationalBirthDate")]
    pub international_birth_date: Option<String>,

    /// The legal framework against which this authorization is granted
    #[serde(rename = "legalBasis")]
    pub legal_basis: Option<CodeableConcept>,

    /// Authorization in areas within a country
    #[serde(rename = "jurisdictionalAuthorization")]
    pub jurisdictional_authorization:
        Option<Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>>,

    /// Marketing Authorization Holder
    pub holder: Option<Box<Reference>>,

    /// Medicines Regulatory Agency
    pub regulator: Option<Box<Reference>>,

    /// The regulatory procedure for granting or amending a marketing authorization
    pub procedure: Option<MedicinalProductAuthorizationProcedure>,
}
