//use std::borrow::BorrowMut;
//use std::ops::Index;

#[cfg(feature = "optimization")] use std::collections::HashMap;

//use peroxide::prelude::P;
//use petgraph::data::Build;
#[cfg(feature = "optimization")] use petgraph::graph::Graph;
//use petgraph::graph::Node;
#[cfg(feature = "optimization")] use petgraph::graph::NodeIndex;

#[cfg(feature = "optimization")] use petgraph::graph::EdgeReference;
#[cfg(feature = "optimization")] use petgraph::algo::*;
#[cfg(feature = "optimization")] use petgraph::data::FromElements;
//use petgraph::visit::GetAdjacencyMatrix;
//use petgraph::visit::IntoEdges;
#[cfg(feature = "optimization")] use super::network::Network;
#[cfg(feature = "optimization")] use super::network::FlowUnits;

#[cfg(feature = "optimization")]
#[derive(Debug, Clone)]
pub struct GraphNet {
   //network : &'a Network, 
   pub graph : Graph<GrNode, GrEdge>,
   pub tree : Option<Graph<GrNode, GrEdge, petgraph::Undirected>>,
   pub refnode_index : Option<NodeIndex<u32>>,
}

#[cfg(feature = "optimization")]
impl GraphNet {
   pub fn build_from(network : &Network)-> Option<Self>{

        // let mut graf : Graph<GrNode, GrEdge, petgraph::Directed> = Graph::new();
         //let mut refnode_index : Option<NodeIndex <GrNode>> = None;   
         let mut netgraph = GraphNet{
            graph : Graph::new(), //graf,
            //network, 
            refnode_index : None,
            tree : None,
        };

         match &network.junctions {
            None =>{},
            Some(nodes) => {
                match &network.options {
                    None => {},
                    Some(options) => {
                        
                        for nd in nodes.iter(){
                            let th = match nd.get_target_head(){
                                None => 00.0,
                                Some(th) => th,
                            };
                            netgraph.graph.add_node(GrNode{id : nd.id, tree_index : 0, target_head : th, head : None, demand : Self::convert2si(&options.flow_unit, nd.demand)});
                        }
                    },
                };                
            },
         };
         
         let mut max_head : f64 = f64::MIN;
                  
         match &network.tanks {
            None =>{},
            Some(nodes) => {
                for nd in nodes.iter(){
                  let ndix = netgraph.graph.add_node(GrNode{id : nd.id, tree_index : 0, demand : 0.0, target_head : nd.head(), head : Some(nd.head())});
                    if nd.head() > max_head { max_head = nd.head(); netgraph.refnode_index = Some(ndix.clone());} 
                } 
            }   
         }

         match &network.reservoirs{
            None =>{},
            Some(nodes) => {
                for nd in nodes.iter(){
                   let ndix = netgraph.graph.add_node(GrNode{id : nd.id, tree_index : 0, demand : 0.0, target_head : nd.head, head : Some(nd.head)});
                   if nd.head > max_head {max_head = nd.head; netgraph.refnode_index = Some(ndix.clone());} 
                }
            }
         }                 

         match &network.pipes {
            None => {},
            Some(pipes) => {
                //let mut i : usize = 0;
                for p in pipes.iter() {
                     let a : NodeIndex = netgraph.graph.node_indices().find(|i| netgraph.graph[*i].id == p.start).unwrap(); //netgraph.graph.add_node(GrNode { id: p.start, target_head: 30.0});
                     let b : NodeIndex = netgraph.graph.node_indices().find(|i| netgraph.graph[*i].id == p.end).unwrap();
                     let _edg = netgraph.graph.add_edge(a, b, GrEdge{ id : p.id, start : p.start, end : p.end, tree_index : 0, length : p.length, flow : None, headloss : None, diameter : None, roughnes : p.roughness});
                //     i+=1;
                };
            },
        }

        netgraph.get_tree();

        Some(netgraph)
    }

    fn convert2si(unit : &FlowUnits, value : f64)-> f64 {
        match &unit {
         FlowUnits::Afd => value*0.0142764,
         FlowUnits::Cfs => value*0.0283168,
         FlowUnits::Cmd => value/86400.0, //3600 x 24
         FlowUnits::Cmh => value/3600.0,
         FlowUnits::Cms => value,
         FlowUnits::Gpm => value*6.309e-5,
         FlowUnits::Imgd => value,
         FlowUnits::Lpm => value*0.06,
         FlowUnits::Lps => value*0.001,
         FlowUnits::Mgd => value,
         FlowUnits::Mld => value/86.40,
        }
    }

   fn get_tree(&mut self) {        
        let tree : Graph<GrNode, GrEdge, petgraph::Undirected> = Graph::from_elements(min_spanning_tree(&self.graph));
        self.tree = Some(tree);        
    }
       
   pub fn compute_heads_headlosses(&mut self){        
        match &self.tree {
            None => {panic!("no tree !!")},
            Some(tree) => {
         
                //let tree = self.get_tree();

               // println!("\n Tree = \n {:?}", tree);

                //println!("ref node : \n {:?} ", self.refnode_index);
                let mut paths : Vec<(f64, Vec<NodeIndex>, f64)> = Vec::new();

                match self.refnode_index {
                    None => {},
                    Some(n1)=> {
                        let nns : Vec<NodeIndex> = tree.node_indices().collect(); 
                        for n2 in nns.iter(){
                            let res_path = astar(&tree, n1, |n| n == *n2 && n1 != *n2, Self::length_as_weight, Self::no_clue_heuristic);
                            match res_path {
                                None => {},
                                Some((cost, path))=> paths.push((cost, path, f64::MAX)), 
                            };
                        }
                    },
                }
                
                //pathes.iter_mut().map(|p| p.2 =Some((self.graph[*p.1.first().unwrap()].target_head -self.graph[*p.1.last().unwrap()].target_head)/p.0));
                //-----
                    for p in paths.iter_mut(){
                        p.2 = (self.graph[*p.1.first().unwrap()].target_head -self.graph[*p.1.last().unwrap()].target_head)/p.0;
                        //println!("path : {:?}", p);
                    }
                //-----

                let critical_path = paths.iter().max_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

                //println!("\n The critical path : \n {:?}", critical_path);        
                    
                //compute heads :
                match critical_path {
                    None => {panic!("no critical path !!")},
                    Some(path) => {
                        for i in 0..path.1.len()-1 {
                            match self.graph[path.1[i]].head {
                                None => panic!("no fixed head in the network"), //println!("\n no head in node {}", i),
                                Some(h) => {
                                    //println!("\n head in node {} : {:?}", i,  h);
                                    match self.graph.find_edge(path.1[i], path.1[i+1]){
                                        None => { 
                                            //println!("No edge between {} and {}", i, i+1);
                                            match self.graph.find_edge(path.1[i+1], path.1[i]) {
                                                None => {},//  println!("No edge between {} and {}", i+1, i),
                                                Some(edge)=> {
                                                    //println!("The edge ----> {}--{} : {:?}", i, i+1, edge);
                                                    self.graph[path.1[i+1]].head = Some(h - (path.2* self.graph[edge].length));
                                                }
                                            }
                                        },
                                        Some(edge) => {
                                            //println!("The edge ----> {}--{} : {:?}", i, i+1, edge);
                                            self.graph[path.1[i+1]].head = Some(h - (path.2* self.graph[edge].length));

                                        }
                                    }
                                }
                            };
                            //self.graph[path.1[i+1]].head = Some(self.graph[path.1[i]].head.unwrap() - (path.2* self.graph[self.graph.find_edge(path.1[i], path.1[i+1]).unwrap()].length));
                        }      
                    }
                }

                /*for nd in critical_path.unwrap().1.iter(){
                     println!("head in critical path --- {:?}",  self.graph[*nd].head);
                }*/     
                
                //println!("--------------");

                let terminal_nodes : Vec<NodeIndex> = tree.node_indices().filter(|a| tree.edges(*a).count()==1).collect();
               
                match critical_path {
                    None => {panic!("no critical path !!!")},
                    Some(critic_path)=>{

                        let mut terminalpaths : Vec<(f64, Vec<NodeIndex>)> = Vec::new();
                         // Find paths from one node in the critical path to terminal nodes.
                        for a in terminal_nodes.iter() {
                                for n1 in critic_path.1.iter(){
                                    let res_path = astar(&tree, *n1, |n| n == *a && n != *n1, Self::length_as_weight, Self::no_clue_heuristic);    
                                    match res_path {
                                        None =>{},
                                        Some(path)=> { terminalpaths.push(path); break;},                                        
                                    }
                            }
                        };

                        // Compute heads

                        for path in terminalpaths.iter() {
                            for i in 0..path.1.len()-1 {
                                match self.graph[path.1[i]].head {
                                    None => panic!("No head fro computing others !!!"),
                                    Some(h) => {
                                        match self.graph.find_edge(path.1[i], path.1[i+1]){
                                            None => { 
                                                //println!("No edge between {} and {}", i, i+1);
                                                match self.graph.find_edge(path.1[i+1], path.1[i]) {
                                                    None => {},//  println!("No edge between {} and {}", i+1, i),
                                                    Some(edge)=> {
                                                        //println!("The edge ----> {}--{} : {:?}", i, i+1, edge);
                                                        self.graph[path.1[i+1]].head = Some(h - (critic_path.2* self.graph[edge].length));
                                                    }
                                                }
                                            },
                                            Some(edge) => {
                                                //println!("The edge ----> {}--{} : {:?}", i, i+1, edge);
                                                self.graph[path.1[i+1]].head = Some(h - (critic_path.2* self.graph[edge].length));
    
                                            }
                                        }
                                    }
                                }
                            }
                        } 

                    },
                };
                    
            },
        }

        // Compute headloss
        for e in self.graph.edge_indices(){
            match self.graph.edge_endpoints(e){
                None => {},
                Some((a,b)) => {
                    match self.graph[a].head {
                        None =>{},
                        Some(h1) => {
                            match self.graph[b].head {
                                None =>{},
                                Some(h2) => {
                                    self.graph[e].headloss = Some(f64::abs(h1-h2));
                                    //println!("e={}, dh = {:?}", e.index(), f64::abs(h1-h2));
                                },
                            }
                        },
                    }                    
                },
            };             
        };

        // Compute tree headloss from the graph
        match &mut self.tree{
            None => {println!("No treee !!!")},
            Some(tre)=>{
               for e_grph in self.graph.edge_indices(){
                    for e_tre in tre.edge_indices(){
                       if self.graph[e_grph].id == tre[e_tre].id { 
                          tre[e_tre].headloss = self.graph[e_grph].headloss;
                          println!("..... i_grpah: {:?}; i_tree: {:?}, h_tree = {:?}. L : {:?}", self.graph[e_grph].id, tre[e_tre].id, tre[e_tre].headloss, tre[e_tre].length);
                       }   
                    }
               }
                
            },
        }  
      
        //println!("{:?}", self.graph);
    }

   pub fn tree_incidence_matrix(&mut self)-> (Vec<Vec<i32>>, Vec<i32>, Vec<f64>) {
    
        let mut size : usize = 0;
        match &self.tree{
            None => {},
            Some(tre) => size = tre.edge_count(),
        }

        let mut matrix = vec![vec![0i32; size]; size];
        let mut vector  = vec![0i32; size];
        let mut demand_vector = vec![0.0f64; size];

        match &mut self.tree {
            None => {},
            Some(tree) => {
                match self.refnode_index {
                    None => {},
                    Some(refnode)=> {
                        let mut i : usize = 0;
                        let ref_tree_index = tree.node_count();
                        for a in tree.node_indices(){
                            if a.index() != refnode.index() {
                                tree[a].tree_index = i;
                                i+=1;
                            } else {tree[a].tree_index = ref_tree_index;}                            
                        }  
                    },                    
                };

                let mut i : usize = 0;
                for e in tree.edge_indices(){
                    tree[e].tree_index = i ;
                    i+=1;
                }
            },
        };   

        

        match &self.tree {
            None => {},
            Some(tree) => {
                
                //println!("The tree : \n {:?}", tree);

                for eg in self.graph.edge_indices(){
                    for et in tree.edge_indices(){
                        if self.graph[eg].id == tree[et].id {
                            self.graph[eg].tree_index = tree[et].id;
                        }
                    }
                }

                /*for a in tree.node_indices(){
                    println!(" Node --- {}, {}, {}", a.index(), tree[a].id, tree[a].tree_index);
                }

                for e in tree.edge_indices(){
                    println!("Tree.Edge --- {}, tree index : {},", e.index(), tree[e].tree_index)
                }
                
                for e in self.graph.edge_indices(){
                    println!("Graph.Edge :::: --- {}, tree index : {},", e.index(), self.graph[e].tree_index)
                }*/

                match self.refnode_index {
                    None =>{},
                    Some(refnode)=> {
                        for n in tree.node_indices(){
                            if n.index() != refnode.index() {
                                
                                for et in tree.edges(n) {
                                    if et.weight().start == tree[n].id {
                                        matrix[tree[n].tree_index][et.weight().tree_index] = -1;
                                    }
                                    if et.weight().end == tree[n].id {
                                        matrix[tree[n].tree_index][et.weight().tree_index] = 1;
                                    }
                                }

                                  /* let edges_in = tree.edges_directed(n, petgraph::Direction::Incoming);
                                  let edges_out = tree.edges_directed(n, petgraph::Direction::Outgoing);

                                  for ein in edges_in{
                                    matrix[tree[n].tree_index][ein.weight().tree_index] = 1;
                                  } 

                                  for eout in edges_out{
                                    matrix[tree[n].tree_index][eout.weight().tree_index] =-1;
                                  } */
                                
                            }
                        }

                    //println!("Incidence matrix : \n {:?}", matrix);
                        // reference vector 

                        /* let edges_in = tree.edges_directed(refnode, petgraph::Direction::Incoming);
                        let edges_out =  tree.edges_directed(refnode, petgraph::Direction::Outgoing);

                        for ein in edges_in{
                            vector[ein.weight().tree_index] =1;
                         } 

                         for eout in edges_out{
                            vector[eout.weight().tree_index] = -1;
                         } */

                         for et in tree.edges(refnode) {
                            if et.weight().start == tree[refnode].id {
                                vector[et.weight().tree_index] = -1;
                            }
                            if et.weight().end == tree[refnode].id {
                                vector[et.weight().tree_index] = 1;
                            }
                        }

                        // println!("Vector : \n {:?}", vector);
                    }                    
                } 
                
                match self.refnode_index {
                    None => {},
                    Some(refn) =>{
                        for n in tree.node_indices(){
                            if n.index() != refn.index(){
                                demand_vector[tree[n].tree_index]= tree[n].demand;
                        
                            }                
                        }             
                    },
                } 

                // println!("Di = {:?}", demand_vector);               
            }
        };

         //println!("Incident matrix : \n {:?}", matrix);  
         //println!("Incidence vectoror : \n {:?}", vector);
         //println!("demand vectoror : \n {:?}", demand_vector);
        (matrix, vector, demand_vector)

    }
       
   pub fn compute_tree_flows_diameters(&mut self){
        let (a, _b, q) = self.tree_incidence_matrix();

        let inv_a = self.inverse_matrix_jordan(&a);
        match inv_a {
            Err(error) => panic!("I cannot inverse the matrix because of : {} !", error),
            Ok(inva)=>{
                let flows = self.product2(&inva, &q);
                match flows {
                    Err(eror) => panic!("I cannot perform product because of : {} !", eror),
                    Ok(flows)=> {
                          match &mut self.tree {
                            None=>{ println!("nooooo tree !!!")},
                            Some(tre) =>{
                                //let mut i : usize = 0;
                                //Compute flows 

                                for i in 0..flows.len() {
                                   for e in tre.edge_indices() {
                                        if tre[e].tree_index == i {
                                            tre[e].flow = Some(flows[i]);

                                            //println!("Tree Q : {:?}", tre[e].flow );

                                            match tre[e].headloss {
                                                None=>{println!("There is no headloss in the tree edge {:?} !!!", e)},
                                                Some(h) => {
                                                   //println!("L = {:?}, Q = {:?} / Chw = {:}, h ={:?}", tre[e].length, tre[e].flow, tre[e].roughnes, h);

                                                    let x : f64 = 10.667 * tre[e].length * f64::powf( tre[e].flow.unwrap().abs(),1.852)/(h*f64::powf(tre[e].roughnes,1.852));

                                                    //println!(" ----------------------------- x = {:?}", x);
                                                    
                                                    //Set diameters in (mm) unit. 
                                                    tre[e].diameter = Some(1000.0f64 * f64::powf(x, 0.205296654));
                                                    
                                                    //println!("Tree i: {} D : {:?}, x : {}", tre[e].index, tre[e].diameter, x);     
                                                },
                                            }
                                        }   
                                    }
                                }

                                // copie diameters to graph
                            },
                        }
                    },
                }
            }
        }
    }

    fn update_graph_diameters(&mut self){
        match  &self.tree {
            None => panic!("no tree !!"),
            Some (tre)=>{
                for e_graph in self.graph.edge_indices(){
                    for e_tree in tre.edge_indices(){
                        if self.graph[e_graph].id == tre[e_tree].id {
                            self.graph[e_graph].diameter = tre[e_tree].diameter;
                            self.graph[e_graph].flow = tre[e_tree].flow;
                        }
                    }
                }
            },
        }
    }

    ///
    /// Compute reduced diameters of the network (in mm).
    /// 
   pub fn compute_diameters(&mut self)-> HashMap<usize, (Option<f64>, Option<f64>)>{
        self.compute_heads_headlosses();
        self.compute_tree_flows_diameters();
        self.update_graph_diameters();

        let mut result : HashMap<usize, (Option<f64>, Option<f64>)> = HashMap::new();
        for e in self.graph.edge_indices(){
            result.insert(self.graph[e].id, (self.graph[e].flow, self.graph[e].diameter));
        }

        for e in self.graph.edge_indices(){
             println!("Graph : i = {:?}, D = {:?}, dh = {:?}", self.graph[e].id, self.graph[e].diameter, self.graph[e].headloss);
        }

        result         
    }

    #[allow(dead_code)]
    fn length_as_weight(er : EdgeReference<GrEdge>)->f64 {
        er.weight().length
    }
    
    #[allow(dead_code)]
    fn no_clue_heuristic<N>(_nd: N) -> f64 {
        0.0
    }

    #[allow(dead_code)]
    fn inverse_matrix_jordan(&self, matrix : &Vec<Vec<i32>>)-> Result<Vec<Vec<f64>>, String> {
        let n = matrix.len();
        
        if matrix.len() != matrix[0].len() {
            Err(String::from("Matrix is not square!"))
        }
        else {
                
         let mut a = vec![vec![0.0f64; 2*n]; n];
    
        //copy th matrix 
        for i in 0..n {
            for j in 0..n {
                a[i][j]=matrix[i][j] as f64;
            }
        }
    
        for i in 0..n {
            for j in 0..n {
                if i==j {
                    a[i][j+n]=1.0;
                }
            }
        }
    
        //Apply Gauss Jordan Elimination on Augmented Matrix (A):
    
        for i in 0..n {
            if a[i][i] == 0.0 {
                panic!("diagonal is nul")
                //Err(String::from("Diagonal is null !"))     
            }
            else {  
    
            for j in 0..n {
                if i != j {
                    let ratio = a[j][i]/a[i][i];
    
                    for k in 0..2*n {
                         a[j][k] = a[j][k] - ratio *a[i][k]   
                    }
                }
            }
        }
    }
        // Row Operation to Convert Principal Diagonal to 1.
        for i in 0..n {
            for j in n..2*n {
                a[i][j] = a[i][j]/a[i][i];
             }
        }   
    
        //copy result to b :
        let mut b = vec![vec![0.0f64; n]; n];    
        for i in 0..n {
            for j in 0..n {
                b[i][j]= a[i][j+n];
            }
        }
        return Ok(b);
    }
    } 
    
    fn product2(&self, left : &Vec<Vec<f64>>, right : &Vec<f64>)-> Result<Vec<f64>, String> {
        
        let m =  left.len();
        let pl = left[0].len();


        let pr = right.len();

        let mut result = vec![0.0f64; m];
        let mut _sum =0.0f64;
        if pl==pr {
            for i in 0..m{     

                _sum = 0.0f64;

                for j in 0..pl{                          
                _sum += left[i][j]*right[j];     
                } 

                result[i]=_sum;
        }
        Ok(result)
        }
        else{
            Err(String::from("Colomns's count of left matrix not equals rows's count of right vector!"))
        }    
    }


}


#[cfg(feature = "optimization")]
#[derive(Debug, Clone)]
pub struct GrNode {
  pub id : usize,
  pub tree_index : usize,
  pub target_head : f64,
  pub head : Option<f64>,
  pub demand : f64,
}

#[cfg(feature = "optimization")]
#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct GrEdge {
   pub id : usize, 
   pub start : usize,
   pub end : usize,
   pub tree_index : usize,
   pub length : f64,
   pub headloss : Option<f64>,
   pub flow : Option<f64>,
   pub diameter : Option<f64>,
   pub roughnes : f64,
}

