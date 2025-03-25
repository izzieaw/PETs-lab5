// disable warnings for code in template -- the rust compiler is very good at spotting unused code and usually that is
// very useful but it's very noisy in the exercise template
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::fs::copy;
// Take a look at the documentation for the imports
// Standard library iteration -- https://doc.rust-lang.org/std/iter/
use std::iter;
// External crate for modular exponentiation (similar to Python3 pow(base, exponent, modulus) -- https://docs.rs/mod_exp/latest/mod_exp/fn.mod_exp.html
use mod_exp::mod_exp;

type Poly = Vec<i128>;

const BIT_SIZE: i128 = 4;

// Generates a random polynomial
fn gen_random_poly(degree: i128, m: i128) -> Poly {
    use rand::Rng;
    let mut poly: Poly = Poly::new();

    let mut rng = rand::thread_rng();

    for _ in 0..=degree {
        let c = rng.gen_range(0..2 ^ BIT_SIZE);
        poly.push(c % m)
    }
    poly
}

// #####################################################
// Task 1 -- Evaluate a polynomial P at value a modulus m.
//           For instance, 10 * x^3 + 7 * x + 8 evaluated at 2 is 102.
//           We define the polynomial as a vector that contains the polynomial's coefficients.
//           The polynomial 10 * x^3 + 7 * x + 8 can be represented as: P = [10, 0, 7, 8].
//           We will use this representation of polynomials for all tasks in the lab.
//           Remember to work modulus m in your implementation!
//           The mod_exp crate is provided for you to use in this function.
//       *** Important: Do not use any library to evaluate the polynomial.
fn polynomial_evaluation(p: &Poly, a: i128, m: i128) -> i128 {
    let mut result: i128 = 0;

    for i in 0..=p.len()-1 {
        let power = mod_exp(a, (p.len() - 1 - i) as i128, m);
        let new = p[i] * power;
        result += new
    }

    result % m
}

// #####################################################
// Task 2 -- Compute the addition of two polynomials modulus m.
//           To add two polynomials with vector representation P_1 and P_2, you can add the two vectors with each other
//           component-wise (i.e., i-th element in first vector is added with i-th vector in the second vector) in Z_m.
//           For example (ignoring the modulus for now):
//           (10 * x^3 + 7 * x + 8)+(2 * x^2 + 1) = 10 * x^3 + 2 * x^2 + 7 * x + 9
//           Just like in Task 1, we define each polynomial as a vector that contains the polynomial's coefficients.
//           The polynomials above can be represented as P_1 = [10, 0, 7, 8] and P_2 = [2, 0, 1].
//           Their sum is equal to [10, 2, 7, 9] and can be calculated as:
//           [10, 0, 7, 8] + [0, 2, 0, 1].
//           Remember to work modulus m in your implementation!
//       *** Important: Do not use any library to calculate the sum of the two polynomials.

fn polynomial_addition(p_1: &Poly, p_2: &Poly, m: i128) -> Poly {
    let mut result: Poly = Poly::new();
    
    let len = p_1.len() as isize - p_2.len() as isize;
    let mut p1 = p_1.clone(); // have to do this as p_1, p_2 are unmutable
    let mut p2 = p_2.clone();


    // pad p_2 so it's same size as p_1
    if len > 0 {
        let lenp2 = len as usize;
        for i in 0..lenp2{
            p2.insert(0, 0);
        }
    }
    // pad p_1 so it's same size as p_2
    if len < 0 {
        let lenp1 = len.abs() as usize;
        for i in 0..lenp1{
            p1.insert(0, 0);
        }
    }


    for i in 0..p1.len() {
        let sum = (p1[i] + p2[i]) % m;
        result.push(sum);
    }

    result
}

// #####################################################
// Task 3 -- Compute the product (or multiplication) of two polynomials modulus m.
//           For example (ignoring the modulus for now):
//           (10 * x^3 + 7 * x + 8)*(2 * x^2 + 1) = 20 * x^5 + 24 * x^3 + 16 * x^2 + 7 * x + 8.
//           Just like in Task 1, we define each polynomial as a vector that contains the polynomial's coefficients.
//           The polynomials above can be represented as P_1 = [10, 0, 7, 8] and P_2 = [2, 0, 1].
//           Their product is equal to [20, 0, 24, 16, 7, 8].
//           Remember to work modulus m in your implementation!
//       *** Important: Do not use any library to calculate the product of two polynomials.

fn polynomial_multiplication(p_1: &Poly, p_2: &Poly, m: i128) -> Poly {
    let mut result: Poly = Poly::new();

    let len = p_1.len() as isize - p_2.len() as isize;
    let mut p1 = p_1.clone(); // have to do this as p_1, p_2 are unmutable
    let mut p2 = p_2.clone();


    // pad p_2 so it's same size as p_1
    if len > 0 {
        let lenp2 = len as usize;
        for i in 0..lenp2{
            p2.insert(0, 0);
        }
    }
    // pad p_1 so it's same size as p_2
    if len < 0 {
        let lenp1 = len.abs() as usize;
        for i in 0..lenp1{
            p1.insert(0, 0);
        }
    }

    let result_len = (p1.len() * 2) - 1;

    for i in 0..result_len {
        let mut test = 0;
        for j in 0..=i {
            if j < p1.len() && (i - j) < p1.len() {
                test = (test + p1[j] * p2[i - j]) % m;
            }
        }
        result.push(test);
    }

    // get rid of 0s at beginning of result
    while result.len() > 1 && result[0] == 0 {
        result.remove(0);
    }
  
    result
}

// #####################################################
// TASK 4 -- Represent the set S of n elements as a polynomial P, such that the roots of P are the elements of the set S
//           Return the coefficients of the polynomial as a vector of coefficients modulus m.
//           For example, when S = {2, 3}, then P = (x-2)*(x-3) = x^2 - 5 * x + 6 and the function returns [1, -5, 6]
//           Remember to work modulus m in your implementation!
//           Hint: You can make use of your `polynomial_multiplication` function.
fn polynomial_representation(s: &Vec<i128>, m: i128) -> Poly {
    let mut result: Poly = Poly::new();

    // initialise s.len() polynomials
    let mut polynomials: Vec<Poly> = Vec::new();
    for _ in s.iter() {
        let poly = Poly::new();
        polynomials.push(poly);
    }

    for i in 0..s.len() {
        polynomials[i].push(1);
        polynomials[i].push(m - s[i]);
    }

    // make individual polynomials e.g. x-2 immutable
    let mut poly_refs: Vec<&Poly> = Vec::new();
    for i in 0..s.len() {
        let poly_ref: &Poly = &polynomials[i];
        poly_refs.push(poly_ref);
    }

    let one_poly: Poly = vec![0, 1];
    let mut result = polynomial_multiplication(poly_refs[0], &one_poly, m);

    for i in 1..s.len() {
        result = polynomial_multiplication(&result, poly_refs[i], m)
    }

    result
}

// #####################################################
// TASK 5 -- Bring everything together to compute the private intersection of two sets belonging to two parties, Alice
//           and Bob, using polynomial representation modulus m.
//           Assume set A belongs to Alice and set B belongs to Bob.
//           Using the functions you created in the previous tasks and the provided `gen_random_poly` function you will need to follow these steps:
//            * Assume set A belongs to Alice and set B belongs to Bob.
//               1 - represent A as a polynomial, say P_1
//               2 - generate a random polynomial, say R_1, with the same degree as P_1
//               3 - compute the product of P_1 and R_1 (i.e., P_1 * R_1)
//
//               4 - represent B as a polynomial, say P_2
//               5 - generate a random polynomial, say R_2, with the same degree as P_2
//               6 - compute the product of P_2 and R_2 (i.e., P_2 * R_2)
//
//               7 - compute Res = P_1 * R_1 + P_2 * R_2
//               8 - evaluate Res to obtain the set intersection of A and B

fn comp_intersection(a: Vec<i128>, b: Vec<i128>, m: i128) -> Vec<i128> {
    let mut result: Poly = Poly::new();
    
    // 1.
    let p_1 = polynomial_representation(&a, m);
    // 2.
    let r_1 = gen_random_poly(p_1.len() as i128, m);
    // 3.
    let product_1 = polynomial_multiplication(&p_1, &r_1, m);

    // 4.
    let p_2 = polynomial_representation(&b, m);
    // 5.
    let r_2 = gen_random_poly(p_2.len() as i128, m);
    // 6.
    let product_2 = polynomial_multiplication(&p_2, &r_2, m);

    // 7.
    let res = polynomial_addition(&product_1, &product_2, m);
    // 8.
    for i in 0..a.len() {
        if polynomial_evaluation(&res, a[i], m) == 0 {
            result.push(a[i]);
        }
    }


    result
}

// ###########################################################
// # TASK Q1 -- Answer the following questions regarding your implementation
//
// Consider the protocol you implemented:
//      * party A computes R_1 * P_1 and sends the result to B
//      * party B computes R_2 * P_2 and sends the result to A
//      * each party computes Res = R_1 * P_1 + R_2 * P_2
//      * each party find the intersection, by evaluating every element of its set at Res and considering the element
//        in the intersection if the evaluation is zero.
// Is the above secure?
// Explain your answer in detail, why it is (not) secure.
//

// ANSWER:
// 
// Multiplying their respective polynomials p_1, p_2 by random polynomials doesnt change the roots, so the polynomial mult. that they send to each other will still contain their secrets.
// The security depends on the degree of (both p_i and r_i) polynomials and if A and B know anything about the number of secrets the other party has.
// As r_i have the same degree as their respective p_i, at face value this gives the other party a 50% chance at correctly guessing their secrets. 
// 


fn main() {
    println!("It works")
}

#[cfg(test)]
mod tests {
    use super::Poly;
    use rstest::rstest;

    const MODULUS: i128 = 868331761881;

    #[rstest]
    #[case(vec![1], 1000, 1)]
    #[case(vec![1, 2], 10, 12)]
    #[case(vec![5, 8, 1], 1000, 5008001)]
    #[case(vec![1, 1, 1, 1, 1, 1, 1, 1], 10000000000, 17795231858)]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 868331761881188649551819440127999999, 569771949032)]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 8683317618811886495518194401279999999, 404799182744)]
    fn test_polynomial_evaluation(
        #[case] polynomial: Poly,
        #[case] value: i128,
        #[case] expected: i128,
    ) {
        use super::polynomial_evaluation;
        let result = polynomial_evaluation(&polynomial, value, MODULUS);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(vec![1], vec![1], vec![2])]
    #[case(vec![10, 0, 7, 8], vec![2, 0, 1], vec![10, 2, 7, 9])]
    #[case(vec![1, 1, 1, 1, 1, 1, 1, 1], vec![MODULUS, MODULUS-1], vec![1, 1, 1, 1, 1, 1, 1, 0])]
    fn test_polynomial_addition(
        #[case] polynomial1: Poly,
        #[case] polynomial2: Poly,
        #[case] expected: Poly,
    ) {
        use super::polynomial_addition;
        let result = polynomial_addition(&polynomial1, &polynomial2, MODULUS);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(vec![1], vec![1], vec![1])]
    #[case(vec![1, 1, 1], vec![2], vec![2, 2, 2])]
    #[case(vec![1, 1, 1], vec![1, 1], vec![1, 2, 2, 1])]
    #[case(vec![1, 2], vec![2, 3], vec![2, 7, 6])]
    #[case(vec![10, 0, 7, 8], vec![2, 0, 1], vec![20, 0, 24, 16, 7, 8])]
    #[case(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![MODULUS - 1],
    vec![MODULUS - 1, MODULUS - 2, MODULUS - 3, MODULUS - 4, MODULUS - 5, MODULUS - 6, MODULUS - 7, MODULUS - 8])]
    fn test_polynomial_multiplication(
        #[case] polynomial1: Poly,
        #[case] polynomial2: Poly,
        #[case] expected: Poly,
    ) {
        use super::polynomial_multiplication;
        let result = polynomial_multiplication(&polynomial1, &polynomial2, MODULUS);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(vec![1], vec![1, MODULUS - 1])]
    #[case(vec![2, 3], vec![1, MODULUS - 5, 6])]
    #[case(vec![1, 2, 3, 4, 5], vec![1, 868331761866, 85, 868331761656, 274, 868331761761])]
    fn test_polynomial_representation(#[case] set: Vec<i128>, #[case] expected: Poly) {
        use super::polynomial_representation;
        let result = polynomial_representation(&set, MODULUS);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(vec![1], vec![1])]
    #[case(vec![1, 19, 10], vec![1, 2])]
    #[case(vec![1, 19, 10], vec![19, 1, 10])]
    #[case(vec![1, 19, 10, 4], vec![19, 1, 10, 80])]
    #[case(vec![9, 8, 2, 1, 100, 3, 44, 4, 234, 21, 45, 33],  vec![4, 10, 55, 23, 8, 12, 30, 89, 6, 102])]
    #[case(vec![9, 8, 2, 1, 100, 3, 44, 4, 234, 21, 45, 33],  vec![9, 8, 2, 1, 100, 3, 44, 4, 234, 21, 45, 33])]
    fn test_comp_intersection(#[case] a: Vec<i128>, #[case] b: Vec<i128>) {
        use super::comp_intersection;

        let mut intersection: Vec<i128> = Vec::new();
        for i in a.iter() {
            if b.contains(&i) {
                intersection.push(*i);
            }
        }

        let result = comp_intersection(a, b, MODULUS);
        assert_eq!(result, intersection)
    }
}
