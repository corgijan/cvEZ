use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CVERecord {
    pub data_type: Option<String>,
    pub data_version: Option<String>,
    pub cve_metadata: Option<CVEMetadata>,
    pub containers: Option<Containers>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CVEMetadata {
    pub cve_id: Option<String>,
    pub assigner_org_id: Option<String>,
    pub assigner_short_name: Option<String>,
    pub requester_user_id: Option<String>,
    pub serial: Option<i32>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Containers {
    pub cna: Option<Cna>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cna {
    pub provider_metadata: Option<ProviderMetadata>,
    pub title: Option<String>,
    pub date_public: Option<String>,
    pub problem_types: Option<Vec<ProblemType>>,
    pub impacts: Option<Vec<Impact>>,
    pub affected: Option<Vec<Affected>>,
    pub descriptions: Option<Vec<Description>>,
    pub metrics: Option<Vec<Metric>>,
    pub solutions: Option<Vec<Solution>>,
    pub workarounds: Option<Vec<Workaround>>,
    pub configurations: Option<Vec<Configuration>>,
    pub exploits: Option<Vec<Exploit>>,
    pub timeline: Option<Vec<TimelineEntry>>,
    pub credits: Option<Vec<Credit>>,
    pub references: Option<Vec<Reference>>,
    pub source: Option<Source>,
    pub taxonomy_mappings: Option<Vec<TaxonomyMapping>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProviderMetadata {
    pub org_id: Option<String>,
    pub short_name: Option<String>,
    pub date_updated: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProblemType {
    pub descriptions: Option<Vec<DescriptionType>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescriptionType {
    pub lang: Option<String>,
    pub cwe_id: Option<String>,
    pub description: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Impact {
    pub capec_id: Option<String>,
    pub descriptions: Option<Vec<DescriptionValue>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DescriptionValue {
    pub lang: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Affected {
    pub vendor: Option<String>,
    pub product: Option<String>,
    pub platforms: Option<Vec<String>>,
    pub collection_url: Option<String>,
    pub package_name: Option<String>,
    pub repo: Option<String>,
    pub modules: Option<Vec<String>>,
    pub program_files: Option<Vec<String>>,
    pub program_routines: Option<Vec<ProgramRoutine>>,
    pub versions: Option<Vec<Version>>,
    pub default_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgramRoutine {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Version {
    pub version: Option<String>,
    pub status: Option<String>,
    pub less_than: Option<String>,
    pub version_type: Option<String>,
    pub changes: Option<Vec<VersionChange>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VersionChange {
    pub at: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Description {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub supporting_media: Option<Vec<SupportingMedia>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupportingMedia {
    pub r#type: Option<String>,
    pub base64: Option<bool>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metric {
    pub format: Option<String>,
    pub scenarios: Option<Vec<Scenario>>,
    pub cvss_v4_0: Option<Cvss40>,
    pub cvss_v3_1: Option<Cvss31>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scenario {
    pub lang: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cvss40 {
    pub base_score: Option<f32>,
    pub base_severity: Option<String>,
    pub vector_string: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cvss31 {
    pub version: Option<String>,
    pub attack_vector: Option<String>,
    pub attack_complexity: Option<String>,
    pub privileges_required: Option<String>,
    pub user_interaction: Option<String>,
    pub scope: Option<String>,
    pub confidentiality_impact: Option<String>,
    pub integrity_impact: Option<String>,
    pub availability_impact: Option<String>,
    pub base_score: Option<f32>,
    pub base_severity: Option<String>,
    pub vector_string: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Solution {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub supporting_media: Option<Vec<SupportingMedia>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Workaround {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub supporting_media: Option<Vec<SupportingMedia>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub supporting_media: Option<Vec<SupportingMedia>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exploit {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub supporting_media: Option<Vec<SupportingMedia>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineEntry {
    pub time: Option<String>,
    pub lang: Option<String>,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credit {
    pub lang: Option<String>,
    pub value: Option<String>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    pub url: Option<String>,
    pub name: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Source {
    pub defects: Option<Vec<String>>,
    pub advisory: Option<String>,
    pub discovery: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaxonomyMapping {
    pub taxonomy_name: Option<String>,
    pub taxonomy_version: Option<String>,
    pub taxonomy_relations: Option<Vec<TaxonomyRelation>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaxonomyRelation {
    pub taxonomy_id: Option<String>,
    pub relationship_name: Option<String>,
    pub relationship_value: Option<String>,
}
