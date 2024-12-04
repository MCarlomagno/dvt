pub mod aggregator;
pub mod dkg;

use crate::aggregator::Aggregator;

fn main() {
    // TODO: implement entrypoint.
   let agg = Aggregator::new(3, 2);
   let res = agg.aggregate();

   match res {
    Ok(_) => {
        println!("Executed successfully ✅");
        println!("shares {:?}", agg.shares);
        println!("keys {:?}", agg.keypairs);
        println!("commitments {:?}", agg.commitments);
    },
    Err(err) => {
        println!("An error has occurred: {:?}", err)
    }
   }
}
