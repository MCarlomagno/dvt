use crate::aggregator::Share;
use rand::{thread_rng, RngCore};

use curve25519_dalek::constants::RISTRETTO_BASEPOINT_TABLE;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;

/**
 * TODO: Implement key distribution ceremony
 */

pub fn create_random_scalar() -> Scalar {
  let mut bytes: [u8; 32] = [0u8; 32];
  thread_rng().fill_bytes(&mut bytes);
  return Scalar::from_bytes_mod_order(bytes);
}


pub fn generate_shares(secret: Scalar, total_shares: u8, threshold: u8) -> Result<(Vec<RistrettoPoint>, Vec<Share>), String> {
  if threshold <= 1 {
    return Err("Threshold must be higher than 1".to_string());
  }
  if total_shares < threshold {
    return Err("Total shares must be equals or higher than threshold".to_string());
  }

  let coefs = threshold - 1;

  let mut coefficients: Vec<Scalar> = Vec::with_capacity(coefs as usize);
  let mut commitment: Vec<RistrettoPoint> = Vec::with_capacity(threshold as usize);

  for _ in 0..coefs {
    coefficients.push(create_random_scalar());
  }

  commitment.push(RISTRETTO_BASEPOINT_TABLE * &secret);
  for c in &coefficients {
    commitment.push(RISTRETTO_BASEPOINT_TABLE * &c);
  }

  let mut shares: Vec<Share> = Vec::with_capacity(total_shares as usize);
  for i in 0..total_shares {
    // polynomial with `secret` as f(0) and random coefficients
    let scalar_index = Scalar::from(i+1);
    let mut value = Scalar::ZERO;
    for j in (0..coefs).rev() {
        value += &coefficients[j as usize];
        value *= scalar_index;
    }
    // The secret is the constant term in the polynomial: f(0)
    value += secret;
    shares.push(Share {
        gen_index: 0,
        index: i,
        value,
    });
  }
  Ok((commitment, shares))
}