use std::path::PathBuf;

use chrono::FixedOffset;
use saleor_app_sdk::manifest::AppManifest;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SaleorApp {
    pub name: String,
    pub slug: String,
    pub authors: Vec<AppAuthor>,
    pub description: String,
    pub versions: Vec<Version>,
    pub curr_version: Version,
    /**
    All files will target ./public/app_data/<app-id>/
    */
    pub logo: PathBuf,
    pub images: Vec<PathBuf>,
    pub manifest: AppManifest,
    /**
    This will be a tag, which shows if I verified this app. Adding apps will be allowed publicly and anonymously.
    This will show a disclaimer about quality and security.
    */
    pub is_verified: bool,
    pub supported_deployments: Vec<DeploymentType>,
    /**
    Service description will be read from this via openGraph.
    */
    pub built_for_url: Option<String>,
    /**
    Will be either a .yml file or a static url to one. Not sure how to store this yet.
    */
    pub minimal_docker_compose: String,
    pub supported_apls: Vec<CustomAplType>,
    pub last_updated: chrono::DateTime<FixedOffset>,
    pub categories: Vec<AppCategory>,
}

#[derive(
    Debug, Serialize, PartialEq, Deserialize, Clone, strum::EnumIter, strum::IntoStaticStr,
)]
pub enum DeploymentType {
    Docker,
    Podman,
    Linux,
    Windows,
    MacOS,
    WASM,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppAuthor {
    name: String,
    links: Vec<String>,
    apps: Vec<SaleorApp>,
}

#[derive(Debug, Serialize, PartialEq, Deserialize, Clone)]
pub struct CustomAplType(String);

#[derive(
    Debug, Serialize, PartialEq, Deserialize, Clone, strum::EnumIter, strum::IntoStaticStr,
)]
pub enum AppCategory {
    CMS,
    Messaging,
    Taxes,
    Payments,
    CRM,
    Monitoring,
    Marketplaces,
    Search,
    SEO,
    DashboardUtilities,
    Other,
}
