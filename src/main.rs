mod fx_rate;
mod ccy;
mod bellman_ford;
mod arbitrage;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    println!("Hello FX Arbitrage Example");
    let usd_eur = fx_rate::FXRate::new(ccy::CCY::USD, ccy::CCY::EUR, 1.3);
    let eur_gbp = fx_rate::FXRate::new(ccy::CCY::EUR, ccy::CCY::GBP, 1.1);
    let gbp_usd = fx_rate::FXRate::new(ccy::CCY::GBP , ccy::CCY::USD, 1.0);
    //let usd_jpy = fx_rate::FXRate::new(ccy::CCY::USD, ccy::CCY::JPY, 10.0);
    let fx_rates_vec: Vec<fx_rate::FXRate> = vec![usd_eur.clone(), eur_gbp.clone(), gbp_usd.clone()];
    let (covered_ccy_pairs, fx_rate_grid, graph) = arbitrage::create_graph_info_from_fx_rates(&fx_rates_vec);
    

    let (status, cycle) = bellman_ford::bellman_ford_neg_cycle( &graph );    
    if status {        
        println!("Arbitrage Found!");
        let cycle_ccy = arbitrage::int_to_ccy_path(&cycle);    
        println!("Arbitrage cycle:");
        arbitrage::print_cycle(&cycle_ccy);
        let init_amount: f32 = 1.0;
        let payout = arbitrage::get_path_payout(&cycle, &fx_rate_grid, init_amount);
        println!("Arbitrage Payout = {} ; with init amount = {}", payout, init_amount);
    }    

    

}
