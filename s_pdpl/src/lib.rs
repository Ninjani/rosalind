#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate inline_python;

use anyhow::Error;
use inline_python::{pyo3, python, Context};

use std::path::Path;
use utility::io::Parseable;

pub type Polynomial = Vec<(i32, i32)>;

fn polynomial_from_sympy(sympy_polynomial: Vec<i32>) -> Polynomial {
    sympy_polynomial
        .into_iter()
        .rev()
        .enumerate()
        .filter(|(_, coef)| *coef != 0)
        .map(|(i, coef)| (i as i32, coef))
        .collect()
}

fn factorize_polynomial(polynomial: Polynomial) -> Vec<Polynomial> {
    let context = Context::new();
    python! {
        #![context = &context]
        import sympy
        from sympy.abc import x
        def polynomial_to_list(poly):
            coefficients = poly.as_coefficients_dict()
            return [(key.as_base_exp()[1], coefficients[key]) if key.as_base_exp()[0] == x else (0, coefficients[key]) for key in coefficients]

        factors = sympy.polys.factor(sum(coef*x**i for (i, coef) in 'polynomial))
        print(factors)
        factored_polynomials = []
        if factors.is_polynomial():
            coefficients = factors.as_coefficients_dict()
            factored_polynomials.append(polynomial_to_list(factors))
        elif poly.is_Pow:
            base, exp = poly.as_base_exp()
            if base[0] != x:
                factored_polynomials.append([()])
            else:
                factored_polynomials += [polynomial_to_list(base) for _ in range(exp)]
            assert factors.is_Mul
            for poly in factors.args:
                if poly.is_Pow and poly.as_base_exp()[0] != x:
                    coefficients = poly.as_coefficients_dict()
                    factored_polynomials.append([(key.as_base_exp()[1], coefficients[key]) if key.as_base_exp()[0] == x else (0, coefficients[key]) for key in coefficients])
    }
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();
    //    let factored_polynomials: Vec<Vec<(i32, i32)>> = context.get_global(py, "factored_polynomials").unwrap().unwrap_or(Vec::new());
    //    factored_polynomials
    Vec::new()
}

fn points_from_polynomial(polynomial: &Polynomial) -> Vec<i32> {
    let mut points: Vec<_> = polynomial
        .iter()
        .flat_map(|(power, coef)| (0..*coef).map(move |_| *power))
        .collect();
    points.sort();
    points
}

/// W.I.P
pub fn rosalind_pdpl(input: &str) -> Result<(), Error> {
    let difference_multiset = i32::parse_line(input)?;
    let polynomial = difference_multiset
        .into_iter()
        .enumerate()
        .map(|(_, power)| (power, 1))
        .collect();
    let factored_polynomials = factorize_polynomial(polynomial);
    println!("{:?}", factored_polynomials);
    Ok(())
}
