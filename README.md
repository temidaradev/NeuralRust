# NeuralRust

## Overview

A neural network implementation written in Rust, designed for learning and experimentation with machine learning concepts.

## Requirements

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)

## Installation

Clone the repository:

```sh
git clone https://github.com/temidaradev/NeuralRust.git
cd NeuralRust
```

Or install from release:

There is only aarch64 mac and x86_64 windows executables!
[v0.1](https://github.com/temidaradev/NeuralRust/releases/tag/v0.1)

Build the project:

```sh
cargo build --release
```

## Usage

Run the project and start the training:

```sh
cargo run
```

Or the run current example which is same with in main with:

```sh
cargo run --example xor
```

## How It Works

### Neural Network Architecture

This implementation uses a **feedforward neural network** with the following components:

#### 1. **Network Structure**

```
Input Layer → Hidden Layer(s) → Output Layer
    [2]    →      [4]       →     [1]
```

- **Input Layer**: Receives the data (e.g., XOR inputs like [0,1])
- **Hidden Layer(s)**: Process the information using weights and activation functions
- **Output Layer**: Produces the final prediction

#### 2. **Forward Propagation**

The network processes information in these steps:

1. **Weighted Sum**: Each neuron calculates: `output = Σ(input × weight) + bias`
2. **Activation**: Applies an activation function to introduce non-linearity
3. **Layer by Layer**: Output of one layer becomes input to the next

```rust
// Simplified forward pass
for layer in &self.layers {
    let weighted_sum = inputs * weights + bias;
    let activated = activation_function(weighted_sum);
    inputs = activated; // Pass to next layer
}
```

#### 3. **Backpropagation (Learning)**

The network learns by adjusting weights based on errors:

1. **Calculate Error**: Compare predicted output with expected output
2. **Propagate Backward**: Calculate how much each weight contributed to the error
3. **Update Weights**: Adjust weights to reduce error using gradient descent

```rust
// Simplified backpropagation
let error = expected - predicted;
let gradient = error * derivative_of_activation(output);
weight = weight + (learning_rate * gradient * input);
```

#### 4. **Training Process**

1. **Epoch**: One complete pass through all training data
2. **Batch Processing**: Process training examples one by one
3. **Loss Calculation**: Measure how "wrong" the network is
4. **Weight Updates**: Adjust weights to improve accuracy

### Key Concepts Implemented

#### **Activation Functions**

Transform the weighted sum to introduce non-linearity:

- **ReLU**: `f(x) = max(0, x)` - Simple and effective, prevents vanishing gradients
- **Sigmoid**: `f(x) = 1/(1 + e^(-x))` - Smooth, outputs between 0 and 1
- **Tanh**: `f(x) = tanh(x)` - Outputs between -1 and 1, zero-centered
- **Identity**: `f(x) = x` - Linear activation, mainly for output layers

#### **Learning Rate**

Controls how big steps the network takes when updating weights:

- **Too High**: Network might overshoot the optimal solution
- **Too Low**: Network learns very slowly
- **Just Right**: Smooth, steady learning

#### **Loss Function**

Measures the difference between predicted and expected outputs:

```rust
// Mean Squared Error (commonly used)
loss = (predicted - expected)² / 2
```

### Why XOR is a Perfect Test Case

XOR (Exclusive OR) is the "Hello World" of neural networks because:

1. **Non-Linear Problem**: Cannot be solved with a single layer (linear separation)
2. **Simple to Understand**: Only 4 possible inputs with clear expected outputs
3. **Requires Hidden Layers**: Forces the network to learn complex patterns

```
XOR Truth Table:
Input A | Input B | Output
   0    |    0    |   0     ← Same inputs = 0
   0    |    1    |   1     ← Different inputs = 1
   1    |    0    |   1     ← Different inputs = 1
   1    |    1    |   0     ← Same inputs = 0
```

### What You See When Training

As shown in the video, during training you'll observe:

1. **Decreasing Loss**: Network gets better at predictions over time
2. **Epoch Progress**: Shows learning iterations
3. **Final Accuracy**: How well the network learned the XOR function

The magic happens when loss decreases from ~0.5 (random guessing) to ~0.001 (nearly perfect)!

## Future of this project

Im thinking of:

- Making this more "modular" like running different examples in different place
- Visualizing
- More flexible and usable

## Activation Functions:

- Identity
- ReLU (Rectified Linear Unit)
- Sigmoid
- Tanh

## What happpens when you run:

https://github.com/user-attachments/assets/10b933af-fecb-43c7-94fb-c62a36fc4d7d

Made with ❤️ and 🦀 by [temidaradev](https://temidara.dev)
