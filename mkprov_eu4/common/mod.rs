use clap::ValueEnum;

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum ItemKind {
    /// Aliases = p, prov
    #[clap(aliases(["p", "prov"]))]
    Province,
    /// Aliases = a
    #[clap(alias("a"))]
    Area,
    /// Aliases = c
    #[clap(alias("c"))]
    Country,
}
