use num_bigint::BigInt;
use num_traits::{One, Zero};

/// Function to compute the modular inverse using the extended Euclidean algorithm
fn mod_inverse(a: &BigInt, p: &BigInt) -> BigInt {
    let mut t = BigInt::zero();
    let mut new_t = BigInt::one();
    let mut r = p.clone();
    let mut new_r = a.clone();
    
    while !new_r.is_zero() {
        let quotient = &r / &new_r;
        
        t = t - &quotient * &new_t;
        std::mem::swap(&mut t, &mut new_t);
        
        r = r - &quotient * &new_r;
        std::mem::swap(&mut r, &mut new_r);
    }
    
    if r > BigInt::one() {
        panic!("a is not invertible");
    }
    if t < BigInt::zero() {
        t = t + p;
    }
    
    t
}

/// Function to multiply two polynomials modulo p
fn poly_mul_mod(a: &[BigInt], b: &[BigInt], p: &BigInt) -> Vec<BigInt> {
    let mut result = vec![BigInt::zero(); a.len() + b.len() - 1];
    
    for (i, a_coeff) in a.iter().enumerate() {
        for (j, b_coeff) in b.iter().enumerate() {
            result[i + j] = (&result[i + j] + (a_coeff * b_coeff)) % p;
        }
    }
    
    result
}

/// Function to compute the Lagrange Interpolation Polynomial modulo p
fn lagrange_interpolation(x_vals: &[BigInt], y_vals: &[BigInt], p: &BigInt) -> Vec<BigInt> {
    let mut result = vec![BigInt::zero(); x_vals.len()];
    
    for (i, (x_i, y_i)) in x_vals.iter().zip(y_vals.iter()).enumerate() {
        let mut numerator = vec![BigInt::one()];  // Start with polynomial "1"
        let mut denominator = BigInt::one();
        
        for (j, x_j) in x_vals.iter().enumerate() {
            if i != j {
                numerator = poly_mul_mod(&numerator, &[(-x_j + p) % p, BigInt::one()], p);
                denominator = (denominator * (x_i - x_j + p)) % p;
            }
        }
        
        let denominator_inv = mod_inverse(&denominator, p);
        let term = (y_i * denominator_inv) % p;
        
        // Collect the results temporarily and update numerator afterward
        let new_numerator: Vec<BigInt> = numerator.iter()
            .map(|coeff| (coeff * &term) % p)
            .collect();

        for (k, coeff) in result.iter_mut().enumerate() {
            if k < new_numerator.len() {
                *coeff = (coeff.clone() + &new_numerator[k]) % p;
            }
        }
    }
    
    result
}

fn main() {
    // Example points: S = {(0, 4), (-2, 1), (2, 3)}
    let x_vals = vec![BigInt::from(0), BigInt::from(-2), BigInt::from(2)];
    let y_vals = vec![BigInt::from(4), BigInt::from(1), BigInt::from(3)];
    let p = BigInt::from(5); // Modulus p

    let polynomial = lagrange_interpolation(&x_vals, &y_vals, &p);

    println!("Lagrange Interpolation Polynomial Coefficients (mod {}):", p);
    for (i, coeff) in polynomial.iter().enumerate() {
        println!("x^{}: {}", i, coeff);
    }
}
