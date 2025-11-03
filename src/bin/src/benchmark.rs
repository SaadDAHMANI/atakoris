pub mod benchmark {
    use atakoris::network::node::junction::{Junction, JunctionBuilder};
    use atakoris::network::node::tank::{Tank, TankBuilder};
    use atakoris::network::{Network, NetworkBuilder};
    //use atakor::network::node::reservoir::{Reservoir, ReservoirBuilder};

    use atakoris::network::link::LinkStatus;
    use atakoris::network::link::pipe::{Pipe, PipeBuilder};
    use atakoris::network::link::pump::{Pump, PumpBuilder};

    use atakoris::network::{FlowUnits, HeadlossFormula, Options, OptionsBuilder};

    #[allow(dead_code)]
    pub fn network3() -> Network {
        let t1: Tank = TankBuilder::new()
            .set_id(0)
            .set_name("T1")
            .set_elevation(100.00)
            .set_initial_level(100.00)
            .build();

        let j1: Junction = JunctionBuilder::new()
            .set_id(1)
            .set_name("J1")
            .set_elevation(0.0)
            .set_demand(0.02)
            .set_pattern(None)
            .build();

        let j2: Junction = JunctionBuilder::new()
            .set_id(2)
            .set_name("J2")
            .set_elevation(0.0f64)
            .set_demand(0.01f64)
            .set_pattern(None)
            .build();

        let p1: Pipe = PipeBuilder::new()
            .set_id(1)
            .set_name("P1")
            .set_start(0)
            .set_end(1)
            .set_length(100.00)
            .set_diameter(0.100)
            .set_roughness(130.0)
            .set_check_valve(false)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .build();

        let p2: Pipe = PipeBuilder::new()
            .set_id(2)
            .set_name("P2")
            .set_start(1)
            .set_end(2)
            .set_length(100.00)
            .set_diameter(0.100)
            .set_roughness(130.0)
            .set_check_valve(false)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .build();

        let p3: Pipe = PipeBuilder::new()
            .set_id(3)
            .set_name("P3")
            .set_start(0)
            .set_end(2)
            .set_length(100.00)
            .set_diameter(0.100)
            .set_roughness(130.0)
            .set_check_valve(false)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .build();

        let ts = vec![t1];
        let js = vec![j1, j2];
        let ps = vec![p1, p2, p3];

        let net3: Network = NetworkBuilder::new()
            .set_title(Some("Network3".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .build();

        net3
    }

    #[allow(dead_code)]
    pub fn network4() -> Network {
        let flow_unit: FlowUnits = FlowUnits::Cms;

        let t1 = Tank::new(0, 50.0, 0.0);
        let t2 = Tank::new(3, 50.0, 0.0);
        let j1 = JunctionBuilder::new()
            .set_id(1)
            .set_name("J1")
            .set_elevation(0.0)
            .set_demand(0.1)
            .set_flow_unit(FlowUnits::Cms)
            .build();

        let j2 = JunctionBuilder::new()
            .set_id(2)
            .set_name("J2")
            .set_elevation(0.0)
            .set_demand(0.1)
            .build();

        let p1 = PipeBuilder::new()
            .set_id(1)
            .set_start(0)
            .set_end(1)
            .set_length(100.0)
            .set_diameter(100.0)
            .set_roughness(130.0)
            .set_flow_unit(flow_unit)
            .build();

        let p2 = PipeBuilder::new()
            .set_id(2)
            .set_start(1)
            .set_end(2)
            .set_length(100.0)
            .set_diameter(100.0)
            .set_roughness(130.0)
            .set_flow_unit(flow_unit)
            .build();

        let p3 = PipeBuilder::new()
            .set_id(3)
            .set_start(0)
            .set_end(2)
            .set_length(100.0)
            .set_diameter(100.0)
            .set_roughness(130.0)
            .set_flow_unit(flow_unit)
            .build();

        let pmp1: Pump = PumpBuilder::new()
            .set_id(4)
            .set_name("Pump1".to_owned())
            .set_start(3)
            .set_end(2)
            .set_alpha(10.0)
            .set_beta(-20.0)
            .set_gamma(50.0)
            .set_status(LinkStatus::Open)
            .build();

        let ts = vec![t1, t2];
        let js = vec![j1, j2];
        let ps = vec![p1, p2, p3];
        let pms = vec![pmp1];

        let net4: Network = NetworkBuilder::new()
            .set_title(Some("Network4".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .set_pumps(Some(pms))
            .build();
        net4
    }

    ///
    ///  Network  (Todini et al., 2021) Pressure Flow-Based Algo for PD analysis of WDN (J. Water Resour.Plann.Manage.)
    ///
    #[allow(dead_code)]
    pub fn network1_todini() -> Network {
        let n1: Tank = TankBuilder::new()
            .set_id(1)
            .set_name("n1-tank")
            .set_elevation(100.00)
            .set_initial_level(0.0)
            .build();

        let n2: Junction = JunctionBuilder::new()
            .set_id(2)
            .set_name("n2")
            .set_elevation(90.0f64)
            .set_demand(77.26 / 3600.0)
            .set_pattern(None)
            .build();

        let n3: Junction = JunctionBuilder::new()
            .set_id(3)
            .set_name("n3")
            .set_elevation(88.0f64)
            .set_demand(76.63 / 3600.0)
            .set_pattern(None)
            .build();

        let n4: Junction = JunctionBuilder::new()
            .set_id(4)
            .set_name("n4")
            .set_elevation(90.0f64)
            .set_demand(75.80 / 3600.0)
            .set_pattern(None)
            .build();

        let n5: Junction = JunctionBuilder::new()
            .set_id(5)
            .set_name("n5")
            .set_elevation(85.0f64)
            .set_demand(145.46 / 3600.0)
            .set_pattern(None)
            .build();

        let p1: Pipe = PipeBuilder::new()
            .set_id(1)
            .set_name("P1")
            .set_start(1)
            .set_end(2)
            .set_length(1000.0)
            .set_diameter(0.40)
            .set_roughness(130.0)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .build();

        let p2: Pipe = PipeBuilder::new()
            .set_id(2)
            .set_name("P2")
            .set_start(2)
            .set_end(3)
            .set_length(1000.0)
            .set_diameter(0.350)
            .set_roughness(130.0)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .build();

        let p3: Pipe = PipeBuilder::new()
            .set_id(3)
            .set_name("P3")
            .set_start(3)
            .set_end(4)
            .set_length(1000.0)
            .set_diameter(0.30)
            .set_roughness(130.0)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .build();
        let p4: Pipe = PipeBuilder::new()
            .set_id(4)
            .set_name("P4")
            .set_start(4)
            .set_end(5)
            .set_length(1000.0)
            .set_diameter(0.30)
            .set_roughness(130.0)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .build();
        let ts = vec![n1];
        let js = vec![n2, n3, n4, n5];
        let ps = vec![p1, p2, p3, p4];

        let net: Network = NetworkBuilder::new()
            .set_title(Some("Network Todini 1".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .build();

        net
    }

    ///
    /// Network 2 (Todini et al., 2021) Pressure Flow-Based Algo for PD analysis of WDN (J. Water Resour.Plann.Manage.)
    ///
    #[allow(dead_code)]
    pub fn network2_todini() -> Network {
        let n1: Tank = TankBuilder::new()
            .set_id(1)
            .set_name("n1-tank")
            .set_elevation(20.0)
            .set_initial_level(0.0)
            .build();

        let n2: Junction = JunctionBuilder::new()
            .set_id(2)
            .set_name("n2")
            .set_elevation(0.0)
            .set_demand(300.74)
            .build();

        let mut n3 = n2.clone();
        n3.id = 3;
        n3.name = Some(String::from("n3"));
        n3.demand = 207.58;

        let mut n4 = n3.clone();
        n4.id = 4;
        n4.name = Some(String::from("n4"));
        n4.demand = 296.86;

        let mut n5 = n3.clone();
        n5.id = 5;
        n5.name = Some(String::from("n5"));
        n5.demand = 205.74;

        let p1: Pipe = PipeBuilder::new()
            .set_id(1)
            .set_name("P1")
            .set_start(1)
            .set_end(2)
            .set_length(500.0)
            .set_diameter(300.0)
            .set_roughness(145.0)
            .set_minorloss(0.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .build();

        let mut p2 = p1.clone();
        p2.id = 2;
        p2.start = 2;
        p2.end = 3;
        p2.name = Some("P2".to_owned());

        let mut p3 = p1.clone();
        p3.id = 3;
        p3.start = 2;
        p3.end = 4;
        p3.name = Some("P3".to_owned());

        let mut p4 = p1.clone();
        p4.id = 4;
        p4.start = 2;
        p4.end = 5;
        p4.name = Some("P4".to_owned());

        let mut p5 = p1.clone();
        p5.id = 5;
        p5.start = 3;
        p5.end = 5;
        p5.name = Some("P5".to_owned());

        let mut p6 = p1.clone();
        p6.id = 6;
        p6.start = 4;
        p6.end = 5;
        p6.name = Some("P6".to_owned());

        let options: Options = OptionsBuilder::new()
            .set_flow_unit(FlowUnits::Cmh)
            .set_headlossformula(HeadlossFormula::Hw)
            .build();

        let ts = vec![n1];
        let js = vec![n2, n3, n4, n5];
        let ps = vec![p1, p2, p3, p4, p5, p6];

        let net2: Network = NetworkBuilder::new()
            .set_title(Some("Network Todini 2".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .set_options(options)
            .build();
        net2
    }

    ///
    /// Two-Loop Network.
    ///
    #[allow(dead_code)]
    pub fn network_2loop() -> Network {
        let r1: Tank = TankBuilder::new()
            .set_id(1)
            .set_elevation(210.0)
            .set_initial_level(0.0)
            .set_name("Reservoir R-1")
            .build();

        let j2: Junction = JunctionBuilder::new()
            .set_id(2)
            .set_name("J-2")
            .set_elevation(150.0)
            .set_demand(100.00)
            .build();

        let mut j3 = j2.clone();
        j3.id = 3;
        j3.elevation = 160.0;
        j3.demand = 100.00;
        j3.name = Some("J-3".to_owned());

        let mut j4 = j2.clone();
        j4.id = 4;
        j4.elevation = 155.0;
        j4.demand = 120.0;
        j4.name = Some("J-4".to_owned());

        let j5: Junction = JunctionBuilder::new()
            .set_id(5)
            .set_name("J-5")
            .set_elevation(150.0)
            .set_demand(270.00)
            .build();

        let j6: Junction = JunctionBuilder::new()
            .set_id(6)
            .set_name("J-6")
            .set_elevation(165.0)
            .set_demand(330.00)
            .build();

        let j7: Junction = JunctionBuilder::new()
            .set_id(7)
            .set_name("J-7")
            .set_elevation(160.0)
            .set_demand(200.00)
            .build();

        let mut p1: Pipe = PipeBuilder::new()
            .set_id(1)
            .set_name("P-1")
            .set_start(r1.id)
            .set_end(j2.id)
            .set_diameter(609.6)
            .set_length(1000.0)
            .set_roughness(130.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .set_minorloss(0.0)
            .build();

        let mut p2: Pipe = p1.clone();
        p2.id = 2;
        p2.name = Some("P-2".to_owned());
        p2.start = 2;
        p2.end = 3;

        let mut p3: Pipe = p1.clone();
        p3.id = 3;
        p3.name = Some("P-3".to_owned());
        p3.start = 2;
        p3.end = 4;

        let mut p4: Pipe = p1.clone();
        p4.id = 4;
        p4.name = Some("P-4".to_owned());
        p4.start = 4;
        p4.end = 5;

        let mut p5: Pipe = p1.clone();
        p5.id = 5;
        p5.name = Some("P-5".to_owned());
        p5.start = 4;
        p5.end = 6;

        let mut p6: Pipe = p1.clone();
        p6.id = 6;
        p6.name = Some("P-6".to_owned());
        p6.start = 6;
        p6.end = 7;

        let mut p7: Pipe = p1.clone();
        p7.id = 7;
        p7.name = Some("P-7".to_owned());
        p7.start = 3;
        p7.end = 5;

        let mut p8: Pipe = p1.clone();
        p8.id = 8;
        p8.name = Some("P-8".to_owned());
        p8.start = 7;
        p8.end = 5;

        let options: Options = OptionsBuilder::new()
            .set_flow_unit(FlowUnits::Cmh)
            .set_headlossformula(HeadlossFormula::Hw)
            .build();

        //----------------
        p1.diameter = 457.2;
        p2.diameter = 254.0;
        p3.diameter = 406.4;
        p4.diameter = 101.6;
        p5.diameter = 406.4;
        p6.diameter = 254.0;
        p7.diameter = 254.0;
        p8.diameter = 25.4;

        let ts = vec![r1];
        let js = vec![j2, j3, j4, j5, j6, j7];
        let ps = vec![p1, p2, p3, p4, p5, p6, p7, p8];

        let netw: Network = NetworkBuilder::new()
            .set_title(Some("Network Two-loop".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .set_options(options)
            .build();
        netw
    }

    ///
    /// Kadu Network.
    ///
    #[allow(dead_code)]
    pub fn kadu_network() -> Network {
        let r1: Tank = TankBuilder::new()
            .set_id(1)
            .set_elevation(100.0)
            .set_initial_level(0.0)
            .set_name("Tank 1")
            .build();

        let j2: Junction = JunctionBuilder::new()
            .set_id(2)
            .set_name("J-2")
            .set_elevation(0.0)
            .set_demand(5.6 / 60.0)
            .build();

        let j3: Junction = JunctionBuilder::new()
            .set_id(3)
            .set_name("J-3")
            .set_elevation(0.0)
            .set_demand(7.2 / 60.0)
            .build();

        let j4: Junction = JunctionBuilder::new()
            .set_id(4)
            .set_name("J-4")
            .set_elevation(0.0)
            .set_demand(2.9 / 60.0)
            .build();

        let j5: Junction = JunctionBuilder::new()
            .set_id(5)
            .set_name("J-5")
            .set_elevation(0.0)
            .set_demand(6.0 / 60.0)
            .build();

        let j6: Junction = JunctionBuilder::new()
            .set_id(6)
            .set_name("J-6")
            .set_elevation(0.0)
            .set_demand(3.2 / 60.0)
            .build();

        let p1: Pipe = PipeBuilder::new()
            .set_id(1)
            .set_name("P-1")
            .set_start(1)
            .set_end(2)
            .set_diameter(350.0)
            .set_length(300.0)
            .set_roughness(130.0)
            .set_status(LinkStatus::Open)
            .set_check_valve(false)
            .set_minorloss(0.0)
            .build();

        let mut p2: Pipe = p1.clone();
        p2.id = 2;
        p2.name = Some("P-2".to_owned());
        p2.start = 1;
        p2.end = 3;
        p2.length = 400.0;

        let mut p3: Pipe = p1.clone();
        p3.id = 3;
        p3.name = Some("P-3".to_owned());
        p3.start = 2;
        p3.end = 4;
        p3.length = 200.0;

        let mut p4: Pipe = p1.clone();
        p4.id = 4;
        p4.name = Some("P-4".to_owned());
        p4.start = 2;
        p4.end = 5;
        p4.length = 250.0;

        let mut p5: Pipe = p1.clone();
        p5.id = 5;
        p5.name = Some("P-5".to_owned());
        p5.start = 5;
        p5.end = 6;
        p5.length = 200.0;

        let mut p6: Pipe = p1.clone();
        p6.id = 6;
        p6.name = Some("P-6".to_owned());
        p6.start = 2;
        p6.end = 3;
        p6.length = 350.0;

        let mut p7: Pipe = p1.clone();
        p7.id = 7;
        p7.name = Some("P-7".to_owned());
        p7.start = 3;
        p7.end = 5;
        p7.length = 300.0;

        let options: Options = OptionsBuilder::new()
            .set_flow_unit(FlowUnits::Cms)
            .set_headlossformula(HeadlossFormula::Hw)
            .build();

        let ts = vec![r1];
        let js = vec![j2, j3, j4, j5, j6];
        let ps = vec![p1, p2, p3, p4, p5, p6, p7];

        let netw: Network = NetworkBuilder::new()
            .set_title(Some("Network Two-loop".into()))
            .set_junctions(Some(js))
            .set_tanks(Some(ts))
            .set_pipes(Some(ps))
            .set_options(options)
            .build();
        netw
    }

    #[derive(Clone)]
    pub struct CostRecord {
        pub diameter: f64,
        pub cost: f64,
    }
    impl CostRecord {
        pub fn new(diameter: f64, cost: f64) -> Self {
            CostRecord { diameter, cost }
        }
    }

    ///
    /// The function returns the cost table of the "Two (2) loop network".
    ///
    pub fn twoloop_cost_table() -> Vec<CostRecord> {
        let mut cost_table: Vec<CostRecord> = Vec::new();
        cost_table.push(CostRecord::new(25.4, 2.0));
        cost_table.push(CostRecord::new(50.8, 5.0));
        cost_table.push(CostRecord::new(76.2, 8.0));
        cost_table.push(CostRecord::new(101.6, 11.0));
        cost_table.push(CostRecord::new(152.4, 16.0));
        cost_table.push(CostRecord::new(203.20, 23.0));
        cost_table.push(CostRecord::new(254.0, 32.0));
        cost_table.push(CostRecord::new(304.8, 50.0));
        cost_table.push(CostRecord::new(355.6, 60.0));
        cost_table.push(CostRecord::new(406.4, 90.0));
        cost_table.push(CostRecord::new(457.2, 130.0));
        cost_table.push(CostRecord::new(508.0, 170.0));
        cost_table.push(CostRecord::new(558.8, 300.0));
        cost_table.push(CostRecord::new(609.6, 550.0));

        cost_table
    }

    ///
    /// The function returns the cost table of "Hanoi city network".
    ///
    pub fn hanoi_cost_table() -> Vec<CostRecord> {
        let mut cost_table: Vec<CostRecord> = Vec::new();
        cost_table.push(CostRecord::new(304.8, 45.726));
        cost_table.push(CostRecord::new(406.4, 70.40));
        cost_table.push(CostRecord::new(508.0, 98.378));
        cost_table.push(CostRecord::new(609.6, 129.333));
        cost_table.push(CostRecord::new(762.0, 180.748));
        cost_table.push(CostRecord::new(1016.0, 278.280));

        cost_table
    }

    pub fn hanoi_searchbounds() -> (Vec<f64>, Vec<f64>) {
        //let lb = vec![609.6,	609.6,	609.6,	609.6,	609.6,	508.0,	508.0,	508.0,	406.4,	406.4,	304.8,	304.8,	304.8,	304.8,	304.8,	304.8,	406.4,	508.0,	508.0,	609.6,	304.8,	304.8,	508.0,	406.4,	304.8,	304.8,	304.8,	304.8,	304.8,	304.8,	304.8,	304.8,	304.8,	406.4];

        //let ub = vec![1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	1016.0,	762.0,	609.6,	508.0,	508.0,	609.6,	762.0,	1016.0,	1016.0,	1016.0,	1016.0,	609.6,	508.0,	1016.0,	1016.0,	762.0,	609.6,	609.6,	508.0,	508.0,	508.0,	508.0,	508.0,	508.0,	1016.0];

        // My reduction technique :

        // 02 lower diameters :
        //let lb = vec![609.6, 609.6, 609.6, 609.6, 609.6, 609.6, 609.6, 609.6, 609.6, 406.4, 304.8, 304.8, 508.0, 508.0, 406.4, 406.4, 304.8, 304.8, 304.8, 609.6, 304.8, 304.8, 508.0, 406.4, 304.8, 304.8, 304.8, 304.8, 406.4, 406.4, 304.8, 304.8, 304.8, 304.8];

        // 03 lower diameters :
        //let lb = vec![508.0, 508.0, 508.0, 508.0, 508.0, 508.0, 508.0, 508.0, 508.0, 304.8, 304.8, 304.8, 406.4, 406.4, 304.8, 304.8, 304.8, 304.8, 304.8, 508.0, 304.8, 304.8, 406.4, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8, 304.8];
        //let ub = vec![1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 1016.0, 762.0, 762.0, 1016.0, 1016.0, 1016.0, 1016.0, 762.0, 508.0, 508.0, 1016.0, 762.0, 609.6, 1016.0, 1016.0, 762.0, 762.0, 508.0, 508.0, 1016.0, 1016.0, 762.0, 609.6, 609.6, 609.6];

        let lb: Vec<f64> = vec![290.0; 34];
        let ub: Vec<f64> = vec![1100.0; 34];
        (lb, ub)
    }

    ///
    /// The function returns the cost table of "Combined Gravity network".
    ///
    pub fn combined_gravity_cost_table() -> Vec<CostRecord> {
        let mut cost_table: Vec<CostRecord> = Vec::new();
        cost_table.push(CostRecord::new(25.0, 4.50));
        cost_table.push(CostRecord::new(50.0, 15.80));
        cost_table.push(CostRecord::new(80.0, 34.20));
        cost_table.push(CostRecord::new(100.0, 66.48));
        cost_table.push(CostRecord::new(125.0, 84.60));
        cost_table.push(CostRecord::new(150.0, 146.80));
        cost_table.push(CostRecord::new(200.0, 238.40));

        cost_table
    }

    ///
    /// The function returns the cost table of "Pescara city network".
    ///
    pub fn pescara_cost_table() -> Vec<CostRecord> {
        let mut cost_table: Vec<CostRecord> = Vec::new();
        cost_table.push(CostRecord::new(100.0, 27.7));
        cost_table.push(CostRecord::new(125.0, 38.0));
        cost_table.push(CostRecord::new(150.0, 40.5));
        cost_table.push(CostRecord::new(200.0, 55.40));
        cost_table.push(CostRecord::new(250.0, 75.0));
        cost_table.push(CostRecord::new(300.0, 92.4));
        cost_table.push(CostRecord::new(350.0, 123.1));
        cost_table.push(CostRecord::new(400.0, 141.9));
        cost_table.push(CostRecord::new(450.0, 169.3));
        cost_table.push(CostRecord::new(500.0, 191.5));
        cost_table.push(CostRecord::new(600.0, 246.0));
        cost_table.push(CostRecord::new(700.0, 319.6));
        cost_table.push(CostRecord::new(800.0, 391.1));
        //cost_table.push(CostRecord::new(0, 0));

        cost_table
    }
}
