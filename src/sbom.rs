use serde::{Deserialize, Serialize};
#[serde(rename = "bom")]
#[derive(Serialize, Deserialize, Debug)]
pub struct SBom {
    #[serde(rename = "@serialNumber")]
    pub serial_number: Option<String>, // Nullable
    #[serde(rename = "@version")]
    pub version: Option<String>, // Nullable
    pub metadata: Option<Metadata>, // Nullable
    pub components: Option<Components>, // Nullable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub timestamp: Option<String>, // Nullable
    pub tools: Option<Tools>, // Nullable
    pub component: Option<Component>, // Main component (Nullable)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tools {
    pub tool: Option<Tool>, // Nullable
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tool {
    pub vendor: Option<String>, // Nullable
    pub name: Option<String>, // Nullable
    pub version: Option<String>, // Nullable
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Component {
    #[serde(rename = "@type")]
    pub component_type: Option<String>, // Nullable
    #[serde(rename = "@bom-ref")]
    pub bom_ref: Option<String>, // Nullable
    pub name: Option<String>, // Nullable
    pub version: Option<String>, // Nullable
    pub description: Option<CData>, // Nullable
    pub licenses: Option<Licenses>, // Nullable
    pub purl: Option<String>, // Nullable
    #[serde(rename = "externalReferences")]
    pub external_references: Option<ExternalReferences>, // Nullable
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct CData {
    #[serde(rename = "$value")]
    pub value: Option<String>, // Nullable
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Licenses {
    pub license: Option<License>, // Nullable
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct License {
    pub id: Option<String>, // Nullable
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalReferences {
    #[serde(rename = "reference")]
    pub reference: Option<Vec<Reference>>, // Nullable
}

#[derive(Serialize, Deserialize, Debug,Clone )]
pub struct Reference {
    #[serde(rename = "@type")]
    pub reference_type: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct Components {
    pub component: Option<Vec<Component>>, // Nullable
}
