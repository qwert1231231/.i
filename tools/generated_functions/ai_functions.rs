//! Ai Functions for i Language
//! Generated automatically - 152 functions

use std::collections::HashMap;
use std::f64::consts::PI;

// Utility functions
fn factorial(n: f64) -> f64 {
    if n <= 1.0 { 1.0 } else { n * factorial(n - 1.0) }
}


// NEURAL network with 1 layers and 1024 neurons
fn ai_neural_network_1l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 2 layers and 256 neurons
fn ai_neural_network_2l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 3 layers and 1024 neurons
fn ai_neural_network_3l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 4 layers and 64 neurons
fn ai_neural_network_4l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 5 layers and 256 neurons
fn ai_neural_network_5l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 6 layers and 128 neurons
fn ai_neural_network_6l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 7 layers and 256 neurons
fn ai_neural_network_7l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 8 layers and 1024 neurons
fn ai_neural_network_8l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 9 layers and 64 neurons
fn ai_neural_network_9l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 10 layers and 512 neurons
fn ai_neural_network_10l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 11 layers and 64 neurons
fn ai_neural_network_11l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 12 layers and 512 neurons
fn ai_neural_network_12l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 13 layers and 256 neurons
fn ai_neural_network_13l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 14 layers and 256 neurons
fn ai_neural_network_14l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 15 layers and 512 neurons
fn ai_neural_network_15l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 16 layers and 1024 neurons
fn ai_neural_network_16l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 17 layers and 32 neurons
fn ai_neural_network_17l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 18 layers and 32 neurons
fn ai_neural_network_18l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// NEURAL network with 19 layers and 128 neurons
fn ai_neural_network_19l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // neural activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 1 layers and 256 neurons
fn ai_cnn_network_1l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 2 layers and 128 neurons
fn ai_cnn_network_2l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 3 layers and 512 neurons
fn ai_cnn_network_3l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 4 layers and 64 neurons
fn ai_cnn_network_4l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 5 layers and 32 neurons
fn ai_cnn_network_5l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 6 layers and 32 neurons
fn ai_cnn_network_6l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 7 layers and 512 neurons
fn ai_cnn_network_7l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 8 layers and 32 neurons
fn ai_cnn_network_8l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 9 layers and 128 neurons
fn ai_cnn_network_9l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 10 layers and 64 neurons
fn ai_cnn_network_10l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 11 layers and 64 neurons
fn ai_cnn_network_11l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 12 layers and 512 neurons
fn ai_cnn_network_12l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 13 layers and 256 neurons
fn ai_cnn_network_13l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 14 layers and 64 neurons
fn ai_cnn_network_14l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 15 layers and 128 neurons
fn ai_cnn_network_15l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 16 layers and 128 neurons
fn ai_cnn_network_16l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 17 layers and 512 neurons
fn ai_cnn_network_17l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 18 layers and 64 neurons
fn ai_cnn_network_18l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// CNN network with 19 layers and 64 neurons
fn ai_cnn_network_19l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // cnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 1 layers and 1024 neurons
fn ai_rnn_network_1l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 2 layers and 1024 neurons
fn ai_rnn_network_2l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 3 layers and 64 neurons
fn ai_rnn_network_3l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 4 layers and 256 neurons
fn ai_rnn_network_4l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 5 layers and 512 neurons
fn ai_rnn_network_5l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 6 layers and 256 neurons
fn ai_rnn_network_6l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 7 layers and 256 neurons
fn ai_rnn_network_7l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 8 layers and 64 neurons
fn ai_rnn_network_8l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 9 layers and 128 neurons
fn ai_rnn_network_9l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 10 layers and 64 neurons
fn ai_rnn_network_10l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 11 layers and 512 neurons
fn ai_rnn_network_11l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 12 layers and 256 neurons
fn ai_rnn_network_12l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 13 layers and 128 neurons
fn ai_rnn_network_13l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 14 layers and 1024 neurons
fn ai_rnn_network_14l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 15 layers and 32 neurons
fn ai_rnn_network_15l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 16 layers and 256 neurons
fn ai_rnn_network_16l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 17 layers and 64 neurons
fn ai_rnn_network_17l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 18 layers and 256 neurons
fn ai_rnn_network_18l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// RNN network with 19 layers and 1024 neurons
fn ai_rnn_network_19l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // rnn activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 1 layers and 256 neurons
fn ai_lstm_network_1l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 2 layers and 512 neurons
fn ai_lstm_network_2l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 3 layers and 32 neurons
fn ai_lstm_network_3l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 4 layers and 32 neurons
fn ai_lstm_network_4l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 5 layers and 128 neurons
fn ai_lstm_network_5l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 6 layers and 512 neurons
fn ai_lstm_network_6l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 7 layers and 32 neurons
fn ai_lstm_network_7l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 8 layers and 1024 neurons
fn ai_lstm_network_8l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 9 layers and 32 neurons
fn ai_lstm_network_9l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 10 layers and 64 neurons
fn ai_lstm_network_10l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 11 layers and 256 neurons
fn ai_lstm_network_11l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 12 layers and 256 neurons
fn ai_lstm_network_12l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 13 layers and 32 neurons
fn ai_lstm_network_13l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 14 layers and 128 neurons
fn ai_lstm_network_14l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 15 layers and 1024 neurons
fn ai_lstm_network_15l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 16 layers and 32 neurons
fn ai_lstm_network_16l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 17 layers and 32 neurons
fn ai_lstm_network_17l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 18 layers and 512 neurons
fn ai_lstm_network_18l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// LSTM network with 19 layers and 32 neurons
fn ai_lstm_network_19l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // lstm activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 1 layers and 128 neurons
fn ai_transformer_network_1l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 2 layers and 64 neurons
fn ai_transformer_network_2l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 3 layers and 64 neurons
fn ai_transformer_network_3l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 4 layers and 256 neurons
fn ai_transformer_network_4l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 5 layers and 512 neurons
fn ai_transformer_network_5l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 6 layers and 32 neurons
fn ai_transformer_network_6l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 7 layers and 1024 neurons
fn ai_transformer_network_7l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 8 layers and 128 neurons
fn ai_transformer_network_8l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 9 layers and 64 neurons
fn ai_transformer_network_9l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 10 layers and 64 neurons
fn ai_transformer_network_10l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 11 layers and 256 neurons
fn ai_transformer_network_11l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 12 layers and 256 neurons
fn ai_transformer_network_12l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 13 layers and 256 neurons
fn ai_transformer_network_13l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 14 layers and 128 neurons
fn ai_transformer_network_14l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 15 layers and 32 neurons
fn ai_transformer_network_15l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 16 layers and 1024 neurons
fn ai_transformer_network_16l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 17 layers and 256 neurons
fn ai_transformer_network_17l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 18 layers and 128 neurons
fn ai_transformer_network_18l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// TRANSFORMER network with 19 layers and 512 neurons
fn ai_transformer_network_19l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // transformer activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 1 layers and 256 neurons
fn ai_gan_network_1l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 2 layers and 128 neurons
fn ai_gan_network_2l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 3 layers and 128 neurons
fn ai_gan_network_3l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 4 layers and 32 neurons
fn ai_gan_network_4l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 5 layers and 256 neurons
fn ai_gan_network_5l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 6 layers and 32 neurons
fn ai_gan_network_6l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 7 layers and 512 neurons
fn ai_gan_network_7l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 8 layers and 256 neurons
fn ai_gan_network_8l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 9 layers and 1024 neurons
fn ai_gan_network_9l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 10 layers and 512 neurons
fn ai_gan_network_10l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 11 layers and 512 neurons
fn ai_gan_network_11l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 12 layers and 256 neurons
fn ai_gan_network_12l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 13 layers and 256 neurons
fn ai_gan_network_13l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 14 layers and 128 neurons
fn ai_gan_network_14l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 15 layers and 64 neurons
fn ai_gan_network_15l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 16 layers and 128 neurons
fn ai_gan_network_16l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 17 layers and 256 neurons
fn ai_gan_network_17l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 18 layers and 128 neurons
fn ai_gan_network_18l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// GAN network with 19 layers and 32 neurons
fn ai_gan_network_19l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // gan activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 1 layers and 32 neurons
fn ai_vae_network_1l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 2 layers and 1024 neurons
fn ai_vae_network_2l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 3 layers and 64 neurons
fn ai_vae_network_3l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 4 layers and 128 neurons
fn ai_vae_network_4l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 5 layers and 256 neurons
fn ai_vae_network_5l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 6 layers and 256 neurons
fn ai_vae_network_6l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 7 layers and 512 neurons
fn ai_vae_network_7l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 8 layers and 128 neurons
fn ai_vae_network_8l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 9 layers and 1024 neurons
fn ai_vae_network_9l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 10 layers and 256 neurons
fn ai_vae_network_10l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 11 layers and 64 neurons
fn ai_vae_network_11l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 12 layers and 32 neurons
fn ai_vae_network_12l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 13 layers and 512 neurons
fn ai_vae_network_13l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 14 layers and 128 neurons
fn ai_vae_network_14l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 15 layers and 512 neurons
fn ai_vae_network_15l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 16 layers and 64 neurons
fn ai_vae_network_16l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 17 layers and 512 neurons
fn ai_vae_network_17l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 18 layers and 512 neurons
fn ai_vae_network_18l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// VAE network with 19 layers and 512 neurons
fn ai_vae_network_19l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // vae activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 1 layers and 512 neurons
fn ai_attention_network_1l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..1 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 2 layers and 256 neurons
fn ai_attention_network_2l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..2 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 3 layers and 512 neurons
fn ai_attention_network_3l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..3 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 4 layers and 64 neurons
fn ai_attention_network_4l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..4 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 5 layers and 32 neurons
fn ai_attention_network_5l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..5 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 6 layers and 256 neurons
fn ai_attention_network_6l_256n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..6 {
        let mut next_layer = vec![0.0; 256];
        for neuron in 0..256 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 7 layers and 1024 neurons
fn ai_attention_network_7l_1024n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..7 {
        let mut next_layer = vec![0.0; 1024];
        for neuron in 0..1024 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 8 layers and 128 neurons
fn ai_attention_network_8l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..8 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 9 layers and 64 neurons
fn ai_attention_network_9l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..9 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 10 layers and 512 neurons
fn ai_attention_network_10l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..10 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 11 layers and 128 neurons
fn ai_attention_network_11l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..11 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 12 layers and 32 neurons
fn ai_attention_network_12l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..12 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 13 layers and 128 neurons
fn ai_attention_network_13l_128n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..13 {
        let mut next_layer = vec![0.0; 128];
        for neuron in 0..128 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 14 layers and 32 neurons
fn ai_attention_network_14l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..14 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 15 layers and 64 neurons
fn ai_attention_network_15l_64n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..15 {
        let mut next_layer = vec![0.0; 64];
        for neuron in 0..64 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 16 layers and 32 neurons
fn ai_attention_network_16l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..16 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 17 layers and 512 neurons
fn ai_attention_network_17l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..17 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 18 layers and 32 neurons
fn ai_attention_network_18l_32n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..18 {
        let mut next_layer = vec![0.0; 32];
        for neuron in 0..32 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}


// ATTENTION network with 19 layers and 512 neurons
fn ai_attention_network_19l_512n(input: &[f64], weights: &[f64]) -> Vec<f64> {
    let mut activations = input.to_vec();
    let mut weight_index = 0;
    
    for layer in 0..19 {
        let mut next_layer = vec![0.0; 512];
        for neuron in 0..512 {
            let mut sum = 0.0;
            for &activation in &activations {
                if weight_index < weights.len() {
                    sum += activation * weights[weight_index];
                    weight_index += 1;
                }
            }
            // attention activation function
            next_layer[neuron] = match layer % 4 {
                0 => sum.tanh(),                    // Tanh
                1 => 1.0 / (1.0 + (-sum).exp()),    // Sigmoid
                2 => sum.max(0.0),                 // ReLU
                _ => sum / (1.0 + sum.abs()),       // Swish
            };
        }
        activations = next_layer;
    }
    
    activations
}

