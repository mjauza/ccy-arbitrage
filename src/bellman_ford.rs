/*
Depth first search algo
*/

pub struct Edge {
    pub a: u32,
    pub b: u32,
    pub cost: f32
}

pub struct Graph {
    pub num_vertex: u32,
    pub edges: Vec<Edge>,    
}

pub fn bellman_ford_neg_cycle( graph: &Graph ) -> (bool, Vec<u32>) {
    let mut d: Vec<f32> = vec![0.0; graph.num_vertex as usize];
    let mut p: Vec<u32> = vec![u32::MAX; graph.num_vertex as usize];
    let mut x: i32 = -1;

    for i in 0..graph.num_vertex {
        x = -1;
        for e in graph.edges.iter() {
            if (d[e.a as usize] + e.cost) < d[e.b as usize] {
                d[e.b as usize] = f32::max( f32::MIN , d[e.a as usize] + e.cost );
                p[e.b as usize] = e.a.clone();
                x = e.b.clone() as i32;                
            }
        }
    }

    if x == -1 {
        println!("No negative cycle");
        let res: (bool, Vec<u32>) = (false, vec![]);
        return res;
    }
    else {
        for i in 0..graph.num_vertex {
            x = p[x as usize].clone() as i32;
        }

        let mut cycle: Vec<u32> = vec![];
        let mut v: u32 = x.clone() as u32;
        loop {            
            cycle.push(v.clone());
            if (v == x as u32) && (cycle.len() > 1) {
                break;
            }            
            v = p[v as usize].clone();
        }
        cycle.reverse();        
        let res: (bool, Vec<u32>) = (true, cycle);
        return res;
    }
}

pub fn print_cycle(cycle: &Vec<u32>) -> () {
    let cycle_str: Vec<String> = cycle.iter().map(|x| x.to_string()).collect();
    let res_str: String = cycle_str.join("->");
    println!("{}",res_str);
}
