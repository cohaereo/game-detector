use fs_err::File;
use log::error;
use serde::Deserialize;
use serde_json::Value;
use std::io::Error;

const MANIFESTS_GLOB: &str = "C:\\ProgramData\\Epic\\EpicGamesLauncher\\Data\\Manifests\\*.item";

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    pub format_version: i64,
    #[serde(rename = "bIsIncompleteInstall")]
    pub is_incomplete_install: bool,
    pub launch_command: String,
    pub launch_executable: String,
    pub manifest_location: String,
    pub manifest_hash: String,
    #[serde(rename = "bIsApplication")]
    pub is_application: bool,
    #[serde(rename = "bIsExecutable")]
    pub is_executable: bool,
    #[serde(rename = "bIsManaged")]
    pub is_managed: bool,
    #[serde(rename = "bNeedsValidation")]
    pub needs_validation: bool,
    #[serde(rename = "bRequiresAuth")]
    pub requires_auth: bool,
    #[serde(rename = "bAllowMultipleInstances")]
    pub allow_multiple_instances: bool,
    #[serde(rename = "bCanRunOffline")]
    pub can_run_offline: bool,
    #[serde(rename = "bAllowUriCmdArgs")]
    pub allow_uri_cmd_args: bool,
    #[serde(rename = "bLaunchElevated")]
    pub launch_elevated: bool,
    #[serde(rename = "BaseURLs")]
    pub base_urls: Vec<String>,
    pub build_label: String,
    pub app_categories: Vec<String>,
    pub chunk_dbs: Vec<Value>,
    pub compatible_apps: Vec<Value>,
    pub display_name: String,
    pub installation_guid: String,
    pub install_location: String,
    pub install_session_id: String,
    pub install_tags: Vec<Value>,
    pub install_components: Vec<Value>,
    pub host_installation_guid: String,
    pub prereq_ids: Vec<Value>,
    #[serde(rename = "PrereqSHA1Hash")]
    pub prereq_sha1hash: String,
    #[serde(rename = "LastPrereqSucceededSHA1Hash")]
    pub last_prereq_succeeded_sha1hash: String,
    pub staging_location: String,
    pub technical_type: String,
    pub vault_thumbnail_url: String,
    pub vault_title_text: String,
    pub install_size: i64,
    pub main_window_process_name: String,
    pub process_names: Vec<Value>,
    pub background_process_names: Vec<Value>,
    pub ignored_process_names: Vec<Value>,
    pub dlc_process_names: Vec<Value>,
    pub mandatory_app_folder_name: String,
    pub ownership_token: String,
    pub catalog_namespace: String,
    pub catalog_item_id: String,
    pub app_name: String,
    pub app_version_string: String,
    pub main_game_catalog_namespace: String,
    pub main_game_catalog_item_id: String,
    pub main_game_app_name: String,
    pub allowed_uri_env_vars: Vec<Value>,
}

pub fn get_all_manifests() -> anyhow::Result<Vec<Manifest>> {
    let mut manifests = vec![];
    for r in glob::glob(MANIFESTS_GLOB)? {
        if let Ok(path) = r {
            match File::open(path) {
                Ok(f) => match serde_json::from_reader::<_, Manifest>(f) {
                    Ok(m) => {
                        manifests.push(m);
                    }
                    Err(e) => error!("Failed to parse manifest: {e}"),
                },
                Err(e) => error!("Failed to open manifest file: {e}"),
            }
        }
    }

    Ok(manifests)
}
