//! FHIR R5 ClinicalUseDefinition Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// Specifics for when this is a contraindication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionContraindication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The situation that is being documented as contraindicating against this item
    #[serde(rename = "diseaseSymptomProcedure")]
    pub disease_symptom_procedure: Option<CodeableReference>,

    /// The status of the disease or symptom for the contraindication
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableReference>,

    /// A comorbidity (concurrent condition) or coinfection
    pub comorbidity: Option<Vec<CodeableReference>>,

    /// The indication which this is a contraidication for
    pub indication: Option<Vec<Box<Reference>>>,

    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    pub applicability: Option<Expression>,

    /// Information about use of the product in relation to other therapies described as part of the contraindication
    #[serde(rename = "otherTherapy")]
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapy>>,
}

/// Information about use of the product in relation to other therapies described as part of the contraindication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionContraindicationOtherTherapy {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of relationship between the product indication/contraindication and another therapy
    #[serde(rename = "relationshipType")]
    pub relationship_type: CodeableConcept,

    /// Reference to a specific medication, substance etc. as part of an indication or contraindication
    pub treatment: CodeableReference,
}

/// Specifics for when this is an indication
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionIndication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The situation that is being documented as an indicaton for this item
    #[serde(rename = "diseaseSymptomProcedure")]
    pub disease_symptom_procedure: Option<CodeableReference>,

    /// The status of the disease or symptom for the indication
    #[serde(rename = "diseaseStatus")]
    pub disease_status: Option<CodeableReference>,

    /// A comorbidity or coinfection as part of the indication
    pub comorbidity: Option<Vec<CodeableReference>>,

    /// The intended effect, aim or strategy to be achieved
    #[serde(rename = "intendedEffect")]
    pub intended_effect: Option<CodeableReference>,

    /// Timing or duration information
    pub duration: Option<serde_json::Value>,

    /// An unwanted side effect or negative outcome of the subject of this resource when being used for this indication
    #[serde(rename = "undesirableEffect")]
    pub undesirable_effect: Option<Vec<Box<Reference>>>,

    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    pub applicability: Option<Expression>,

    /// The use of the medicinal product in relation to other therapies described as part of the indication
    #[serde(rename = "otherTherapy")]
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapy>>,
}

/// Specifics for when this is an interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionInteraction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific medication, product, food etc. or laboratory test that interacts
    pub interactant: Option<Vec<ClinicalUseDefinitionInteractionInteractant>>,

    /// The type of the interaction e.g. drug-drug interaction, drug-lab test interaction
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The effect of the interaction, for example "reduced gastric absorption of primary medication"
    pub effect: Option<CodeableReference>,

    /// The incidence of the interaction, e.g. theoretical, observed
    pub incidence: Option<CodeableConcept>,

    /// Actions for managing the interaction
    pub management: Option<Vec<CodeableConcept>>,
}

/// The specific medication, product, food etc. or laboratory test that interacts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionInteractionInteractant {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific medication, product, food etc. or laboratory test that interacts
    pub item: serde_json::Value,
}

/// A possible negative outcome from the use of this treatment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionUndesirableEffect {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The situation in which the undesirable effect may manifest
    #[serde(rename = "symptomConditionEffect")]
    pub symptom_condition_effect: Option<CodeableReference>,

    /// High level classification of the effect
    pub classification: Option<CodeableConcept>,

    /// How often the effect is seen
    #[serde(rename = "frequencyOfOccurrence")]
    pub frequency_of_occurrence: Option<CodeableConcept>,
}

/// Critical environmental, health or physical risks or hazards. For example 'Do not operate heavy machinery', 'May cause drowsiness'
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinitionWarning {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A textual definition of this warning, with formatting
    pub description: Option<String>,

    /// A coded or unformatted textual definition of this warning
    pub code: Option<CodeableConcept>,
}

/// A single issue - either an indication, contraindication, interaction or an undesirable effect for a medicinal product, medication, device or procedure.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClinicalUseDefinition {
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

    /// Business identifier for this issue
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// indication | contraindication | interaction | undesirable-effect | warning
    #[serde(rename = "type")]
    pub r#type: String,

    /// A categorisation of the issue, primarily for dividing warnings into subject heading areas such as "Pregnancy", "Overdose"
    pub category: Option<Vec<CodeableConcept>>,

    /// The medication, product, substance, device, procedure etc. for which this is an indication
    pub subject: Option<Vec<Box<Reference>>>,

    /// Whether this is a current issue or one that has been retired etc
    pub status: Option<CodeableConcept>,

    /// Specifics for when this is a contraindication
    pub contraindication: Option<ClinicalUseDefinitionContraindication>,

    /// Specifics for when this is an indication
    pub indication: Option<ClinicalUseDefinitionIndication>,

    /// Specifics for when this is an interaction
    pub interaction: Option<ClinicalUseDefinitionInteraction>,

    /// The population group to which this applies
    pub population: Option<Vec<Box<Reference>>>,

    /// Logic used by the clinical use definition
    pub library: Option<Vec<String>>,

    /// A possible negative outcome from the use of this treatment
    #[serde(rename = "undesirableEffect")]
    pub undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffect>,

    /// Critical environmental, health or physical risks or hazards. For example 'Do not operate heavy machinery', 'May cause drowsiness'
    pub warning: Option<ClinicalUseDefinitionWarning>,
}
