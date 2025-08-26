#[derive(Debug)]
pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    bias: Vec<Matrix>,
    data: Vec<Matrix>
}

impl Network {
    pub fn new(layers: Vec<usize>){}
}
