//! FHIR R4B AdministrableProductDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// Characteristics e.g. a product's onset of action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionProperty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A code expressing the type of characteristic
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// A value for the characteristic
    #[serde(rename = "valueCodeableConcept")]
    pub value_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Option<Quantity>,

    #[serde(rename = "valueDate")]
    pub value_date: Option<String>,

    #[serde(rename = "valueBoolean")]
    pub value_boolean: Option<bool>,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Option<Attachment>,

    /// The status of characteristic e.g. assigned or pending
    pub status: Option<CodeableConcept>,
}

/// The path by which the product is taken into or makes contact with the body
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministration {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Coded expression for the route
    pub code: CodeableConcept,

    /// The first dose (dose quantity) administered can be specified for the product
    #[serde(rename = "firstDose")]
    pub first_dose: Option<Quantity>,

    /// The maximum single dose that can be administered
    #[serde(rename = "maxSingleDose")]
    pub max_single_dose: Option<Quantity>,

    /// The maximum dose quantity to be administered in any one 24-h period
    #[serde(rename = "maxDosePerDay")]
    pub max_dose_per_day: Option<Quantity>,

    /// The maximum dose per treatment period that can be administered
    #[serde(rename = "maxDosePerTreatmentPeriod")]
    pub max_dose_per_treatment_period: Option<Ratio>,

    /// The maximum treatment period during which the product can be administered
    #[serde(rename = "maxTreatmentPeriod")]
    pub max_treatment_period: Option<Duration>,

    /// A species for which this route applies
    #[serde(rename = "targetSpecies")]
    pub target_species:
        Option<Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpecies>>,
}

/// A species for which this route applies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpecies {
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
        Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
    >,
}

/// A species specific time during which consumption of animal product is not appropriate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of tissue for which the withdrawal period applies, e.g. meat, milk
    pub tissue: CodeableConcept,

    /// A value for the time
    pub value: Quantity,

    /// Extra information about the withdrawal period
    #[serde(rename = "supportingInformation")]
    pub supporting_information: Option<String>,
}

/// A medicinal product in the final form which is suitable for administering to a patient (after any mixing of multiple components, dissolution etc. has been performed).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdministrableProductDefinition {
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

    /// An identifier for the administrable product
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product
    #[serde(rename = "formOf")]
    pub form_of: Option<Vec<Box<Reference>>>,

    /// The dose form of the final product after necessary reconstitution or processing
    #[serde(rename = "administrableDoseForm")]
    pub administrable_dose_form: Option<CodeableConcept>,

    /// The presentation type in which this item is given to a patient. e.g. for a spray - 'puff'
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,

    /// Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form
    #[serde(rename = "producedFrom")]
    pub produced_from: Option<Vec<Box<Reference>>>,

    /// The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton, or using by incoming references from the In...
    pub ingredient: Option<Vec<CodeableConcept>>,

    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product
    pub device: Option<Box<Reference>>,

    /// Characteristics e.g. a product's onset of action
    pub property: Option<Vec<AdministrableProductDefinitionProperty>>,

    /// The path by which the product is taken into or makes contact with the body
    #[serde(rename = "routeOfAdministration")]
    pub route_of_administration: Vec<AdministrableProductDefinitionRouteOfAdministration>,
}
