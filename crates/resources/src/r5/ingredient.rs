//! FHIR R5 Ingredient Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// An organization that manufactures this ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientManufacturer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// allowed | possible | actual
    pub role: Option<String>,

    /// An organization that manufactures this ingredient
    pub manufacturer: Box<Reference>,
}

/// The substance that comprises this ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A code or full resource that represents the ingredient substance
    pub code: CodeableReference,

    /// The quantity of substance, per presentation, or per volume or mass, and type of quantity
    pub strength: Option<Vec<IngredientSubstanceStrength>>,
}

/// The quantity of substance, per presentation, or per volume or mass, and type of quantity
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The quantity of substance in the unit of presentation
    #[serde(rename = "presentationRatio")]
    pub presentation_ratio: Option<Ratio>,

    #[serde(rename = "presentationRatioRange")]
    pub presentation_ratio_range: Option<RatioRange>,

    #[serde(rename = "presentationCodeableConcept")]
    pub presentation_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "presentationQuantity")]
    pub presentation_quantity: Option<Quantity>,

    /// Text of either the whole presentation strength or a part of it (rest being in Strength.presentation as a ratio)
    #[serde(rename = "textPresentation")]
    pub text_presentation: Option<String>,

    /// The strength per unitary volume (or mass)
    #[serde(rename = "concentrationRatio")]
    pub concentration_ratio: Option<Ratio>,

    #[serde(rename = "concentrationRatioRange")]
    pub concentration_ratio_range: Option<RatioRange>,

    #[serde(rename = "concentrationCodeableConcept")]
    pub concentration_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "concentrationQuantity")]
    pub concentration_quantity: Option<Quantity>,

    /// Text of either the whole concentration strength or a part of it (rest being in Strength.concentration as a ratio)
    #[serde(rename = "textConcentration")]
    pub text_concentration: Option<String>,

    /// A code that indicates if the strength is, for example, based on the ingredient substance as stated or on the substance base (when the ingredient is a salt)
    pub basis: Option<CodeableConcept>,

    /// When strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<String>,

    /// Where the strength range applies
    pub country: Option<Vec<CodeableConcept>>,

    /// Strength expressed in terms of a reference substance
    #[serde(rename = "referenceStrength")]
    pub reference_strength: Option<Vec<IngredientSubstanceStrengthReferenceStrength>>,
}

/// Strength expressed in terms of a reference substance
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngredientSubstanceStrengthReferenceStrength {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Relevant reference substance
    pub substance: CodeableReference,

    /// Strength expressed in terms of a reference substance
    #[serde(rename = "strengthRatio")]
    pub strength_ratio: Ratio,

    #[serde(rename = "strengthRatioRange")]
    pub strength_ratio_range: RatioRange,

    #[serde(rename = "strengthQuantity")]
    pub strength_quantity: Quantity,

    /// When strength is measured at a particular point or distance
    #[serde(rename = "measurementPoint")]
    pub measurement_point: Option<String>,

    /// Where the strength range applies
    pub country: Option<Vec<CodeableConcept>>,
}

/// An ingredient of a manufactured item or pharmaceutical product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ingredient {
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

    /// An identifier or code by which the ingredient can be referenced
    pub identifier: Option<Box<Identifier>>,

    /// draft | active | retired | unknown
    pub status: String,

    /// The product which this ingredient is a constituent part of
    #[serde(rename = "for")]
    pub r#for: Option<Vec<Box<Reference>>>,

    /// Purpose of the ingredient within the product, e.g. active, inactive
    pub role: CodeableConcept,

    /// Precise action within the drug product, e.g. antioxidant, alkalizing agent
    pub function: Option<Vec<CodeableConcept>>,

    /// A classification of the ingredient according to where in the physical item it tends to be used, such the outer shell of a tablet, inner body or ink
    pub group: Option<CodeableConcept>,

    /// If the ingredient is a known or suspected allergen
    #[serde(rename = "allergenicIndicator")]
    pub allergenic_indicator: Option<bool>,

    /// A place for providing any notes that are relevant to the component, e.g. removed during process, adjusted for loss on drying
    pub comment: Option<String>,

    /// An organization that manufactures this ingredient
    pub manufacturer: Option<Vec<IngredientManufacturer>>,

    /// The substance that comprises this ingredient
    pub substance: IngredientSubstance,
}
