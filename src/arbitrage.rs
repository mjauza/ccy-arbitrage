use crate::bellman_ford;
use crate::fx_rate;
use crate::ccy;
//use enum_map::{enum_map, Enum, EnumMap};
use std::collections::HashSet;
use std::collections::HashMap;
use ndarray::Array2;


pub fn create_graph_info_from_fx_rates(fx_rates: &Vec<fx_rate::FXRate>) -> (HashMap<ccy::CCY, HashSet<ccy::CCY>>, Array2<f32>, bellman_ford::Graph) {    
    let mut covered_ccy_pairs: HashMap<ccy::CCY, HashSet<ccy::CCY>> = HashMap::new();
    let mut edges: Vec<bellman_ford::Edge> = vec![];
    let num_ccys: usize = ccy::get_num_ccys();
    let mut fx_rate_grid = Array2::<f32>::zeros((num_ccys, num_ccys));
    for rate in fx_rates.iter() {
        let ccy1 = &rate.ccy1;
        let ccy2 = &rate.ccy2;
        let ccy1_to_ccy2_edge = -(rate.ccy1_to_ccy2(1.0).ln());
        let ccy2_to_ccy1_edge = -(rate.ccy2_to_ccy1(1.0).ln());

        // 
        if !covered_ccy_pairs.contains_key(ccy1) {
            covered_ccy_pairs.insert((*ccy1).clone(),HashSet::new());
        }

        if !covered_ccy_pairs.contains_key(ccy2) {
            covered_ccy_pairs.insert((*ccy2).clone(),HashSet::new());
        }

        //check if ccy1 in covered pairs and insert
        let mut record_as_ccy1 = true;        
        if covered_ccy_pairs.contains_key(ccy1) && !covered_ccy_pairs[ccy1].contains(ccy2) {
            record_as_ccy1 = false;
        }
        let mut record_as_ccy2 = true;
        if covered_ccy_pairs.contains_key(ccy2) && !covered_ccy_pairs[ccy2].contains(ccy1) {
            record_as_ccy2 = false;
        }

        if (!record_as_ccy1) & (!record_as_ccy2) {
            //record in covered pairs
            covered_ccy_pairs.get_mut(ccy1).unwrap().insert(ccy2.clone());
            covered_ccy_pairs.get_mut(ccy2).unwrap().insert(ccy1.clone());
            // record in edges
            edges.push(bellman_ford::Edge{a:(*ccy1).clone() as u32, b:(*ccy2).clone() as u32 , cost:ccy1_to_ccy2_edge});
            edges.push(bellman_ford::Edge{a:(*ccy2).clone() as u32, b:(*ccy1).clone() as u32, cost:ccy2_to_ccy1_edge});
            // record on rate grid
            fx_rate_grid[[*ccy1 as usize,*ccy2 as usize]] = rate.ccy1_to_ccy2(1.0);
            fx_rate_grid[[*ccy2 as usize,*ccy1 as usize]] = rate.ccy2_to_ccy1(1.0);
        } 
    }
    let graph: bellman_ford::Graph = bellman_ford::Graph{num_vertex: num_ccys as u32, edges: edges};

    let res = (covered_ccy_pairs, fx_rate_grid, graph);
    return res;

    //println!("{:?}", covered_ccy_pairs[ccy::CCY::USD]);
}

pub fn int_to_ccy_path(path: &Vec<u32>) -> Vec<ccy::CCY> {
    return path.iter().map(|x| ccy::CCY::from_int(x)).collect();
}

pub fn print_cycle(cycle: &Vec<ccy::CCY>) -> () {    
    let cycle_str: Vec<String> = cycle.iter().map(|x| x.to_string()).collect();
    let res_str: String = cycle_str.join("->");
    println!("{}",res_str);
}

pub fn get_path_payout(path: &Vec<u32>, fx_rate_grid: &Array2<f32>, init_amount: f32) -> f32 {
    let mut payout: f32 = init_amount.clone();
    for i in 0..(path.len()-1) {
        payout *= fx_rate_grid[[path[i] as usize,path[i+1] as usize]];
    }
    return payout;
}