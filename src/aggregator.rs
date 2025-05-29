use crate::dkg::{generate_shares, create_random_scalar};

use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::traits::Identity;
use sha2::{Digest, Sha512};

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
  pub r: RistrettoPoint,
  pub s: Scalar,
}

impl AggregatedSignature {
  pub fn new(r: RistrettoPoint, s: Scalar) -> Self {
      Self { r, s }
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

  pub fn aggregate(&self, message: &[u8]) -> Result<AggregatedSignature, AggregationError> {
    let mut nonces: Vec<Scalar> = vec![];
    let mut r_points: Vec<RistrettoPoint> = vec![];

    // Step 1: Each signer generates a nonce and computes R_i = G * r_i
    for _ in 0..self.threshold {
        let r_i = create_random_scalar();
        let R_i = RISTRETTO_BASEPOINT_TABLE * &r_i;
        nonces.push(r_i);
        r_points.push(R_i);
    }

    // Step 2: Aggregate commitments
    let R = r_points.iter().fold(RistrettoPoint::identity(), |acc, x| acc + x);

    // Step 3: Compute challenge c = H(R || group_public || message)
    let mut hasher = Sha512::default();
    hasher.update(R.compress().as_bytes());
    hasher.update(self.keypairs[0].group_public.compress().as_bytes());
    hasher.update(message);
    let hash_result = hasher.finalize(); // 64 bytes from SHA-512
    let c = Scalar::from_bytes_mod_order(hash_result[..32].try_into().unwrap());

    // Step 4: Each signer computes s_i = r_i + c * share.value
    let s_i: Vec<Scalar> = nonces.iter().zip(&self.shares)
        .map(|(r_i, share)| r_i + c * share.value)
        .collect();

    // Step 5: Aggregate responses: s = Σ s_i
    let s = s_i.iter().fold(Scalar::ZERO, |acc, x| acc + x);

    // Step 6: Return final aggregated signature
    Ok(AggregatedSignature::new(R, s))
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