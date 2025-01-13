[<img alt="points bar" align="right" height="36" src="../../blob/badges/.github/badges/points-bar.svg" />
<img alt="Workflow status" align="right" height="30" src="../../workflows/Autograding/badge.svg" />](../../actions/workflows/classroom.yml)

# COMP0061 -- Privacy Enhancing Technologies -- Lab on private set intersection

This lab will introduce the basics of working with polynomials and their use for private set intersection.

This lab requires a basic understanding of polynomials: their representation, degree, evaluation, addition, and 
multiplication.
If you are unfamiliar with these concepts, we recommend you read the following Wikipedia article 
[Wikipedia: Polynomial](https://en.wikipedia.org/wiki/Polynomial), especially the section on Arithmetic.

### Structure of Lab
This lab departs from previous labs in that you are no longer using Petlib and Python 3.
You will use the Rust programming language.
You can read more about it in the [Rust docs introduction](https://doc.rust-lang.org/book/ch00-00-introduction.html).
The documentation provides a gentle introduction to the main concepts in the language, refer to it if you're unsure how to proceed.

This lab contains one file `src/main` with two sections:
- function declarations that you need to complete.
- a series of tests that ensure your code works as required in the `tests` module (the `mod tests` block).

Note that the tests passing is a necessary but not sufficient condition to fulfill each task.
There are programs that would make the tests pass that would still be invalid (or blatantly insecure) implementations.

### Checking out code

Check out the code by using your preferred git client (e.g., git command line client, GitHub Desktop, Sourcetree).

***Alternatively***, you can use the GitHub Codespaces feature to check out and work on the code in the cloud.

### Setup
The intended environment for this lab is the Linux operating system with Rust installed.

#### Local installation
You can install the Rust compiler on your machine using [rustup](https://rustup.rs/).
It supports Unix-like (Linux, macOS) and Windows operating systems.

#### Visual Studio Code development containers

As an alternative to a local virtual environment, we provide the setup files for VSCode
 [development containers](https://code.visualstudio.com/docs/remote/containers)
which use [Docker](https://docs.docker.com/get-docker/) to create a separate development environment for each 
repository and install the required libraries.
You don't need to know how to use Docker to use development containers.

#### GitHub Codespaces

Another alternative for running your code is to use GitHub Codespaces.
On GitHub, the "<> Code" button at the top right of the repository page will have a Codespaces tab.
This allows you to create a cloud-based environment to work on the assignment.
You still need to use `git` to commit and push your work when working in a codespace.

#### GitHub Classroom tests

The tests are the same as the ones that run as part of the GitHub Classroom automated marking system,
so you can also run the tests by simply committing and pushing your changes to GitHub, without the need for a local
setup or even having Rust installed.

### Working with unit tests
Tests in Rust are run using the `cargo` utility which is installed by `rustup`.
Unit tests are run from the command line by executing the command:

```sh
$ cargo test
```

To run a specific test you can append its name after the command:

```sh
$ cargo test test_polynomial_evaluation
```

Each test uses multiple sets of inputs or cases to test.
You can run only one of these cases by appending it to the name of the test.

```sh
$ cargo test test_polynomial_evaluation::case_2
```

### What you will have to submit
The deadline for all labs is at the end of term but labs will be progressively released throughout the term, as new
concepts are introduced. 
We encourage you to attempt labs as soon as they are made available and to use the dedicated lab time to bring up any
queries with the TAs.

Labs will be checked using GitHub Classroom, and the tests will be run each
time you push any changes to the `main` branch of your GitHub repository.
The latest score from automarking should be shown in the Readme file.
To see the test runs, look at the Actions tab in your GitHub repository.

Make sure the submitted `main.rs` file at least satisfies the tests, without the need for any additional external dependencies. 
Only submissions prior to the GitHub Classroom deadline will be marked, so make sure you push your code in time.


To re-iterate, the tests passing is a necessary but not sufficient condition to fulfill each task.
All submissions will be checked by TAs for correctness and your final marks are based on their assessment of your work.  
For full marks, make sure you have fully filled in any sections marked with `TODO` comments, including answering any
questions in the comments of the `src/main.rs` file.

## TASK 1 -- Evaluating a polynomial. \[1 point\]
> Evaluate a polynomial P at value a modulus m.
> For instance, 10 * x^3 + 7 * x + 8 evaluated at 2 is 102.
> We define the polynomial as a vector that contains the polynomial's coefficients.
> The polynomial 10 * x^3 + 7 * x + 8 can be represented as: P = [10, 0, 7, 8].
> We will use this representation of polynomials for all tasks in the lab.


### Hints
- Execute the following command to ensure the tests run:
```sh
$ cargo test test_polynomial_evaluation
```
- Remember to work modulus m in your implementation!

## TASK 2 -- Add two polynomials. \[1 point\]
> To add two polynomials with vector representation P_1 and P_2, you can add the two vectors with each other
> component-wise (i.e., i-th element in first vector is added with i-th vector in the second vector) in Z_m.
> For example (ignoring the modulus for now):
> (10 * x^3 + 7 * x + 8)+(2 * x^2 + 1) = 10 * x^3 + 2 * x^2 + 7 * x + 9
> Just like in Task 1, we define each polynomial as a vector that contains the polynomial's coefficients.
> The polynomials above can be represented as P_1 = [10, 0, 7, 8] and P_2 = [2, 0, 1].
> Their product is equal to [10, 2, 7, 9] and can be calculated as:
> [10, 0, 7, 8] + [0, 2, 0, 1].

**Important**: Do not use any library to calculate the sum of two polynomials.

### Hints
- Execute the following command to ensure the tests run:
```sh
$ cargo test test_polynomial_addition
```
- Remember to work modulus m in your implementation!


## TASK 3 -- Multiply two polynomials. \[1 point\]
> Compute the product (or multiplication) of two polynomials modulus m.
> For example (ignoring the modulus for now):
> (10 * x^3 + 7 * x + 8)*(2 * x^2 + 1) = 20 * x^5 + 24 * x^3 + 16 * x^2 + 7 * x + 8.
> Just like in Task 1, we define each polynomial as a vector that contains the polynomial's coefficients.
> The polynomials above can be represented as P_1 = [10, 0, 7, 8] and P_2 = [2, 0, 1].
> Their product is equal to [20, 0, 24, 16, 7, 8].

**Important**: Do not use any library to calculate the product of two polynomials.

### Hints
- Execute the following command to ensure the tests run:
```sh
$ cargo test test_polynomial_multiplication
```
- Remember to work modulus m in your implementation!


## TASK 4 -- Represent a set as a polynomial. \[1 point\]
> Represent the set S of n elements as a polynomial P, such that the roots of P are the elements of the set S
> Return the coefficients of the polynomial as a vector of coefficients modulus m.
> For example, when S = [2, 3], then P = (x-2)*(x-3) = x^2 - 5 * x + 6 and the function returns [1, -5, 6]

**Important**: Do not use any library to calculate the product of two polynomials.

### Hints
- Execute the following command to ensure the tests run:
```sh
$ cargo test test_polynomial_representation
```
- Remember to work modulus m in your implementation!
- You can make use of your `polynomial_multiplication` function.


## TASK 5 -- Compute the private set intersection. \[1 point\]
> Bring everything together to compute the private intersection of two sets belonging to two parties, Alice 
> and Bob, using polynomial representation modulus m.
> Assume set A belongs to Alice and set B belongs to Bob.
> Using the functions you created in the previous tasks and the provided `gen_random_poly` you will need to follow these steps:


1. represent A as a polynomial, say P_1
2. generate a random polynomial, say R_1, with the same degree as P_1
3. compute the product of P_1 and R_1 (i.e., P_1 * R_1)
4. represent B as a polynomial, say P_2
5. generate a random polynomial, say R_2, with the same degree as P_2
6. compute the product of P_2 and R_2 (i.e., P_2 * R_2)
7. compute Res = P_1 * R_1 + P_2 * R_2
8. evaluate Res to obtain the set intersection of A and B


**Important**: Do not use any library to calculate the private intersection of the sets.

### Hints
- Execute the following command to ensure the tests run:
```sh
$ cargo test test_comp_intersection
```
- Remember to work modulus m in your implementation!
- Use the functions you implemented


## TASK Q1  -- Answer the questions with reference to the protocol you implemented.  \[1 point\]

- Please include the answer as part of the Code file submitted, as a multi-line string, where the `TODO` indicates.

