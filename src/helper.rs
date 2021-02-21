pub fn linspace(start: f64, end: f64, size: usize) -> Vec<f64> {
    let dt = (end - start) / (size -1) as f64;
    let mut space: Vec<f64> = Vec::new();
    
    space.resize(size, 0.0);

    for i in 0..size {
        space[i] = start + i as f64 * dt;
    }

    space
}