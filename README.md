# acvm-backend-groth16

This is a backend for the [ACVM](https://github.com/noir-lang/acvm) which allows proving/verifying ACIR circuits against Arkworks' [groth16](https://github.com/arkworks-rs/groth16) proving system.

**Both this backend and the underlying `ark-groth16` library are proofs of concept and are not production-ready.**

## Circuit specific setup required

Note that groth16 proving system requires a circuit specific trusted setup in order to be able to generate secure proofs.

By default this backend will generate a local set of proving and verification keys to allow for circuit development; however these should be replaced with keys from a [trusted setup with contributions from multiple parties](https://vitalik.ca/general/2022/03/14/trustedsetup.html).
