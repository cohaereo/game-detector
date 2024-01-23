use game_detector::InstalledGame;

fn main() {
    env_logger::init();

    let games = game_detector::find_all_games();
    for g in games {
        match g {
            InstalledGame::Steam(a) => {
                println!("{} / {} (Steam) -> {}", a.name, a.appid, a.game_path)
            }
            InstalledGame::EpicGames(m) => {
                println!("{} (Epic Games) -> {}", m.display_name, m.install_location)
            }
            InstalledGame::MicrosoftStore(p) => {
                println!("{} (Microsoft Store) -> {}", p.app_name, p.path)
            }
            _ => eprintln!("Unhandled store {}", g.store_name()),
        }
    }
}
