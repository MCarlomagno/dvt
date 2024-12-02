mod aggregator;

fn main() {
    // TODO: implement entrypoint.
   let agg = aggregator::SignatureAggregator::new();
   let res = agg.aggregate();

   match res {
    Ok(_) => {
        println!("Executed successfully ✅");

    },
    Err(err) => {
        println!("An error has occurred: {:?}", err)
    }
   }
}
