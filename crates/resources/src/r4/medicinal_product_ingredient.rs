//! FHIR R4 MedicinalProductIngredient Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// A specified substance that comprises this ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredientSpecifiedSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specified substance
    pub code: CodeableConcept,

    /// The group of specified substance, e.g. group 1 to 4
    pub group: CodeableConcept,

    /// Confidentiality level of the specified substance as the ingredient
    pub confidentiality: Option<CodeableConcept>,

    /// Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product
    pub strength: Option<Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>>,
}

/// Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The quantity of substance in the unit of presentation, or in the volume (or mass) of the single pharmaceutical product or manufactured item
    pub presentation: Ratio,

    /// A lower limit for the quantity of substance in the unit of presentation. For use when there is a range of strengths, this is the lower limit, with the presentation attribute becoming the upper limit
    #[serde(rename = "presentationLowLimit")]
    pub presentation_low_limit: Option<Ratio>,

    /// The strength per unitary volume (or mass)
    pub concentration: Option<Ratio>,

    /// A lower limit for the strength per unitary volume (or mass), for when there is a range. The concentration attribute then becomes the upper limit
    #[serde(rename = "concentrationLowLimit")]
    pub concentration_low_limit: Option<Ratio>,

    /// For when strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<String>,

    /// The country or countries for which the strength range applies
    pub country: Option<Vec<CodeableConcept>>,

    /// Strength expressed in terms of a reference substance
    #[serde(rename = "referenceStrength")]
    pub reference_strength:
        Option<Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>>,
}

/// Strength expressed in terms of a reference substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Relevant reference substance
    pub substance: Option<CodeableConcept>,

    /// Strength expressed in terms of a reference substance
    pub strength: Ratio,

    /// Strength expressed in terms of a reference substance
    #[serde(rename = "strengthLowLimit")]
    pub strength_low_limit: Option<Ratio>,

    /// For when strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<String>,

    /// The country or countries for which the strength range applies
    pub country: Option<Vec<CodeableConcept>>,
}

/// The ingredient substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredientSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The ingredient substance
    pub code: CodeableConcept,

    /// Quantity of the substance or specified substance present in the manufactured item or pharmaceutical product
    pub strength: Option<Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>>,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductIngredient {
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

    /// Identifier for the ingredient
    pub identifier: Option<Box<Identifier>>,

    /// Ingredient role e.g. Active ingredient, excipient
    pub role: CodeableConcept,

    /// If the ingredient is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<bool>,

    /// Manufacturer of this Ingredient
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// A specified substance that comprises this ingredient
    #[serde(rename = "specifiedSubstance")]
    pub specified_substance: Option<Vec<MedicinalProductIngredientSpecifiedSubstance>>,

    /// The ingredient substance
    pub substance: Option<MedicinalProductIngredientSubstance>,
}
