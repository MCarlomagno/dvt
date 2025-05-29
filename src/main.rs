pub mod aggregator;
pub mod dkg;

use crate::aggregator::Aggregator;

fn main() {
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

   let res = agg.aggregate("test".as_bytes());

   match res {
    Ok(signature) => {
        println!("signature R {:?}", signature.r);
        println!("signature S {:?}", signature.s);
        println!("Executed successfully ✅");
    },
    Err(err) => {
        println!("An error has occurred: {:?}", err)
    }
   }
}
