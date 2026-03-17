//! FHIR R5 TestPlan Resource
//!
//! Auto-generated from FHIR schema definitions.
//! Do not modify directly.

use crate::r5::types::*;
use serde::{Deserialize, Serialize};

/// The required criteria to execute the test plan - e.g. preconditions, previous tests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanDependency {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of the dependency criterium
    pub description: Option<String>,

    /// Link to predecessor test plans
    pub predecessor: Option<Box<Reference>>,
}

/// The test cases that constitute this plan
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCase {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Sequence of test case in the test plan
    pub sequence: Option<i32>,

    /// The scope or artifact covered by the case
    pub scope: Option<Vec<Box<Reference>>>,

    /// Required criteria to execute the test case
    pub dependency: Option<Vec<TestPlanTestCaseDependency>>,

    /// The actual test to be executed
    #[serde(rename = "testRun")]
    pub test_run: Option<Vec<TestPlanTestCaseTestRun>>,

    /// The test data used in the test case
    #[serde(rename = "testData")]
    pub test_data: Option<Vec<TestPlanTestCaseTestData>>,

    /// Test assertions or expectations
    pub assertion: Option<Vec<TestPlanTestCaseAssertion>>,
}

/// Required criteria to execute the test case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseDependency {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Description of the criteria
    pub description: Option<String>,

    /// Link to predecessor test plans
    pub predecessor: Option<Box<Reference>>,
}

/// The actual test to be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestRun {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The narrative description of the tests
    pub narrative: Option<String>,

    /// The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript
    pub script: Option<TestPlanTestCaseTestRunScript>,
}

/// The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestRunScript {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The language for the test cases e.g. 'gherkin', 'testscript'
    pub language: Option<CodeableConcept>,

    /// The actual content of the cases - references to TestScripts or externally defined content
    pub source: Option<serde_json::Value>,
}

/// The test data used in the test case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseTestData {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// The type of test data description, e.g. 'synthea'
    #[serde(rename = "type")]
    pub r#type: Coding,

    /// The actual test resources when they exist
    pub content: Option<Box<Reference>>,

    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc
    pub source: Option<serde_json::Value>,
}

/// Test assertions or expectations
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlanTestCaseAssertion {
    /// Unique id for inter-element referencing
    pub id: Option<String>,

    /// Additional content defined by implementations
    pub extension: Option<Vec<Extension>>,

    /// Extensions that cannot be ignored even if unrecognized
    #[serde(rename = "modifierExtension")]
    pub modifier_extension: Option<Vec<Extension>>,

    /// Assertion type - for example 'informative' or 'required'
    #[serde(rename = "type")]
    pub r#type: Option<Vec<CodeableConcept>>,

    /// The focus or object of the assertion
    pub object: Option<Vec<CodeableReference>>,

    /// The actual result assertion
    pub result: Option<Vec<CodeableReference>>,
}

/// A plan for executing testing on an artifact or specifications
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestPlan {
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

    /// Canonical identifier for this test plan, represented as a URI (globally unique)
    pub url: Option<String>,

    /// Business identifier identifier for the test plan
    pub identifier: Option<Vec<Box<Identifier>>>,

    /// Business version of the test plan
    pub version: Option<String>,

    /// How to compare versions
    #[serde(rename = "versionAlgorithm")]
    pub version_algorithm: Option<serde_json::Value>,

    /// Name for this test plan (computer friendly)
    pub name: Option<String>,

    /// Name for this test plan (human friendly)
    pub title: Option<String>,

    /// draft | active | retired | unknown
    pub status: String,

    /// For testing purposes, not real usage
    pub experimental: Option<bool>,

    /// Date last changed
    pub date: Option<String>,

    /// Name of the publisher/steward (organization or individual)
    pub publisher: Option<String>,

    /// Contact details for the publisher
    pub contact: Option<Vec<ContactDetail>>,

    /// Natural language description of the test plan
    pub description: Option<String>,

    /// The context that the content is intended to support
    #[serde(rename = "useContext")]
    pub use_context: Option<Vec<UsageContext>>,

    /// Intended jurisdiction where the test plan applies (if applicable)
    pub jurisdiction: Option<Vec<CodeableConcept>>,

    /// Why this test plan is defined
    pub purpose: Option<String>,

    /// Use and/or publishing restrictions
    pub copyright: Option<String>,

    /// Copyright holder and year(s)
    #[serde(rename = "copyrightLabel")]
    pub copyright_label: Option<String>,

    /// The category of the Test Plan - can be acceptance, unit, performance
    pub category: Option<Vec<CodeableConcept>>,

    /// What is being tested with this Test Plan - a conformance resource, or narrative criteria, or an external reference
    pub scope: Option<Vec<Box<Reference>>>,

    /// A description of test tools to be used in the test plan - narrative for now
    #[serde(rename = "testTools")]
    pub test_tools: Option<String>,

    /// The required criteria to execute the test plan - e.g. preconditions, previous tests
    pub dependency: Option<Vec<TestPlanDependency>>,

    /// The threshold or criteria for the test plan to be considered successfully executed - narrative
    #[serde(rename = "exitCriteria")]
    pub exit_criteria: Option<String>,

    /// The test cases that constitute this plan
    #[serde(rename = "testCase")]
    pub test_case: Option<Vec<TestPlanTestCase>>,
}
