//! FHIR R4 MedicinalProductManufactured Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// The manufactured item as contained in the packaged medicinal product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MedicinalProductManufactured {
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

    /// Dose form as manufactured and before any transformation into the pharmaceutical product
    #[serde(rename = "manufacturedDoseForm")]
    pub manufactured_dose_form: CodeableConcept,

    /// The “real world” units in which the quantity of the manufactured item is described
    #[serde(rename = "unitOfPresentation")]
    pub unit_of_presentation: Option<CodeableConcept>,

    /// The quantity or "count number" of the manufactured item
    pub quantity: Quantity,

    /// Manufacturer of the item (Note that this should be named "manufacturer" but it currently causes technical issues)
    pub manufacturer: Option<Vec<Box<Reference>>>,

    /// Ingredient
    pub ingredient: Option<Vec<Box<Reference>>>,

    /// Dimensions, color etc.
    #[serde(rename = "physicalCharacteristics")]
    pub physical_characteristics: Option<ProdCharacteristic>,

    /// Other codeable characteristics
    #[serde(rename = "otherCharacteristics")]
    pub other_characteristics: Option<Vec<CodeableConcept>>,
}
