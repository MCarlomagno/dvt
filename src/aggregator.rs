use crate::dkg::{generate_shares, create_random_scalar};

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;

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

impl std::fmt::Display for Share {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Share(gen: {}, owner: {}, value: {:?})", 
          self.gen_index, self.index, self.value)
  }
}

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
  commitments: Vec<RistrettoPoint>,
  keypairs: Vec<KeyPair>,
  pub shares: Vec<Share>,
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

      Self { keypairs, commitments, shares }
  }

  pub fn sign(&mut self, share: Share) -> Result<(), AggregationError> {
      if Self::validate_share(&share) {
          self.shares.push(share);
          Ok(())
      } else {
          Err(AggregationError::InvalidShare)
      }
  }

  pub fn aggregate(&self) -> Result<AggregatedSignature, AggregationError> {
      // Logic to combine signatures into one aggregated signature
      Ok(AggregatedSignature::new())
  }

  fn validate_share(share: &Share) -> bool {
    // Implement validation logic
    return true;
  }
}