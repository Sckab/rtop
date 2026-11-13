use crate::system::sorting::TemperatureSort;

#[derive(Debug, Default, Clone)]
pub struct Temperature {
    pub sensor: String,
    pub temperature: Option<f32>,
}

#[derive(Default, Clone)]
pub struct TemperatureState {
    pub sort: TemperatureSort,
    pub temperatures: Vec<Temperature>,
}
