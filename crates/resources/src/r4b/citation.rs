//! FHIR R4B Citation Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r4b::types::*;
use serde::{Deserialize, Serialize};

/// A human-readable display of the citation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationSummary {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Format for display of the citation
    pub style: Option<CodeableConcept>,

    /// The human-readable display of the citation
    pub text: String,
}

/// The assignment to an organizing scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationClassification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of classifier (e.g. publication type, keyword)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The specific classification value
    pub classifier: Option<Vec<CodeableConcept>>,
}

/// An effective date or period for a status of the citation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationStatusDate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Classification of the status
    pub activity: CodeableConcept,

    /// Either occurred or expected
    pub actual: Option<bool>,

    /// When the status started and/or ended
    pub period: Period,
}

/// Artifact related to the Citation Resource
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How the Citation resource relates to the target artifact
    #[serde(rename = "relationshipType")]
    pub relationship_type: CodeableConcept,

    /// The clasification of the related artifact
    #[serde(rename = "targetClassifier")]
    pub target_classifier: Option<Vec<CodeableConcept>>,

    /// The article or artifact that the Citation Resource is related to
    pub target: serde_json::Value,
}

/// The article or artifact being described
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifact {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// May include DOI, PMID, PMCID, etc.
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// May include trial registry identifiers
    #[serde(rename = "relatedIdentifier")]
    pub related_identifier: Option<Vec<Box<Identifier>>>,

    /// When the cited artifact was accessed
    #[serde(rename = "dateAccessed")]
    pub date_accessed: Option<String>,

    /// The defined version of the cited artifact
    pub version: Option<CitationCitedArtifactVersion>,

    /// The status of the cited artifact
    #[serde(rename = "currentState")]
    pub current_state: Option<Vec<CodeableConcept>>,

    /// An effective date or period for a status of the cited artifact
    #[serde(rename = "statusDate")]
    pub status_date: Option<Vec<CitationCitedArtifactStatusDate>>,

    /// The title details of the article or artifact
    pub title: Option<Vec<CitationCitedArtifactTitle>>,

    /// Summary of the article or artifact
    #[serde(rename = "abstract")]
    pub r#abstract: Option<Vec<CitationCitedArtifactAbstract>>,

    /// The component of the article or artifact
    pub part: Option<CitationCitedArtifactPart>,

    /// The artifact related to the cited artifact
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<CitationCitedArtifactRelatesTo>>,

    /// If multiple, used to represent alternative forms of the article that are not separate citations
    #[serde(rename = "publicationForm")]
    pub publication_form: Option<Vec<CitationCitedArtifactPublicationForm>>,

    /// Used for any URL for the article or artifact cited
    #[serde(rename = "webLocation")]
    pub web_location: Option<Vec<CitationCitedArtifactWebLocation>>,

    /// The assignment to an organizing scheme
    pub classification: Option<Vec<CitationCitedArtifactClassification>>,

    /// Attribution of authors and other contributors
    pub contributorship: Option<CitationCitedArtifactContributorship>,

    /// Any additional information or content for the article or artifact
    pub note: Option<Vec<Annotation>>,
}

/// The defined version of the cited artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactVersion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The version number or other version identifier
    pub value: String,

    /// Citation for the main version of the cited artifact
    #[serde(rename = "baseCitation")]
    pub base_citation: Option<Box<Reference>>,
}

/// An effective date or period for a status of the cited artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactStatusDate {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Classification of the status
    pub activity: CodeableConcept,

    /// Either occurred or expected
    pub actual: Option<bool>,

    /// When the status started and/or ended
    pub period: Period,
}

/// The title details of the article or artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactTitle {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of title
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// Used to express the specific language
    pub language: Option<CodeableConcept>,

    /// The title of the article or artifact
    pub text: String,
}

/// Summary of the article or artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactAbstract {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of abstract
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Used to express the specific language
    pub language: Option<CodeableConcept>,

    /// Abstract content
    pub text: String,

    /// Copyright notice for the abstract
    pub copyright: Option<String>,
}

/// The component of the article or artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPart {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of component
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The specification of the component
    pub value: Option<String>,

    /// The citation for the full article or artifact
    #[serde(rename = "baseCitation")]
    pub base_citation: Option<Box<Reference>>,
}

/// The artifact related to the cited artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactRelatesTo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// How the cited artifact relates to the target artifact
    #[serde(rename = "relationshipType")]
    pub relationship_type: CodeableConcept,

    /// The clasification of the related artifact
    #[serde(rename = "targetClassifier")]
    pub target_classifier: Option<Vec<CodeableConcept>>,

    /// The article or artifact that the cited artifact is related to
    pub target: serde_json::Value,
}

/// If multiple, used to represent alternative forms of the article that are not separate citations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationForm {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The collection the cited article or artifact is published in
    #[serde(rename = "publishedIn")]
    pub published_in: Option<CitationCitedArtifactPublicationFormPublishedIn>,

    /// The specific issue in which the cited article resides
    #[serde(rename = "periodicRelease")]
    pub periodic_release: Option<CitationCitedArtifactPublicationFormPeriodicRelease>,

    /// The date the article was added to the database, or the date the article was released
    #[serde(rename = "articleDate")]
    pub article_date: Option<String>,

    /// The date the article was last revised or updated in the database
    #[serde(rename = "lastRevisionDate")]
    pub last_revision_date: Option<String>,

    /// Language in which this form of the article is published
    pub language: Option<Vec<CodeableConcept>>,

    /// Entry number or identifier for inclusion in a database
    #[serde(rename = "accessionNumber")]
    pub accession_number: Option<String>,

    /// Used for full display of pagination
    #[serde(rename = "pageString")]
    pub page_string: Option<String>,

    /// Used for isolated representation of first page
    #[serde(rename = "firstPage")]
    pub first_page: Option<String>,

    /// Used for isolated representation of last page
    #[serde(rename = "lastPage")]
    pub last_page: Option<String>,

    /// Number of pages or screens
    #[serde(rename = "pageCount")]
    pub page_count: Option<String>,

    /// Copyright notice for the full article or artifact
    pub copyright: Option<String>,
}

/// The collection the cited article or artifact is published in
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationFormPublishedIn {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Kind of container (e.g. Periodical, database, or book)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID; Book identifiers include ISBN
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Name of the database or title of the book or journal
    pub title: Option<String>,

    /// Name of the publisher
    pub publisher: Option<Box<Reference>>,

    /// Geographic location of the publisher
    #[serde(rename = "publisherLocation")]
    pub publisher_location: Option<String>,
}

/// The specific issue in which the cited article resides
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationFormPeriodicRelease {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Internet or Print
    #[serde(rename = "citedMedium")]
    pub cited_medium: Option<CodeableConcept>,

    /// Volume number of journal in which the article is published
    pub volume: Option<String>,

    /// Issue, part or supplement of journal in which the article is published
    pub issue: Option<String>,

    /// Defining the date on which the issue of the journal was published
    #[serde(rename = "dateOfPublication")]
    pub date_of_publication:
        Option<CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication>,
}

/// Defining the date on which the issue of the journal was published
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactPublicationFormPeriodicReleaseDateOfPublication {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Date on which the issue of the journal was published
    pub date: Option<String>,

    /// Year on which the issue of the journal was published
    pub year: Option<String>,

    /// Month on which the issue of the journal was published
    pub month: Option<String>,

    /// Day on which the issue of the journal was published
    pub day: Option<String>,

    /// Season on which the issue of the journal was published
    pub season: Option<String>,

    /// Text representation of the date of which the issue of the journal was published
    pub text: Option<String>,
}

/// Used for any URL for the article or artifact cited
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactWebLocation {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Code the reason for different URLs, e.g. abstract and full-text
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The specific URL
    pub url: Option<String>,
}

/// The assignment to an organizing scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactClassification {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The kind of classifier (e.g. publication type, keyword)
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The specific classification value
    pub classifier: Option<Vec<CodeableConcept>>,

    /// Provenance and copyright of classification
    #[serde(rename = "whoClassified")]
    pub who_classified: Option<CitationCitedArtifactClassificationWhoClassified>,
}

/// Provenance and copyright of classification
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactClassificationWhoClassified {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Person who created the classification
    pub person: Option<Box<Reference>>,

    /// Organization who created the classification
    pub organization: Option<Box<Reference>>,

    /// The publisher of the classification, not the publisher of the article or artifact being cited
    pub publisher: Option<Box<Reference>>,

    /// Rights management statement for the classification
    #[serde(rename = "classifierCopyright")]
    pub classifier_copyright: Option<String>,

    /// Acceptable to re-use the classification
    #[serde(rename = "freeToShare")]
    pub free_to_share: Option<bool>,
}

/// Attribution of authors and other contributors
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorship {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Indicates if the list includes all authors and/or contributors
    pub complete: Option<bool>,

    /// An individual entity named in the list
    pub entry: Option<Vec<CitationCitedArtifactContributorshipEntry>>,

    /// Used to record a display of the author/contributor list without separate coding for each list member
    pub summary: Option<Vec<CitationCitedArtifactContributorshipSummary>>,
}

/// An individual entity named in the list
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipEntry {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// A name associated with the person
    pub name: Option<HumanName>,

    /// Initials for forename
    pub initials: Option<String>,

    /// Used for collective or corporate name as an author
    #[serde(rename = "collectiveName")]
    pub collective_name: Option<String>,

    /// Author identifier, eg ORCID
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Organizational affiliation
    #[serde(rename = "affiliationInfo")]
    pub affiliation_info: Option<Vec<CitationCitedArtifactContributorshipEntryAffiliationInfo>>,

    /// Physical mailing address
    pub address: Option<Vec<Address>>,

    /// Email or telephone contact methods for the author or contributor
    pub telecom: Option<Vec<ContactPoint>>,

    /// The specific contribution
    #[serde(rename = "contributionType")]
    pub contribution_type: Option<Vec<CodeableConcept>>,

    /// The role of the contributor (e.g. author, editor, reviewer)
    pub role: Option<CodeableConcept>,

    /// Contributions with accounting for time or number
    #[serde(rename = "contributionInstance")]
    pub contribution_instance:
        Option<Vec<CitationCitedArtifactContributorshipEntryContributionInstance>>,

    /// Indication of which contributor is the corresponding contributor for the role
    #[serde(rename = "correspondingContact")]
    pub corresponding_contact: Option<bool>,

    /// Used to code order of authors
    #[serde(rename = "listOrder")]
    pub list_order: Option<i32>,
}

/// Organizational affiliation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipEntryAffiliationInfo {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Display for the organization
    pub affiliation: Option<String>,

    /// Role within the organization, such as professional title
    pub role: Option<String>,

    /// Identifier for the organization
    pub identifier: Option<Vec<Box<Identifier>>>,
}

/// Contributions with accounting for time or number
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipEntryContributionInstance {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The specific contribution
    #[serde(rename = "type")]
    pub r#type: CodeableConcept,

    /// The time that the contribution was made
    pub time: Option<String>,
}

/// Used to record a display of the author/contributor list without separate coding for each list member
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationCitedArtifactContributorshipSummary {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Either authorList or contributorshipStatement
    #[serde(rename = "type")]
    pub r#type: Option<CodeableConcept>,

    /// The format for the display string
    pub style: Option<CodeableConcept>,

    /// Used to code the producer or rule for creating the display string
    pub source: Option<CodeableConcept>,

    /// The display string for the author list, contributor list, or contributorship statement
    pub value: String,
}

/// The Citation Resource enables reference to any knowledge artifact for purposes of identification and attribution. The Citation Resource supports existing reference structures and developing publica...
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Citation {
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

    /// Canonical identifier for this citation, represented as a globally unique URI
    pub url: Option<String>,

    /// Identifier for the Citation resource itself
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the citation
    pub version: Option<String>,

    /// Name for this citation (computer friendly)
    pub name: Option<String>,

    /// Name for this citation (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// The publisher of the Citation, not the publisher of the article or artifact being cited
    pub publisher: Option<String>,

    /// Contact details for the publisher of the Citation Resource
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the citation
    pub description: Option<String>,

    /// The context that the Citation Resource content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction for citation (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this citation is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions for the Citation, not for the cited artifact
    pub copyright: Option<String>,

    /// When the citation was approved by publisher
    #[serde(rename = "approvalDate")]
    pub approval_date: Option<String>,

    /// When the citation was last reviewed
    #[serde(rename = "lastReviewDate")]
    pub last_review_date: Option<String>,

    /// When the citation is expected to be used
    #[serde(rename = "effectivePeriod")]
    pub effective_period: Option<Period>,

    /// Who authored the Citation
    pub author: Option<Vec<ContactDetail>>,

    /// Who edited the Citation
    pub editor: Option<Vec<ContactDetail>>,

    /// Who reviewed the Citation
    pub reviewer: Option<Vec<ContactDetail>>,

    /// Who endorsed the Citation
    pub endorser: Option<Vec<ContactDetail>>,

    /// A human-readable display of the citation
    pub summary: Option<Vec<CitationSummary>>,

    /// The assignment to an organizing scheme
    pub classification: Option<Vec<CitationClassification>>,

    /// Used for general notes and annotations not coded elsewhere
    pub note: Option<Vec<Annotation>>,

    /// The status of the citation
    #[serde(rename = "currentState")]
    pub current_state: Option<Vec<CodeableConcept>>,

    /// An effective date or period for a status of the citation
    #[serde(rename = "statusDate")]
    pub status_date: Option<Vec<CitationStatusDate>>,

    /// Artifact related to the Citation Resource
    #[serde(rename = "relatesTo")]
    pub relates_to: Option<Vec<CitationRelatesTo>>,

    /// The article or artifact being described
    #[serde(rename = "citedArtifact")]
    pub cited_artifact: Option<CitationCitedArtifact>,
}
