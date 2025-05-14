use ark_ff::{Field, Zero, One};
use ark_test_curves::bls12_381::Fq;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SparsePolynomial {
    terms: HashMap<usize, Fq>, //Map to store indexes and coefficients (Fq)
}

impl SparsePolynomial {
    // Generates a new SparsePolynomial from a vector of (index, coefficient) pairs
    // O(n)
    pub fn new(terms: Vec<(usize, Fq)>) -> Self {
        let mut map = HashMap::new();
        for (index, coeff) in terms {
            if !coeff.is_zero() {
                map.insert(index, coeff);
            }
        }
        SparsePolynomial { terms: map }
    }

    // Returns the largest degree of the polynomial, or None if there are no terms
    pub fn degree(&self) -> Option<usize> {
        self.terms.keys().max().cloned()
    }

    // Returns an evaluation of the polynomial at a given (Field) value
    pub fn evaluate(&self, x: Fq) -> Fq {
        let mut result = Fq::zero();
        for (&index, &coeff) in self.terms.iter() {
            result += coeff * x.pow(&[index as u64]);
        }
        result
    }

    
    // Returns an evaluation of the polynomial at a given (Field) value using Horner's method
    // This method is commented out because it is not used in the example
    // Wanted to implement this function without power operations but its not working as expected
    pub fn evaluate_horners_method(&self, x: Fq) -> Fq {
        let mut result = Fq::zero();
        println!("Initial result: {}", result);
        let mut keys: Vec<_> = self.terms.keys().collect();
        println!("Keys: {:?}", keys);
        keys.sort();
        println!("Sorted keys: {:?}", keys);
        for &index in keys.iter() {
            result *= x;
            if let Some(&coeff) = self.terms.get(index) {
                result += coeff;
                println!("Result after index {}, {}: {}", index, coeff, result);
            }
        }
        result
    }
    

    pub fn evaluate_pre_compute_powers(&self, x: Fq) -> Fq {
        let mut powers = vec![Fq::one()];
        for _ in 1..=self.degree().unwrap() {
            powers.push(*powers.last().unwrap() * x);
        }
    
        let mut result = Fq::zero();
        for (&index, &coeff) in self.terms.iter() {
            result += coeff * powers[index];
        }
        result
    }
    
    // Multiplies (element-wise) every coefficient of the polynomial by a given (Field) value
    pub fn multiply_by_scalar(&mut self, scalar: Fq) {
        for coeff in self.terms.values_mut() {
            *coeff *= scalar;
        }
    }

    // Divides (element-wise) every coefficient of the polynomial by a given (Field) value
    pub fn divide_by_scalar(&mut self, scalar: Fq) {
        let inv_scalar = scalar.inverse().unwrap();
        for coeff in self.terms.values_mut() {
            *coeff *= inv_scalar;
        }
    }

    // Chains a series of operations (multiplication and division by a scalar) and returns the evaluation of the resulting polynomial at a given (Field) value
    pub fn chain_operations(&self, operations: Vec<(char, Fq)>, x: Fq) -> Fq {
        let mut result = self.clone();
        for (op, value) in operations {
            match op {
                '*' => result.multiply_by_scalar(value),
                '/' => result.divide_by_scalar(value),
                _ => panic!("Unsupported operation"),
            }
        }
        result.evaluate(x)
    }

    // Evaluates multiple polynomials at a given (Field) value and returns a vector of the results
    pub fn evaluate_multiple(polynomials: Vec<SparsePolynomial>, x: Fq) -> Vec<Fq> {
        polynomials
            .into_iter()
            .map(|poly| poly.evaluate(x))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let polynomial = SparsePolynomial::new(terms.clone());
        let mut expected_terms = HashMap::new();
        for (index, coeff) in terms {
            if !coeff.is_zero() {
                expected_terms.insert(index, coeff);
            }
        }
        assert_eq!(polynomial.terms, expected_terms);
    }

    #[test]
    fn test_degree() {
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        assert_eq!(poly.degree().unwrap(), 3);
    }

    #[test]
    fn test_evaluate() {
        let terms = vec![
            (0, Fq::from(5u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        let x = Fq::from(2u64);
        println!("Evaluation at {}: {}", x, poly.evaluate(x));
        assert_eq!(poly.evaluate(x), Fq::from(33u64)); // 5 + 2*2 + 3*2^3 = 5 + 4 + 24 = 33
    }

    #[test]
    fn test_evaluate_at_zero() {
        let terms = vec![
            (0, Fq::from(1u32)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        assert_eq!(poly.evaluate(Fq::from(0u64)), Fq::from(1u64));
    }

    #[test]
    fn test_evaluate_pre_compute_powers() {
        // Define a polynomial: P(x) = 1 + 2x + 3x^2
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (2, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);

        // Evaluate the polynomial at x = 2
        let x = Fq::from(2u64);
        let result = poly.evaluate_pre_compute_powers(x);

        // Expected result: P(2) = 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
        let expected = Fq::from(17u64);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate_horners_method() {
        // Define a polynomial: P(x) = 1 + 2x + 3x^2
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);

        // Evaluate the polynomial at x = 2
        let x = Fq::from(2u64);
        let result = poly.evaluate_horners_method(x);

        // Expected result: P(2) = 1 + 2*2 + 3*2^2 = 1 + 4 + 12 = 17
        let expected = Fq::from(29u64);
        println!("Horners method: {}", result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_all_zero_coefficients() {
        let terms = vec![
            (0, Fq::from(0u64)),
            (1, Fq::from(0u64)),
            (3, Fq::from(0u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        assert_eq!(poly.evaluate(Fq::from(1u64)), Fq::from(0u64));
    }

    #[test]
    fn test_large_indices() {
        let terms = vec![
            (100, Fq::from(1u64)),
            (200, Fq::from(2u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        // Evaluating at x = 1 should sum the coefficients
        assert_eq!(poly.evaluate(Fq::from(1u64)), Fq::from(3u64));
    }

    #[test]
    fn test_multiply_by_scalar() {
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let mut poly = SparsePolynomial::new(terms);
        poly.multiply_by_scalar(Fq::from(2u64));
        let expected_terms = vec![
            (0, Fq::from(2u64)),
            (1, Fq::from(4u64)),
            (3, Fq::from(6u64)),
        ];
        let mut expected_map = HashMap::new();
        for (index, coeff) in expected_terms {
            expected_map.insert(index, coeff);
        }
        assert_eq!(poly.terms, expected_map);
    }

    #[test]
    fn test_divide_by_scalar() {
        let terms = vec![
            (0, Fq::from(2u64)),
            (1, Fq::from(4u64)),
            (3, Fq::from(6u64)),
        ];
        let mut poly = SparsePolynomial::new(terms);
        poly.divide_by_scalar(Fq::from(2u64));
        let expected_terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let mut expected_map = HashMap::new();
        for (index, coeff) in expected_terms {
            expected_map.insert(index, coeff);
        }
        assert_eq!(poly.terms, expected_map);
    }

    #[test]
    fn test_chain_operations() {
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly = SparsePolynomial::new(terms);
        let operations = vec![('*', Fq::from(2u64)), ('/', Fq::from(2u64))];
        let x = Fq::from(2u64);
        assert_eq!(poly.chain_operations(operations, x), poly.evaluate(x));
    }

    #[test]
    fn test_evaluate_multiple() {
        let terms = vec![
            (0, Fq::from(1u64)),
            (1, Fq::from(2u64)),
            (3, Fq::from(3u64)),
        ];
        let poly1 = SparsePolynomial::new(terms.clone());
        let mut poly2 = SparsePolynomial::new(terms);
        poly2.multiply_by_scalar(Fq::from(2u64));
        let x = Fq::from(2u64);
        let evaluations = SparsePolynomial::evaluate_multiple(vec![poly1, poly2], x);
        assert_eq!(evaluations, vec![Fq::from(29u64), Fq::from(58u64)]);
    }
}
