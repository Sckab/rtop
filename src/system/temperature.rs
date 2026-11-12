#[derive(Debug, Default)]
pub struct Temperature {
    pub sensor: String,
    pub temperature: Option<f32>,
}
