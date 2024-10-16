## Overview 
- This Rust program computes the Lagrange interpolation polynomial over a finite field $\mathbb{Z}_p$, given a set of points. The result is the interpolating polynomial modulo 
$p$, expressed as coefficients of powers of $x$.
## Features
- **Lagrange Interpolation:** Compute the polynomial that passes through the given points modulo a prime $p$.
- **Modular Arithmetic:** Uses modular arithmetic throughout to ensure computations stay within the field $\mathbb{Z}_p$.
- **Arbitrary Precision:** Utilizes `num-bigint` for handling large integers and `num-traits` for basic number operations.
## Example
- Given the set $S={(0,4),(−2,1),(2,3)}$ and the modulus $p=5$, the program computes the polynomial:
>```
>P(x) = 4 + 3x + 2x^2 (mod 5)
## Output
>```
>Lagrange Interpolation Polynomial Coefficients (mod 5):
>x^0: 4
>x^1: 3
>x^2: 2
- This corresponds to the polynomial $P(x)=4+3x+2x^2 mod 5$.
## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
## Acknowledgments
- Rust

## Clone the repository:

   ```bash
git clone https://github.com/cypriansakwa/Lagrange_Interpolation_over_Finite_Fields_in_Rust
   cd Lagrange_Interpolation_over_Finite_Fields_in_Rust
