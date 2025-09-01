use std::vec;

use lib::network::Network;

use crate::lib::activations::{IDENTITY, RELU, SIGMOID, TANH};

// 0, 0 --> 0
// 0, 1 --> 1
// 1, 0 --> 1
// 1, 1 --> 0

pub mod lib;
fn main() {
    let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ];

    let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];
    let mut network = Network::new(vec![2, 3, 1], 10.00, SIGMOID);

    network.train(inputs, targets, 65535);

    println!("0 and 0: {:?}", network.feed_forward(vec![0.0, 0.0]));
    println!("0 and 1: {:?}", network.feed_forward(vec![0.0, 1.0]));
    println!("1 and 0: {:?}", network.feed_forward(vec![1.0, 0.0]));
    println!("1 and 1: {:?}", network.feed_forward(vec![1.0, 1.0]));
}
