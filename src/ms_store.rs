use anyhow::Context;
use log::warn;
use std::path::Path;
use winreg::enums::HKEY_LOCAL_MACHINE;

const GAMING_SERVICES_PACKAGE_REPOSITORY: &str =
    "SOFTWARE\\Microsoft\\GamingServices\\PackageRepository\\Package";
const FULL_PACKAGE_REPOSITORY: &str =
    "SOFTWARE\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\CurrentVersion\\AppModel\\PackageRepository\\Packages";

#[derive(Debug, Clone)]
pub struct GamePackage {
    pub app_id: String,

    pub app_publisher: String,
    pub app_name: String,
    pub app_version: String,
    pub app_arch: String,
    pub app_publisher_id: String,

    pub path: String,
}

/// Returns [publisher, name, version, arch, publisher id]
/// eg. ["Bungie", "Destiny2PCbasegame", "0.3.26071.0", "x64", "8xb1a0vv8ay84"]
fn parse_app_id(s: &str) -> anyhow::Result<[String; 5]> {
    let mut parts = s.split('_');
    let publisher_and_name = parts.next().context("Invalid app id (publisher+name)")?;
    let mut pan_parts = publisher_and_name.split('.');
    let publisher = pan_parts.next().context("Invalid app id (publisher)")?;
    let name = pan_parts.next().context("Invalid app id (name)")?;

    let version = parts.next().context("Invalid app id (version)")?;
    let arch = parts.next().context("Invalid app id (arch)")?;
    parts.next().context("Invalid app id (field4)")?;
    let publisher_id = parts.next().context("Invalid app id (publisher id)")?;
    Ok([
        publisher.to_string(),
        name.to_string(),
        version.to_string(),
        arch.to_string(),
        publisher_id.to_string(),
    ])
}

pub fn get_game_packages() -> anyhow::Result<Vec<GamePackage>> {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let package_repo = hklm.open_subkey(GAMING_SERVICES_PACKAGE_REPOSITORY)?;

    let mut packages = vec![];
    for (app_id, _) in package_repo.enum_values().flatten() {
        let pkg_path = Path::new(FULL_PACKAGE_REPOSITORY).join(&app_id);
        let pkg = hklm.open_subkey(pkg_path)?;
        match pkg.get_value::<String, _>("Path") {
            Ok(path) => match parse_app_id(&app_id) {
                Ok(o) => {
                    let [app_publisher, app_name, app_version, app_arch, app_publisher_id] = o;
                    packages.push(GamePackage {
                        app_id,
                        app_publisher,
                        app_name,
                        app_version,
                        app_arch,
                        app_publisher_id,
                        path,
                    })
                }
                Err(e) => {
                    eprintln!("Couldn't parse package id '{app_id}': {e}");
                }
            },
            Err(e) => {
                warn!("Couldn't read path key for package '{app_id}': {e}");
            }
        }
    }

    Ok(packages)
}
