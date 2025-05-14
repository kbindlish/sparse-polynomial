/// This program demonstrates the usage of the `SparsePolynomial` struct from the `sparse_polynomial` crate
/// with the `Fq` field element type from the `ark_test_curves::bls12_381` module. The following operations
/// are performed:
///
/// 1. **Polynomial Initialization**: A sparse polynomial is created with specified terms (coefficients and degrees).
/// 2. **Printing Polynomial**: The polynomial is printed to the console.
/// 3. **Degree Calculation**: The degree of the polynomial is calculated and printed.
/// 4. **Evaluation**: The polynomial is evaluated at `x = 2` and the result is printed.
/// 5. **Scalar Multiplication**: The polynomial is multiplied by a scalar value (2) and the result is printed.
/// 6. **Scalar Division**: The polynomial is divided by a scalar value (2) and the result is printed.
/// 7. **Chained Operations**: A series of operations (multiplication and division by 2) are chained and the result is printed.
/// 8. **Multiple Evaluations**: Multiple polynomials are evaluated at `x = 2` and the results are printed.
///
use ark_test_curves::bls12_381::Fq;
use sparse_polynomial::SparsePolynomial;

fn main() {
    // Define some co-efficients and degrees for the polynomial
    let terms = vec![
        (0, Fq::from(1u64)),
        (1, Fq::from(2u64)),
        (3, Fq::from(3u64)),
    ];
    let poly = SparsePolynomial::new(terms);

    // Print the polynomial
    println!("Polynomial: {:?}", poly);

    // Print the degree of the polynomial
    println!("Degree: {}", poly.degree().unwrap_or(0));

    // Evaluate the polynomial at x = 2
    let x = Fq::from(2u64);
    println!("Evaluation at {}: {}", x, poly.evaluate(x));

    // Multiply the polynomial by 2
    let mut poly_clone = poly.clone();
    poly_clone.multiply_by_scalar(Fq::from(2u64));
    println!("After multiplication by 2: {:?}", poly_clone);

    // Divide the polynomial by 2
    poly_clone.divide_by_scalar(Fq::from(2u64));
    println!("After division by 2: {:?}", poly_clone);

    // Chain operations on the polynomial
    let operations = vec![('*', Fq::from(2u64)), ('/', Fq::from(2u64))];
    println!(
        "Chained operations result: {}",
        poly.chain_operations(operations, x)
    );

    // Evaluate multiple polynomials at x = 2
    let polys = vec![poly.clone(), poly_clone];
    println!(
        "Multiple evaluations: {:?}",
        SparsePolynomial::evaluate_multiple(polys, x)
    );
}
