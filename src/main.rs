mod aggregator;
mod dkg;

fn main() {
    // TODO: implement entrypoint.
   let agg = aggregator::Aggregator::new(3, 2);
   let res = agg.aggregate();

   match res {
    Ok(_) => {
        println!("Executed successfully ✅");
        println!("shares {:?}", agg.shares);
    },
    Err(err) => {
        println!("An error has occurred: {:?}", err)
    }
   }
}
