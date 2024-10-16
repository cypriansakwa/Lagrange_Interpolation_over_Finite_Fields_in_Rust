## Overview 
- This Rust program computes the Lagrange interpolation polynomial over a finite field $\mathbb{Z}_p$, given a set of points. The result is the interpolating polynomial modulo 
$p$, expressed as coefficients of powers of $x$.
## Features
- **Lagrange Interpolation:** Compute the polynomial that passes through the given points modulo a prime $p$.
- **Modular Arithmetic:** Uses modular arithmetic throughout to ensure computations stay within the field $\mathbb{Z}_p$.
- **Arbitrary Precision:** Utilizes `num-bigint` for handling large integers and `num-traits` for basic number operations.
## Example
- Given the set $S={(0,4),(−2,1),(2,3)}$ and the modulus $p=5$, the program computes the polynomial:
