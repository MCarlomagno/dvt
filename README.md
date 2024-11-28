## Algorithm
1. generate shares
2. derive public key using shares
3. distribute shares using distributed key generation ceremony (DKG).
3. A new message arrives and colaborators sign the message uing shares
4. When a new signature is added, the resulting signature is verified against public key
5. Once the resulting signature is valid (correctly verified) the message get's succesfully signed.

## TSS for ECDSA (Ethereum-Compatible)
### Key Generation:
- The private key is split into shares, one for each participant.
- Each participant holds a private share, and the private key is mathematically represented as the sum of these shares weighted by Lagrange coefficients.
- The shared public key is derived from the private key and is used for signature verification.

### Signature Generation:
- The signature consists of two parts: a random value and the signing component.
- A random nonce is generated collaboratively by the participants in a distributed manner. This nonce is used to derive the first part of the signature, called the random value.
- Each participant uses their private share and the nonce to compute their contribution to the second part of the signature, which is the signing component. This involves the hash of the message being signed and the participant's private share.
- The contributions from all participants are combined using Lagrange interpolation to produce the final signing component.

### Final Signature:
- The final signature is indistinguishable from one produced by a standard ECDSA process.
- It can be verified using the shared public key and is compatible with Ethereum's transaction verification mechanisms.
