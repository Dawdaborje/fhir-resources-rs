//! FHIR R4 MedicinalProductPharmaceutical Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Characteristics e.g. a products onset of action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A coded characteristic
    pub code: CodeableConcept,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<CodeableConcept>,
}

/// The path by which the pharmaceutical product is taken into or makes contact with the body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coded expression for the route
    pub code: CodeableConcept,

    /// The first dose (dose quantity) administered in humans can be specified, for a product under investigation, using a numerical value and its unit of measurement
    #[serde(rename = "firstDose")]
    pub first_dose: Option<Quantity>,

    /// The maximum single dose that can be administered as per the protocol of a clinical trial can be specified using a numerical value and its unit of measurement
    #[serde(rename = "maxSingleDose")]
    pub max_single_dose: Option<Quantity>,

    /// The maximum dose per day (maximum dose quantity to be administered in any one 24-h period) that can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxDosePerDay")]
    pub max_dose_per_day: Option<Quantity>,

    /// The maximum dose per treatment period that can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxDosePerTreatmentPeriod")]
    pub max_dose_per_treatment_period: Option<Ratio>,

    /// The maximum treatment period during which an Investigational Medicinal Product can be administered as per the protocol referenced in the clinical trial authorisation
    #[serde(rename = "maxTreatmentPeriod")]
    pub max_treatment_period: Option<Duration>,

    /// A species for which this route applies
    #[serde(rename = "targetSpecies")]
    pub target_species:
        Option<Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>>,
}

/// A species for which this route applies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coded expression for the species
    pub code: CodeableConcept,

    /// A species specific time during which consumption of animal product is not appropriate
    #[serde(rename = "withdrawalPeriod")]
    pub withdrawal_period: Option<
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
    >,
}

/// A species specific time during which consumption of animal product is not appropriate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coded expression for the type of tissue for which the withdrawal period applues, e.g. meat, milk
    pub tissue: CodeableConcept,

    /// A value for the time
    pub value: Quantity,

    /// Extra information about the withdrawal period
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<String>,
}

/// A pharmaceutical product described in terms of its composition and dose form.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductPharmaceutical {
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

    /// An identifier for the pharmaceutical medicinal product
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// The administrable dose form, after necessary reconstitution
    #[serde(rename = "administrableDoseForm")]
    pub administrable_dose_form: CodeableConcept,

    /// Todo
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,

    /// Ingredient
    pub ingredient: Option<Vec<Box<Reference>>>,

    /// Accompanying device
    pub device: Option<Vec<Box<Reference>>>,

    /// Characteristics e.g. a products onset of action
    pub characteristics: Option<Vec<MedicinalProductPharmaceuticalCharacteristics>>,

    /// The path by which the pharmaceutical product is taken into or makes contact with the body
    #[serde(rename = "routeOfAdministration")]
    pub route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
}
