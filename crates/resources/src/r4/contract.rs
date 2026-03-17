//! FHIR R4 Contract Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4::types::*;
use serde::{Deserialize, Serialize};

/// Contract precursor content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractContentDefinition {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Content structure and use
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Detailed Content Type Definition
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,

    /// Publisher Entity
    pub publisher: Option<Box<Reference>>,

    /// When published
    #[serde(rename = "publicationDate")]
    pub publication_date: Option<String>,

    /// amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated
    #[serde(rename = "publicationStatus")]
    pub publication_status: String,

    /// Publication Ownership
    pub copyright: Option<String>,
}

/// Contract Term List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerm {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Contract Term Number
    pub identifier: Option<Box<Identifier>>,

    /// Contract Term Issue Date Time
    pub issued: Option<String>,

    /// Contract Term Effective Time
    pub applies: Option<Period>,

    /// Term Concern
    #[serde(rename = "topicCodeableConcept")]
    pub topic_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "topicReference")]
    pub topic_reference: Option<Box<Reference>>,

    /// Contract Term Type or Form
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Contract Term Type specific classification
    #[serde(rename = "subType")]
    pub sub_type: Option<CodeableConcept>,

    /// Term Statement
    pub text: Option<String>,

    /// Protection for the Term
    #[serde(rename = "securityLabel")]
    pub security_label: Option<Vec<ContractTermSecurityLabel>>,

    /// Context of the Contract term
    pub offer: ContractTermOffer,

    /// Contract Term Asset List
    pub asset: Option<Vec<ContractTermAsset>>,

    /// Entity being ascribed responsibility
    pub action: Option<Vec<ContractTermAction>>,

    /// Nested Contract Term Group
    pub group: Option<Vec<ContractTerm>>,
}

/// Protection for the Term
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermSecurityLabel {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Link to Security Labels
    pub number: Option<Vec<u32>>,

    /// Confidentiality Protection
    pub classification: Coding,

    /// Applicable Policy
    pub category: Option<Vec<Coding>>,

    /// Handling Instructions
    pub control: Option<Vec<Coding>>,
}

/// Context of the Contract term
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOffer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Offer business ID
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Offer Recipient
    pub party: Option<Vec<ContractTermOfferParty>>,

    /// Negotiable offer asset
    pub topic: Option<Box<Reference>>,

    /// Contract Offer Type or Form
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Accepting party choice
    pub decision: Option<CodeableConcept>,

    /// How decision is conveyed
    #[serde(rename = "decisionMode")]
    pub decision_mode: Option<Vec<CodeableConcept>>,

    /// Response to offer text
    pub answer: Option<Vec<ContractTermOfferAnswer>>,

    /// Human readable offer text
    pub text: Option<String>,

    /// Pointer to text
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<String>>,

    /// Offer restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<u32>>,
}

/// Offer Recipient
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOfferParty {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Referenced entity
    pub reference: Vec<Box<Reference>>,

    /// Participant engagement type
    pub role: CodeableConcept,
}

/// Response to offer text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermOfferAnswer {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The actual answer response
    #[serde(rename = "valueBoolean")]
    pub value_boolean: bool,

    #[serde(rename = "valueDecimal")]
    pub value_decimal: f64,

    #[serde(rename = "valueInteger")]
    pub value_integer: i32,

    #[serde(rename = "valueDate")]
    pub value_date: String,

    #[serde(rename = "valueDateTime")]
    pub value_date_time: String,

    #[serde(rename = "valueTime")]
    pub value_time: String,

    #[serde(rename = "valueString")]
    pub value_string: String,

    #[serde(rename = "valueUri")]
    pub value_uri: String,

    #[serde(rename = "valueAttachment")]
    pub value_attachment: Attachment,

    #[serde(rename = "valueCoding")]
    pub value_coding: Coding,

    #[serde(rename = "valueQuantity")]
    pub value_quantity: Quantity,

    #[serde(rename = "valueReference")]
    pub value_reference: Box<Reference>,
}

/// Contract Term Asset List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAsset {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Range of asset
    pub scope: Option<CodeableConcept>,

    /// Asset category
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Associated entities
    #[serde(rename = "typeReference")]
    pub type_reference: Option<Vec<Box<Reference>>>,

    /// Asset sub-category
    pub subtype: Option<Vec<CodeableConcept>>,

    /// Kinship of the asset
    pub relationship: Option<Coding>,

    /// Circumstance of the asset
    pub context: Option<Vec<ContractTermAssetContext>>,

    /// Quality desctiption of asset
    pub condition: Option<String>,

    /// Asset availability types
    #[serde(rename = "periodType")]
    pub period_type: Option<Vec<CodeableConcept>>,

    /// Time period of the asset
    pub period: Option<Vec<Period>>,

    /// Time period
    #[serde(rename = "usePeriod")]
    pub use_period: Option<Vec<Period>>,

    /// Asset clause or question text
    pub text: Option<String>,

    /// Pointer to asset text
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<String>>,

    /// Response to assets
    pub answer: Option<Vec<ContractTermOfferAnswer>>,

    /// Asset restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<u32>>,

    /// Contract Valued Item List
    #[serde(rename = "valuedItem")]
    pub valued_item: Option<Vec<ContractTermAssetValuedItem>>,
}

/// Circumstance of the asset
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAssetContext {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Creator,custodian or owner
    pub reference: Option<Box<Reference>>,

    /// Codeable asset context
    pub code: Option<Vec<CodeableConcept>>,

    /// Context description
    pub text: Option<String>,
}

/// Contract Valued Item List
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAssetValuedItem {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Contract Valued Item Type
    #[serde(rename = "entityCodeableConcept")]
    pub entity_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "entityReference")]
    pub entity_reference: Option<Box<Reference>>,

    /// Contract Valued Item Number
    pub identifier: Option<Box<Identifier>>,

    /// Contract Valued Item Effective Tiem
    #[serde(rename = "effectiveTime")]
    pub effective_time: Option<String>,

    /// Count of Contract Valued Items
    pub quantity: Option<Quantity>,

    /// Contract Valued Item fee, charge, or cost
    #[serde(rename = "unitPrice")]
    pub unit_price: Option<Money>,

    /// Contract Valued Item Price Scaling Factor
    pub factor: Option<f64>,

    /// Contract Valued Item Difficulty Scaling Factor
    pub points: Option<f64>,

    /// Total Contract Valued Item Value
    pub net: Option<Money>,

    /// Terms of valuation
    pub payment: Option<String>,

    /// When payment is due
    #[serde(rename = "paymentDate")]
    pub payment_date: Option<String>,

    /// Who will make payment
    pub responsible: Option<Box<Reference>>,

    /// Who will receive payment
    pub recipient: Option<Box<Reference>>,

    /// Pointer to specific item
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<String>>,

    /// Security Labels that define affected terms
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<u32>>,
}

/// Entity being ascribed responsibility
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermAction {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// True if the term prohibits the action
    #[serde(rename = "doNotPerform")]
    pub do_not_perform: Option<bool>,

    /// Type or form of the action
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// Entity of the action
    pub subject: Option<Vec<ContractTermActionSubject>>,

    /// Purpose for the Contract Term Action
    pub intent: CodeableConcept,

    /// Pointer to specific item
    #[serde(rename = "linkId")]
    pub link_id: Option<Vec<String>>,

    /// State of the action
    pub status: CodeableConcept,

    /// Episode associated with action
    pub context: Option<Box<Reference>>,

    /// Pointer to specific item
    #[serde(rename = "contextLinkId")]
    pub context_link_id: Option<Vec<String>>,

    /// When action happens
    #[serde(rename = "occurrenceDateTime")]
    pub occurrence_date_time: Option<String>,

    #[serde(rename = "occurrencePeriod")]
    pub occurrence_period: Option<Period>,

    #[serde(rename = "occurrenceTiming")]
    pub occurrence_timing: Option<Timing>,

    /// Who asked for action
    pub requester: Option<Vec<Box<Reference>>>,

    /// Pointer to specific item
    #[serde(rename = "requesterLinkId")]
    pub requester_link_id: Option<Vec<String>>,

    /// Kind of service performer
    #[serde(rename = "performerType")]
    pub performer_type: Option<Vec<CodeableConcept>>,

    /// Competency of the performer
    #[serde(rename = "performerRole")]
    pub performer_role: Option<CodeableConcept>,

    /// Actor that wil execute (or not) the action
    pub performer: Option<Box<Reference>>,

    /// Pointer to specific item
    #[serde(rename = "performerLinkId")]
    pub performer_link_id: Option<Vec<String>>,

    /// Why is action (not) needed?
    #[serde(rename = "reasonCode")]
    pub reason_code: Option<Vec<CodeableConcept>>,

    /// Why is action (not) needed?
    #[serde(rename = "reasonReference")]
    pub reason_reference: Option<Vec<Box<Reference>>>,

    /// Why action is to be performed
    pub reason: Option<Vec<String>>,

    /// Pointer to specific item
    #[serde(rename = "reasonLinkId")]
    pub reason_link_id: Option<Vec<String>>,

    /// Comments about the action
    pub note: Option<Vec<Annotation>>,

    /// Action restriction numbers
    #[serde(rename = "securityLabelNumber")]
    pub security_label_number: Option<Vec<u32>>,
}

/// Entity of the action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractTermActionSubject {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Entity of the action
    pub reference: Vec<Box<Reference>>,

    /// Role type of the agent
    pub role: Option<CodeableConcept>,
}

/// Contract Signatory
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractSigner {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Contract Signatory Role
    #[serde(rename = "type")]
    pub r#type: Coding,

    /// Contract Signatory Party
    pub party: Box<Reference>,

    /// Contract Documentation Signature
    pub signature: Vec<Signature>,
}

/// Contract Friendly Language
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractFriendly {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Easily comprehended representation of this Contract
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,

    #[serde(rename = "contentReference")]
    pub content_reference: Box<Reference>,
}

/// Contract Legal Language
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractLegal {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Contract Legal Text
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,

    #[serde(rename = "contentReference")]
    pub content_reference: Box<Reference>,
}

/// Computable Contract Language
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractRule {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Computable Contract Rules
    #[serde(rename = "contentAttachment")]
    pub content_attachment: Attachment,

    #[serde(rename = "contentReference")]
    pub content_reference: Box<Reference>,
}

/// Legally enforceable, formally recorded unilateral or bilateral directive i.e., a policy or agreement.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
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

    /// Contract number
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Basal definition
    pub url: Option<String>,

    /// Business edition
    pub version: Option<String>,

    /// amended | appended | cancelled | disputed | entered-in-error | executable | executed | negotiable | offered | policy | rejected | renewed | revoked | resolved | terminated
    pub status: Option<String>,

    /// Negotiation status
    #[serde(rename = "legalState")]
    pub legal_state: Option<CodeableConcept>,

    /// Source Contract Definition
    #[serde(rename = "instantiatesCanonical")]
    pub instantiates_canonical: Option<Box<Reference>>,

    /// External Contract Definition
    #[serde(rename = "instantiatesUri")]
    pub instantiates_uri: Option<String>,

    /// Content derived from the basal information
    #[serde(rename = "contentDerivative")]
    pub content_derivative: Option<CodeableConcept>,

    /// When this Contract was issued
    pub issued: Option<String>,

    /// Effective time
    pub applies: Option<Period>,

    /// Contract cessation cause
    #[serde(rename = "expirationType")]
    pub expiration_type: Option<CodeableConcept>,

    /// Contract Target Entity
    pub subject: Option<Vec<Box<Reference>>>,

    /// Authority under which this Contract has standing
    pub authority: Option<Vec<Box<Reference>>>,

    /// A sphere of control governed by an authoritative jurisdiction, organization, or person
    pub domain: Option<Vec<Box<Reference>>>,

    /// Specific Location
    pub site: Option<Vec<Box<Reference>>>,

    /// Computer friendly designation
    pub name: Option<String>,

    /// Human Friendly name
    pub title: Option<String>,

    /// Subordinate Friendly name
    pub subtitle: Option<String>,

    /// Acronym or short name
    pub alias: Option<Vec<String>>,

    /// Source of Contract
    pub author: Option<Box<Reference>>,

    /// Range of Legal Concerns
    pub scope: Option<CodeableConcept>,

    /// Focus of contract interest
    #[serde(rename = "topicCodeableConcept")]
    pub topic_codeable_concept: Option<CodeableConcept>,

    #[serde(rename = "topicReference")]
    pub topic_reference: Option<Box<Reference>>,

    /// Legal instrument category
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Subtype within the context of type
    #[serde(rename = "subType")]
    pub sub_type: Option<Vec<CodeableConcept>>,

    /// Contract precursor content
    #[serde(rename = "contentDefinition")]
    pub content_definition: Option<ContractContentDefinition>,

    /// Contract Term List
    pub term: Option<Vec<ContractTerm>>,

    /// Extra Information
    #[serde(rename = "supportingInfo")]
    pub supporting_info: Option<Vec<Box<Reference>>>,

    /// Key event in Contract History
    #[serde(rename = "relevantHistory")]
    pub relevant_history: Option<Vec<Box<Reference>>>,

    /// Contract Signatory
    pub signer: Option<Vec<ContractSigner>>,

    /// Contract Friendly Language
    pub friendly: Option<Vec<ContractFriendly>>,

    /// Contract Legal Language
    pub legal: Option<Vec<ContractLegal>>,

    /// Computable Contract Language
    pub rule: Option<Vec<ContractRule>>,

    /// Binding Contract
    #[serde(rename = "legallyBindingAttachment")]
    pub legally_binding_attachment: Option<Attachment>,

    #[serde(rename = "legallyBindingReference")]
    pub legally_binding_reference: Option<Box<Reference>>,
}
