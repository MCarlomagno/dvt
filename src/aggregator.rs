use crate::dkg::{generate_shares, create_random_scalar};

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::traits::Identity;

#[derive(Debug)]
pub enum AggregationError {
  InvalidShare,
  // Other error variants as needed
}

#[derive(Debug)]
pub struct Share {
  pub gen_index: u8,
  pub index: u8,
  pub value: Scalar,
}


#[derive(Debug)]
pub struct KeyPair {
  pub index: u8,
  pub secret: Scalar,
  pub public: RistrettoPoint,
  pub group_public: RistrettoPoint,
}


pub struct AggregatedSignature {
  // TODO: implement aggregated signature
}

impl AggregatedSignature {
  // TODO: implement aggregated signature
  pub fn new() -> Self {
    Self { }
}
}

pub struct Aggregator {
  pub commitments: Vec<RistrettoPoint>,
  pub keypairs: Vec<KeyPair>,
  pub shares: Vec<Share>,
  pub threshold: u8,
}

impl Aggregator {
  pub fn new(shares: u8, threshold: u8) -> Self {
      // TODO: secret must not be a random value.
      let secret = create_random_scalar();
  
      // create shares
      let res = generate_shares(secret, shares, threshold);

      let (commitments, shares) = match res {
        Ok(val) => val,
        Err(msg) => panic!("Failed to generate shares: {:?}", msg)
      };

      let keypairs: Vec<KeyPair> = shares
        .iter()
        .map(|share| KeyPair {
            secret: share.value,
            public: RISTRETTO_BASEPOINT_TABLE * &share.value,
            group_public: RISTRETTO_BASEPOINT_TABLE * &secret,
            index: share.index,
        }).collect();

      Self { keypairs, commitments, shares, threshold }
  }

  pub fn aggregate(&self) -> Result<AggregatedSignature, AggregationError> {
      // Logic to combine signatures into one aggregated signature
      Ok(AggregatedSignature::new())
  }


  pub fn validate_share(&self, share: &Share) -> Result<(),&str> {
    let term = Scalar::from(share.index);
    let f_result = RISTRETTO_BASEPOINT_TABLE * &share.value;
    let mut result = RistrettoPoint::identity();

    if self.commitments.len() != self.threshold as usize {
        return Err("Missing commitments");
    }

    for (i, comm_i) in self.commitments.iter().rev().enumerate() {
        result += comm_i;

        if i != self.commitments.len() - 1 {
            result *= term;
        }
    }

    if !(f_result == result) {
        return Err("Invalid share");
    }

    Ok(())
  }
}