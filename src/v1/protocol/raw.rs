use std::collections::HashMap;
use serde::{Serialize, Deserialize};
pub use serde_json::Value;

pub type ServerProperties = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHello {
  pub name: String,
  pub description: String,
  pub protocol: String,
  pub root: String,
  pub property: ServerProperties,
  pub services: Services,
  pub plugins: Plugins,
  pub ventoy: VentoyResource,
  pub kernel: FileResource,
  pub hub: HubResource,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Service {
  pub name: String,
  pub path: String,
}

pub type Services = Vec<Service>;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Plugins {
  pub tree: PluginTree,
  pub path: String,
}

pub type PluginTree = HashMap<String, Vec<PluginResource>>;

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct PluginResource {
  pub name: String,
  pub size: usize,
  pub timestamp: usize,
  pub integrity: Integrity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integrity {
  pub method: String,
  pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResource {
  pub version: String,
  pub name: String,
  pub url: String,
  pub timestamp: usize,
  pub size: usize,
  pub integrity: Integrity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverResource {
  pub lower_than: String,
  pub file: FileResource,
}

pub type VentoyResource = HashMap<String, FileResource>;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct HubVersion {
  pub version: String,
  pub page: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubUpdateOptions {
  pub allow_normal_since: String,
  pub force_update_until: String,
  pub wide_gaps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubNotice {
  pub id: String,
  pub channel: String,
  pub level: String,
  pub message: String,
  pub description: String,
  pub close_text: String,
  pub lower_than: String,
  pub repeat_after: isize,
}

pub type HubPackages = HashMap<String, FileResource>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubResource {
  pub latest: HubVersion,
  pub update: HubUpdateOptions,
  pub notices: Vec<HubNotice>,
  pub packages: HubPackages,
}
