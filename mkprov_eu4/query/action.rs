use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum ClimateType {
    Tropical,
    Arid,
    Arctic,
    #[default]
    Normal,
}

#[derive(ValueEnum, Clone, Debug, Default)]
pub enum Severity {
    Normal,
    Mild,
    Severe,
    #[default]
    None,
}
