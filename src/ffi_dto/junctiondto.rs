#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JunctionDto {
    pub id: usize,
    pub elevation: f64,
    pub demand: f64,
    //  pub pattern: Option<usize>,
    //  pub name: Option<String>,
    // pub head: Option<f64>,
    // #[cfg(feature = "optimization")]
    // target_head: Option<f64>,
    // pub pressure: Option<f64>,
}
