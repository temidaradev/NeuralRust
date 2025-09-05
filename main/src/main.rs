use activations::{IDENTITY, RELU, SIGMOID, TANH};
use eframe::{egui, CreationContext};
use egui::Context;
use egui_graphs::{DefaultGraphView, Graph, GraphView, LayoutForceDirected, SettingsStyle};
use network::Network;
use petgraph::{stable_graph::StableGraph, Undirected};
use std::vec;

mod graph;

fn main() {
    handle_gui().unwrap();
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
fn handle_gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640., 480.]),
        ..Default::default()
    };

    eframe::run_native(
        "NeuralUI",
        options,
        Box::new(|cc| Ok(Box::new(NeuralUI::new(cc)))),
    )
}

struct NeuralUI {
    g: Graph<(), (), Undirected>,
}

impl NeuralUI {
    fn new(_: &CreationContext<'_>) -> Self {
        let g = generate_graph();
        Self { g: Graph::from(&g) }
    }
}

pub fn generate_graph() -> StableGraph<(), (), Undirected> {
    let mut g = StableGraph::<_, _, Undirected>::default();

    let a = g.add_node(());
    let b = g.add_node(());
    let c = g.add_node(());

    g.add_edge(a, b, ());
    g.add_edge(b, c, ());
    g.add_edge(c, a, ());

    g
}

impl eframe::App for NeuralUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut GraphView::<_, _, _>::new(&mut self.g));
            ui.heading("My egui Application");
        });
    }
}
