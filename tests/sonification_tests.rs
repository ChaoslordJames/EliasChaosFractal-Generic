#[test]
fn test_sonify() {
    let sonification = crate::rendering::fractal_sonification::FractalSonification::new();
    sonification.sonify(0.5);
    // Audio output simulated via println
}
