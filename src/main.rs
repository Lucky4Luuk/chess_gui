#[derive(Default)]
pub struct ChessApp {

}

impl ChessApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for ChessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hi");
        });
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
