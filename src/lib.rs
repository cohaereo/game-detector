use log::error;

#[cfg(feature = "epic_games")]
pub mod epic_games;
#[cfg(feature = "ms_store")]
pub mod ms_store;
#[cfg(feature = "steam")]
pub mod steam;

#[non_exhaustive]
#[derive(Debug)]
pub enum InstalledGame {
    #[cfg(feature = "steam")]
    Steam(Box<steam::AppState>),
    #[cfg(feature = "epic_games")]
    EpicGames(Box<epic_games::Manifest>),
    #[cfg(feature = "ms_store")]
    MicrosoftStore(Box<ms_store::GamePackage>),
}

impl InstalledGame {
    pub fn store_name(&self) -> &str {
        match self {
            InstalledGame::Steam(_) => "Steam",
            InstalledGame::EpicGames(_) => "Epic Games",
            InstalledGame::MicrosoftStore(_) => "Microsoft Store",
        }
    }
}

pub fn find_all_games() -> Vec<InstalledGame> {
    let mut games = vec![];
    #[cfg(feature = "ms_store")]
    {
        match ms_store::get_game_packages() {
            Ok(packages) => {
                games.extend(
                    packages
                        .into_iter()
                        .map(|p| InstalledGame::MicrosoftStore(Box::new(p))),
                );
            }
            Err(e) => {
                error!("Failed to read Microsoft Store packages: {e}");
            }
        }
    }

    #[cfg(feature = "steam")]
    {
        match steam::get_all_apps() {
            Ok(apps) => {
                games.extend(apps.into_iter().map(|a| InstalledGame::Steam(Box::new(a))));
            }
            Err(e) => {
                error!("Failed to read Steam apps: {e}");
            }
        }
    }

    #[cfg(feature = "epic_games")]
    {
        match epic_games::get_all_manifests() {
            Ok(manifests) => {
                games.extend(
                    manifests
                        .into_iter()
                        .map(|m| InstalledGame::EpicGames(Box::new(m))),
                );
            }
            Err(e) => {
                error!("Failed to read Epic Games manifests: {e}");
            }
        }
    }

    games
}
