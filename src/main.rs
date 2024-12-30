pub mod aggregator;
pub mod dkg;

use crate::aggregator::Aggregator;

fn main() {
    // TODO: implement entrypoint.
   let agg = Aggregator::new(3, 2);

   for share in &agg.shares {
      match agg.validate_share(&share) {
        Ok(_) => {
            println!("Share {:?} valid ✅", &share.index);
        },
        Err(err) => {
            println!("Invalid share {:?}", err);
        }
      }
   }

   let res = agg.aggregate();

   match res {
    Ok(_) => {
        println!("shares {:?}", agg.shares);
        println!("keys {:?}", agg.keypairs);
        println!("commitments {:?}", agg.commitments);
        println!("Executed successfully ✅");
    },
    Err(err) => {
        println!("An error has occurred: {:?}", err)
    }
   }
}
