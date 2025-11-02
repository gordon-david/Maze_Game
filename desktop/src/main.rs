use eframe::{App, egui};
use game_core::GameState;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Maze Game",
        options,
        Box::new(|_cc| Ok(Box::new(MazeApp::default()))),
    )
}

/// Actions that can be triggered by UI interactions.
/// Used to collect user intent during UI rendering before applying
/// changes to game state, avoiding borrow checker conflicts.
#[derive(Debug)]
enum GameAction {
    /// Start a new game
    Restart,
    /// Choose an exit at the given index
    ChooseExit(usize),
}

struct MazeApp {
    state: GameState,
}

impl Default for MazeApp {
    fn default() -> Self {
        // Try to load maze.json from the executable's directory
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|p| p.to_owned()));

        let state = if let Some(dir) = exe_dir {
            let maze_path = dir.join("maze.json");
            if maze_path.exists() {
                match GameState::load_from_file(&maze_path) {
                    Ok(state) => state,
                    Err(e) => {
                        eprintln!("Error loading maze.json: {}. Using default maze.", e);
                        GameState::new()
                    }
                }
            } else {
                GameState::new()
            }
        } else {
            GameState::new()
        };

        Self { state }
    }
}

impl MazeApp {
    /// Render the game UI and collect any user actions.
    /// This function only reads state, never modifies it.
    fn render_ui(&self, ctx: &egui::Context) -> Option<GameAction> {
        let mut action = None;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🧱 Maze Game");
            ui.separator();

            let room = self.state.current_room();
            ui.label(room.description.clone());
            ui.add_space(20.0);

            if room.is_end {
                ui.label("🎉 You reached the end of the maze!");
                if ui.button("Restart").clicked() {
                    action = Some(GameAction::Restart);
                }
            } else {
                for (i, exit) in room.exits.iter().enumerate() {
                    if ui.button(exit.label.clone()).clicked() {
                        action = Some(GameAction::ChooseExit(i));
                    }
                }
            }
        });

        action
    }

    /// Update game state based on user actions.
    /// Only called when there are actions to process.
    fn update_state(&mut self, action: GameAction) {
        match action {
            GameAction::Restart => self.state = GameState::new(),
            GameAction::ChooseExit(i) => self.state.choose_exit(i),
        }
    }
}

impl App for MazeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // First collect any actions using only immutable access
        let action = self.render_ui(ctx);

        // Then update state if we have an action
        if let Some(_action) = action {
            self.update_state(_action);
        }
    }
}
