mod aggregator;

fn main() {
    // TODO: implement entrypoint.
   let agg = aggregator::SignatureAggregator::new();
   agg.aggregate();
}
