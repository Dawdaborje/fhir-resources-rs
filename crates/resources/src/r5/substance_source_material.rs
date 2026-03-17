//! FHIR R5 SubstanceSourceMaterial Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances deriv...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialFractionDescription {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// This element is capturing information about the fraction of a plant part, or human plasma for fractionation
    pub fraction: Option<String>,

    /// The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1
    #[serde(rename = "materialType")]
    pub material_type: Option<CodeableConcept>,
}

/// This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will ...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganism {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The family of an organism shall be specified
    pub family: Option<CodeableConcept>,

    /// The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name; it is present in names for genera, species and infraspecies
    pub genus: Option<CodeableConcept>,

    /// The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal; it is present in names for species and infraspecies
    pub species: Option<CodeableConcept>,

    /// The Intraspecific type of an organism shall be specified
    #[serde(rename = "intraspecificType")]
    pub intraspecific_type: Option<CodeableConcept>,

    /// The intraspecific description of an organism shall be specified based on a controlled vocabulary. For Influenza Vaccine, the intraspecific description shall contain the syntax of the antigen in lin...
    #[serde(rename = "intraspecificDescription")]
    pub intraspecific_description: Option<String>,

    /// 4.9.13.6.1 Author type (Conditional)
    pub author: Option<Vec<SubstanceSourceMaterialOrganismAuthor>>,

    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
    pub hybrid: Option<SubstanceSourceMaterialOrganismHybrid>,

    /// 4.9.13.7.1 Kingdom (Conditional)
    #[serde(rename = "organismGeneral")]
    pub organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneral>,
}

/// 4.9.13.6.1 Author type (Conditional)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismAuthor {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of author of an organism species shall be specified. The parenthetical author of an organism species refers to the first author who published the plant/animal name (of any rank). The prima...
    #[serde(rename = "authorType")]
    pub author_type: Option<CodeableConcept>,

    /// The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable; refers to the year in which the first author(s) published the infraspec...
    #[serde(rename = "authorDescription")]
    pub author_description: Option<String>,
}

/// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismHybrid {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary. For plants, the parents aren’t always known, and it is unlikely that i...
    #[serde(rename = "maternalOrganismId")]
    pub maternal_organism_id: Option<String>,

    /// The name of the maternal species constituting the hybrid organism shall be specified. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and w...
    #[serde(rename = "maternalOrganismName")]
    pub maternal_organism_name: Option<String>,

    /// The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary
    #[serde(rename = "paternalOrganismId")]
    pub paternal_organism_id: Option<String>,

    /// The name of the paternal species constituting the hybrid organism shall be specified
    #[serde(rename = "paternalOrganismName")]
    pub paternal_organism_name: Option<String>,

    /// The hybrid type of an organism shall be specified
    #[serde(rename = "hybridType")]
    pub hybrid_type: Option<CodeableConcept>,
}

/// 4.9.13.7.1 Kingdom (Conditional)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialOrganismOrganismGeneral {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kingdom of an organism shall be specified
    pub kingdom: Option<CodeableConcept>,

    /// The phylum of an organism shall be specified
    pub phylum: Option<CodeableConcept>,

    /// The class of an organism shall be specified
    pub class: Option<CodeableConcept>,

    /// The order of an organism shall be specified,
    pub order: Option<CodeableConcept>,
}

/// To do
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterialPartDescription {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Entity of anatomical origin of source material within an organism
    pub part: Option<CodeableConcept>,

    /// The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply
    #[serde(rename = "partLocation")]
    pub part_location: Option<CodeableConcept>,
}

/// Source material shall capture information on the taxonomic and anatomical origins as well as the fraction of a material that can result in or can be modified to form a substance. This set of data e...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstanceSourceMaterial {
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

    /// General high level classification of the source material specific to the origin of the material
    #[serde(rename = "sourceMaterialClass")]
    pub source_material_class: Option<CodeableConcept>,

    /// The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent
    #[serde(rename = "sourceMaterialType")]
    pub source_material_type: Option<CodeableConcept>,

    /// The state of the source material when extracted
    #[serde(rename = "sourceMaterialState")]
    pub source_material_state: Option<CodeableConcept>,

    /// The unique identifier associated with the source material parent organism shall be specified
    #[serde(rename = "organismId")]
    pub organism_id: Option<Box<Identifier>>,

    /// The organism accepted Scientific name shall be provided based on the organism taxonomy
    #[serde(rename = "organismName")]
    pub organism_name: Option<String>,

    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)
    #[serde(rename = "parentSubstanceId")]
    pub parent_substance_id: Option<Vec<Box<Identifier>>>,

    /// The parent substance of the Herbal Drug, or Herbal preparation
    #[serde(rename = "parentSubstanceName")]
    pub parent_substance_name: Option<Vec<String>>,

    /// The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File. For “Plasma-derived substances” the a...
    #[serde(rename = "countryOfOrigin")]
    pub country_of_origin: Option<Vec<CodeableConcept>>,

    /// The place/region where the plant is harvested or the places/regions where the animal source material has its habitat
    #[serde(rename = "geographicalLocation")]
    pub geographical_location: Option<Vec<String>>,

    /// Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)
    #[serde(rename = "developmentStage")]
    pub development_stage: Option<CodeableConcept>,

    /// Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances deriv...
    #[serde(rename = "fractionDescription")]
    pub fraction_description: Option<Vec<SubstanceSourceMaterialFractionDescription>>,

    /// This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will ...
    pub organism: Option<SubstanceSourceMaterialOrganism>,

    /// To do
    #[serde(rename = "partDescription")]
    pub part_description: Option<Vec<SubstanceSourceMaterialPartDescription>>,
}
