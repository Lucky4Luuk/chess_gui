mod engine_player;
mod board_vis;

use engine_player::*;
use board_vis::*;

fn show_player_engine_picker(ui: &mut egui::Ui) -> Option<EngineOrPlayer> {
    let mut choice = None;
    ui.vertical_centered_justified(|ui| {
        if ui.button("Player").clicked() {
            choice = Some(EngineOrPlayer::Player);
        }
        if ui.button("Engine").clicked() {
            choice = Some(EngineOrPlayer::Engine(Engine::empty()));
        }
    });
    choice
}

fn show_engine_choice(ui: &mut egui::Ui, path: &mut String) -> bool {
    ui.add(egui::TextEdit::singleline(path).hint_text("/path/to/bot.exe"));
    if ui.button("Choose").clicked() {
        if !path.is_empty() {
            return true;
        }
    }
    false
}

fn show_side_ui(ui: &mut egui::Ui, side: &mut EngineOrPlayer, is_thinking: bool) {
    match side {
        EngineOrPlayer::Player => {}, // TODO: User input
        EngineOrPlayer::Engine(engine) => {
            if engine.locked_in {
                if is_thinking {
                    ui.label("Thinking..."); // TODO: Format nicely
                }
            } else {
                engine.locked_in = show_engine_choice(ui, &mut engine.path);
            }
        }
    }
}

pub struct ChessApp {
    white: Option<EngineOrPlayer>,
    black: Option<EngineOrPlayer>,

    game: chess::Game,
    board_image: Option<egui_extras::image::RetainedImage>,
    board_image_update: bool,

    move_promise: Option<MovePromise>,
}

impl ChessApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Self {
            white: None,
            black: None,

            game: chess::Game::new(),
            board_image: None,
            board_image_update: false,

            move_promise: None,
        }
    }

    pub fn play_move(&mut self, cmove: chess::ChessMove) -> bool {
        println!("cmove: {}", cmove);
        self.board_image_update = true;
        self.game.make_move(cmove) || self.game.result().is_some()
    }

    pub fn parse_move(&mut self, move_string: String) -> Option<chess::ChessMove> {
        let move_string = move_string.to_lowercase();
        if move_string.as_str() == "resign" {
            return None;
        } else {
            // Parse the move
            // Format:
            // a1b2 (a1 to b2)
            // a2a1q (a2 to a1, q = promotion to queen)
            let chars: Vec<char> = move_string.chars().collect();
            if chars.len() < 4 || chars.len() > 5 {
                return None;
            }
            let mut source_str = String::new();
            source_str.push(chars[0]);
            source_str.push(chars[1]);
            let mut dest_str = String::new();
            dest_str.push(chars[2]);
            dest_str.push(chars[3]);
            let promo_c = chars.get(4);

            let source = if let Some(v) = chess::Square::from_string(source_str) { v } else { return None; };
            let dest = if let Some(v) = chess::Square::from_string(dest_str) { v } else { return None; };
            let mut promo = None;
            if let Some(c) = promo_c {
                promo = Some(match c {
                    'q' => chess::Piece::Queen,
                    'r' => chess::Piece::Rook,
                    'k' => chess::Piece::Knight,
                    'b' => chess::Piece::Bishop,
                    _ => return None,
                });
            }
            return Some(chess::ChessMove::new(source, dest, promo));
        }
        None
    }

    pub fn update_move_side(&mut self, side_to_move: chess::Color) -> bool {
        let mut remove_side = false;
        let side = if side_to_move == chess::Color::White { self.white.as_mut().unwrap() } else { self.black.as_mut().unwrap() };
        match side {
            EngineOrPlayer::Engine(engine) => {
                if engine.locked_in == false {
                    return false;
                } else {
                    if let Some(promise) = self.move_promise.take() {
                        match promise.poll_recv() {
                            Ok(result) => {
                                match result {
                                    Ok(move_string) => {
                                        if let Some(cmove) = self.parse_move(move_string.trim().to_string()) {
                                            if !self.play_move(cmove) {
                                                println!("Move was illegal or lead to checkmate!");
                                                remove_side = true;
                                            }
                                        } else {
                                            println!("Bot returned an unparsable move!");
                                            remove_side = true;
                                        }
                                    },
                                    Err(err_string) => {
                                        println!("{}", err_string);
                                        remove_side = true;
                                    }
                                }
                            },
                            Err(promise) => self.move_promise = Some(promise),
                        }
                    } else {
                        self.move_promise = Some(engine.gen_move(&self.game.current_position()));
                    }
                }
            },
            EngineOrPlayer::Player => {},
        }
        remove_side
    }

    pub fn reset_game(&mut self) {
        self.game = chess::Game::new();
        self.board_image_update = true;
    }

    pub fn update_moves(&mut self) {
        if self.white.is_none() || self.black.is_none() { return; }

        let side_to_move = self.game.side_to_move();
        if self.update_move_side(side_to_move) {
            if side_to_move == chess::Color::White {
                self.white = None;
                self.reset_game();
            } else {
                self.black = None;
                self.reset_game();
            }
        }
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_moves();

        egui::SidePanel::left("player_white").show(ctx, |ui| {
            ui.heading("White");
            ui.separator();

            if let Some(white) = &mut self.white {
                let is_thinking = self.game.side_to_move() == chess::Color::White && self.move_promise.is_some();
                show_side_ui(ui, white, is_thinking);
            } else {
                self.white = show_player_engine_picker(ui);
            }
        });
        egui::SidePanel::right("player_black").show(ctx, |ui| {
            ui.heading("Black");
            ui.separator();

            if let Some(black) = &mut self.black {
                let is_thinking = self.game.side_to_move() == chess::Color::Black && self.move_promise.is_some();
                show_side_ui(ui, black, is_thinking);
            } else {
                self.black = show_player_engine_picker(ui);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.board_image.is_none() || self.board_image_update {
                self.board_image_update = false;
                let bytes = board_to_image(&self.game.current_position());
                self.board_image = Some(egui_extras::image::RetainedImage::from_image_bytes(
                    "board_img",
                    &bytes[..]
                ).unwrap());
            }
            if let Some(img) = &self.board_image {
                let available_size = ui.available_size();
                let size = egui::Vec2::splat(available_size.min_elem());
                if available_size.min_elem() == available_size.x {
                    ui.horizontal_centered(|ui| {
                        img.show_size(ui, size);
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        img.show_size(ui, size);
                    });
                }
            }
        });
        ctx.request_repaint();
    }
}

fn main() {
    println!("Hello, world!");

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Chess Bot GUI",
        native_options,
        Box::new(|cc| Box::new(ChessApp::new(cc))),
    );
}
