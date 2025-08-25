use game_detector::InstalledGame;

fn main() {
    env_logger::init();

    let games = game_detector::find_all_games();
    for g in games {
        match g {
            #[cfg(feature = "steam")]
            InstalledGame::Steam(a) => {
                println!("{} / {} (Steam) -> {}", a.name, a.appid, a.game_path)
            }
            #[cfg(feature = "epic_games")]
            InstalledGame::EpicGames(m) => {
                println!("{} (Epic Games) -> {}", m.display_name, m.install_location)
            }
            #[cfg(feature = "ms_store")]
            InstalledGame::MicrosoftStore(p) => {
                println!("{} (Microsoft Store) -> {}", p.app_name, p.path)
            }
            _ => eprintln!("Unhandled store {}", g.store_name()),
        }
    }
}
