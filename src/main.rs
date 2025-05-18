mod fx_rate;
mod ccy;
mod bellman_ford;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //let usd = ccy::CCY::USD;
    //let eur = ccy::CCY::EUR;
    /*
    let usd_eur = fx_rate::FXRate::new(ccy::CCY::USD, ccy::CCY::EUR, 1.2);
    println!("{}", usd_eur.ccy1_to_ccy2(100.0));
    println!("{}", usd_eur.ccy2_to_ccy1(100.0));
    */
    println!("Hello World");

    let e1: bellman_ford::Edge = bellman_ford::Edge{a:0, b:1 , cost:-1.0};
    let e2: bellman_ford::Edge = bellman_ford::Edge{a:1, b:2 , cost:2.0};
    let e3: bellman_ford::Edge = bellman_ford::Edge{a:2, b:0 , cost:-2.0};

    let graph: bellman_ford::Graph = bellman_ford::Graph{num_vertex: 3, edges: vec![e1,e2,e3]};

    let (status, cycle) = bellman_ford::bellman_ford_neg_cycle( &graph );
    println!("{}", status);
    if status {
        bellman_ford::print_cycle(&cycle);
    }

}
