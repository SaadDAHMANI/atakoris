/// NetworkDto is Network data transfer object.
///
use super::junctiondto::JunctionDto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkDto {
    pub title: String,
    pub junctions: Vec<JunctionDto>,
    // pub tanks : Option<Vec<Tank>>,
    //  pub reservoirs : Option<Vec<Reservoir>>,
    //  pub pipes : Option<Vec<Pipe>>,
    //  pub pumps : Option<Vec<Pump>>,
    // pub valves : Option<Vec<Valve>>,
    // pub options : Option<Options>,
}
