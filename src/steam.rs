use fs_err::File;
use log::{error, warn};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename = "libraryfolders")]
pub struct LibraryFolders(pub HashMap<usize, LibraryFolder>);

#[derive(Debug, Clone, Deserialize)]
pub struct LibraryFolder {
    pub path: String,
    pub label: String,
    pub contentid: u64,
    pub totalsize: usize,
    pub update_clean_bytes_tally: usize,
    pub time_last_update_corruption: u64,
    pub apps: HashMap<u32, usize>,
}

// cohae: Yikes Valve.
#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
pub struct AppState {
    pub appid: u64,
    pub universe: u64,
    pub LauncherPath: Option<String>,
    pub name: String,
    pub StateFlags: u64,
    pub installdir: String,
    pub LastUpdated: u64,
    pub SizeOnDisk: usize,
    pub StagingSize: usize,
    pub buildid: u64,
    pub LastOwner: u64,
    pub UpdateResult: Option<u64>,
    pub BytesToDownload: Option<usize>,
    pub BytesDownloaded: Option<usize>,
    pub BytesToStage: Option<usize>,
    pub BytesStaged: Option<usize>,
    pub TargetBuildID: Option<usize>,
    pub AutoUpdateBehavior: u64,
    pub AllowOtherDownloadsWhileRunning: bool,
    pub ScheduledAutoUpdate: u64,
    pub InstalledDepots: HashMap<u64, InstalledDepot>,
    pub SharedDepots: Option<HashMap<u64, u64>>,
    // pub UserConfig: AppConfig,
    // pub MountedConfig: AppConfig,
    /// Base library path (eg. D:/Steam/)
    #[serde(skip)]
    pub library_path: String,

    /// Full game path (eg. D:/Steam/steamapps/common/Team Fortress 2/
    #[serde(skip)]
    pub game_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstalledDepot {
    pub manifest: u64,
    pub size: usize,
}

// #[derive(Debug, Clone, Deserialize)]
// pub struct AppConfig {
//     pub language: String,
// }


#[cfg(not(windows))]
pub fn get_all_apps() -> anyhow::Result<Vec<AppState>> {
    anyhow::bail!("Not supported on this platform")
}

const STEAM_REGKEY_PATH: &str = "SOFTWARE\\Valve\\Steam";

#[cfg(windows)]
pub fn get_all_apps() -> anyhow::Result<Vec<AppState>> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER);
    let steam_key = hkcu.open_subkey(STEAM_REGKEY_PATH)?;
    let steam_path: String = steam_key.get_value("SteamPath")?;
    let vdf_path = Path::new(&steam_path).join("config\\libraryfolders.vdf");

    let mut apps = vec![];
    let folders: LibraryFolders = keyvalues_serde::from_reader(File::open(vdf_path)?)?;
    for f in folders.0.values() {
        let steamapps_path = Path::new(&f.path).join("steamapps");
        for &app_id in f.apps.keys() {
            let appmanifest_path = steamapps_path.join(format!("appmanifest_{app_id}.acf"));
            match File::open(&appmanifest_path).map(keyvalues_serde::from_reader::<_, AppState>) {
                Ok(a) => match a {
                    Ok(mut a) => {
                        a.library_path = f.path.clone();
                        a.game_path = steamapps_path
                            .join("common")
                            .join(&a.installdir)
                            .to_string_lossy()
                            .to_string();
                        apps.push(a);
                    }
                    Err(e) => {
                        error!(
                            "Failed to read appmanifest {}: {e}",
                            appmanifest_path.display()
                        );
                    }
                },
                Err(e) => {
                    // cohae: Sometimes happens after uninstalling an app, so doesn't have to be an error
                    warn!(
                        "Failed to open appmanifest {}: {e}",
                        appmanifest_path.display()
                    );
                }
            }
        }
    }

    Ok(apps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_appstate_serde() {
        const MANIFEST_DATA: &str = r#"
"AppState"
{
	"appid"		"70"
	"universe"		"1"
	"LauncherPath"		"C:\\Program Files (x86)\\Steam\\steam.exe"
	"name"		"Half-Life"
	"StateFlags"		"6"
	"installdir"		"Half-Life"
	"LastUpdated"		"1703587250"
	"SizeOnDisk"		"589449723"
	"StagingSize"		"0"
	"buildid"		"13032868"
	"LastOwner"		"76561198166639473"
	"UpdateResult"		"0"
	"BytesToDownload"		"42478352"
	"BytesDownloaded"		"0"
	"BytesToStage"		"127625842"
	"BytesStaged"		"0"
	"TargetBuildID"		"13032868"
	"AutoUpdateBehavior"		"0"
	"AllowOtherDownloadsWhileRunning"		"0"
	"ScheduledAutoUpdate"		"1706853353"
	"InstalledDepots"
	{
		"1"
		{
			"manifest"		"6665583105370934040"
			"size"		"513399487"
		}
		"3"
		{
			"manifest"		"6081070194444336449"
			"size"		"893958"
		}
		"71"
		{
			"manifest"		"5133329123964362030"
			"size"		"16416909"
		}
		"96"
		{
			"manifest"		"6298465564582633871"
			"size"		"9067684"
		}
		"2"
		{
			"manifest"		"3124227209284380614"
			"size"		"49671685"
		}
	}
	"SharedDepots"
	{
		"228988"		"228980"
	}
	"UserConfig"
	{
		"language"		"english"
	}
	"MountedConfig"
	{
		"language"		"english"
	}
}
"#;

        let app_state: AppState =
            keyvalues_serde::from_str(MANIFEST_DATA).expect("Failed to parse app manifest data");

        assert_eq!(app_state.appid, 70);
        assert_eq!(app_state.universe, 1);
        assert_eq!(
            app_state.LauncherPath,
            Some("C:\\Program Files (x86)\\Steam\\steam.exe".to_string())
        );
        assert_eq!(app_state.name, "Half-Life");
        assert_eq!(app_state.StateFlags, 6);
        assert_eq!(app_state.installdir, "Half-Life");
        assert_eq!(app_state.LastUpdated, 1703587250);
        assert_eq!(app_state.SizeOnDisk, 589449723);
        assert_eq!(app_state.StagingSize, 0);
        assert_eq!(app_state.buildid, 13032868);
        assert_eq!(app_state.LastOwner, 76561198166639473);
        assert_eq!(app_state.UpdateResult, Some(0));
        assert_eq!(app_state.BytesToDownload, Some(42478352));
        assert_eq!(app_state.BytesDownloaded, Some(0));
        assert_eq!(app_state.BytesToStage, Some(127625842));
        assert_eq!(app_state.BytesStaged, Some(0));
        assert_eq!(app_state.TargetBuildID, Some(13032868));
        assert_eq!(app_state.AutoUpdateBehavior, 0);
        assert!(!app_state.AllowOtherDownloadsWhileRunning);
        assert_eq!(app_state.ScheduledAutoUpdate, 1706853353);

        assert_eq!(app_state.InstalledDepots.len(), 5);
        // assert_eq!(app_state.UserConfig.language, "english");
    }

    #[test]
    fn test_libraryfolders_serde() {
        const MANIFEST_DATA: &str = r#"
"libraryfolders"
{
	"0"
	{
		"path"		"C:\\Program Files (x86)\\Steam"
		"label"		""
		"contentid"		"3328371409298419016"
		"totalsize"		"0"
		"update_clean_bytes_tally"		"131786642906"
		"time_last_update_corruption"		"0"
		"apps"
		{
			"228980"		"747619496"
			"250820"		"5464658003"
			"365670"		"1174137444"
			"629730"		"9384044754"
			"992490"		"85670250"
			"1009850"		"79179350"
			"1068820"		"825086515"
			"1826330"		"274110"
		}
	}
	"1"
	{
		"path"		"D:\\Steam"
		"label"		""
		"contentid"		"1039182383252157525"
		"totalsize"		"2000397791232"
		"update_clean_bytes_tally"		"133903725466"
		"time_last_update_corruption"		"0"
		"apps"
		{
			"70"		"589449723"
			"240"		"4628887753"
			"440"		"28179155955"
			"620"		"12753876784"
		}
	}
}
"#;

        let folders: LibraryFolders =
            keyvalues_serde::from_str(MANIFEST_DATA).expect("Failed to parse app manifest data");

        assert_eq!(folders.0.len(), 2);
    }
}
