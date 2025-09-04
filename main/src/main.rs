use activations::{IDENTITY, RELU, SIGMOID, TANH};
use eframe::egui;
use network::Network;
use std::vec;

fn main() {
    handle_gui();
    let file = "log.json".to_string();
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut network = Network::new(vec![2, 3, 1], 0.1, SIGMOID);

    network.train(inputs, targets, 65535);

    println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));

    Network::save(&network, file.clone());
}

async fn handle_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640., 480.]),
        ..Default::default()
    };

    eframe::run_native(
        "NeuralRust",
        options,
        Box::new(|cc| Ok(Box::<NeuralUI>::default())),
    )
}

struct NeuralUI {
    points: u32,
    bridges: u32,
}

impl Default for NeuralUI {
    fn default() -> Self {
        Self {
            points: 0,
            bridges: 0,
        }
    }
}

impl eframe::App for NeuralUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.add(egui::Slider::new(&mut self.bridges, 0..=120).text("age"));
            ui.label(format!("Hello '{}', age {}", self.points, self.bridges));
        });
    }
}
