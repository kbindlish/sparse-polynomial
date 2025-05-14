## What is this implementation

# Sparse Polynomial
This program demonstrates the usage of the `SparsePolynomial` struct from the `sparse_polynomial` crate
with the `Fq` field element type from the `ark_test_curves::bls12_381` module. The following operations
are performed:

1. **Polynomial Initialization**: A sparse polynomial is created with specified terms (coefficients and degrees).
2. **Printing Polynomial**: The polynomial is printed to the console.
3. **Degree Calculation**: The degree of the polynomial is calculated and printed.
4. **Evaluation**: The polynomial is evaluated at `x = 2` and the result is printed.
5. **Scalar Multiplication**: The polynomial is multiplied by a scalar value (2) and the result is printed.
6. **Scalar Division**: The polynomial is divided by a scalar value (2) and the result is printed.
7. **Chained Operations**: A series of operations (multiplication and division by 2) are chained and the result is printed.
8. **Multiple Evaluations**: Multiple polynomials are evaluated at `x = 2` and the results are printed.

[arkworks Field interface](https://github.com/arkworks-rs/algebra/blob/master/ff/README.md#field) 

[NOTE] arithmetic over large finite fields can be quite expensive: division costs more than multiplication, which costs more than addition/subtraction.

## Hardware and Software Environment

- **Hardware**: 
    - CPU: Apple M3
    - RAM: 8GB
    - OS: macOS Sonoma 14.5

- **Rust Version**: 
    - Rust 1.81.0 (stable)

## Design Choices and Assumptions

1. **Sparse Polynomial Representation**:
     - Polynomials are represented using a hash map where keys are the exponents and values are the coefficients. This allows efficient storage and manipulation of sparse polynomials.

2. **Arithmetic Operations**:
     - Addition and subtraction of polynomials are implemented by iterating through the keys of both polynomials and performing the necessary operations on the coefficients.

3. **Assumptions**:
     - The exponents are non-negative integers.
     - The coefficients are integers.
     - The input polynomials are valid and well-formed.

4. **Error Handling**:
     - Basic error handling is implemented to manage invalid inputs and operations that result in overflow.

5. **Performance Considerations**:
     - The use of hash maps ensures average-case O(1) complexity for insertion and lookup operations, making the polynomial operations efficient even for large sparse polynomials.

6. **Testing**:
     - The solution has been tested using unit tests to ensure correctness of polynomial operations.

## Conclusion

This design leverages Rust's powerful standard library and type system to create an efficient and reliable implementation of sparse polynomial arithmetic. The use of hash maps for polynomial representation ensures that the solution is both space and time efficient, making it suitable for handling large sparse polynomials.