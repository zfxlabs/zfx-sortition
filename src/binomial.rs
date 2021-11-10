//extern crate num;
use num::{FromPrimitive, ToPrimitive};
use statrs::function::beta::beta_inc;

// Error handling
// https://www.boost.org/doc/libs/1_74_0/libs/math/doc/html/math_toolkit/pol_ref/error_handling_policies.html
#[derive(Debug)]
// FIXME: this is a complete module
enum ErrorPolicy {
    ThrowOnError, // throw an exception.
                  //ErrNoOnError, // set ::errno & return 0, NaN, infinity or best guess.
                  //IgnoreError, // return 0, NaN, infinity or best guess.
                  //UserError,    // call a user-defined error handler.
}

// Equivalent of policies::raise_domain_error
// Raised when more or more arguments are outside the defined range of the function.
// Defaults to boost::math::policies::domain_error<throw_on_error>
// When the action is set to throw_on_error then throws std::domain_error
fn raise_domain_error<R: std::fmt::Debug>(
    function: &str,
    message: &str,
    val: R,
    _policy: &ErrorPolicy,
) {
    panic!("{}:{}:{:?}", function, message, val);
    //FIXME: implement this logic, see: ./include/boost/math/policies/error_handling.hpp (boost)
    //return val;
}

// FIXME: implement boost::math::isfinite
fn isfinite<R>(_n: &R) -> bool {
    true
}

// FIXME: this is dummy behaviour check how binomial.hpp Policy() works
fn policy() -> ErrorPolicy {
    ErrorPolicy::ThrowOnError
}

fn check_n<R: std::fmt::Debug + std::cmp::PartialOrd + FromPrimitive>(
    function: &str,
    n: R,
    policy: &ErrorPolicy,
) -> bool {
    if (n < FromPrimitive::from_u64(0).unwrap()) || !isfinite(&n) {
        raise_domain_error(
            function,
            "Number of Trials argument is %1%, but must be >= 0 !",
            n,
            policy,
        );
        return false;
    }
    return true;
}

fn check_success_fraction<R: std::fmt::Debug + std::cmp::PartialOrd + FromPrimitive>(
    function: &str,
    p: R,
    policy: &ErrorPolicy,
) -> bool {
    if p < FromPrimitive::from_u64(0).unwrap()
        || p > FromPrimitive::from_u64(1).unwrap()
        || !isfinite(&p)
    {
        raise_domain_error(
            function,
            "Success fraction argument is %1%, but must be >= 0 and <= 1 !",
            p,
            policy,
        );
        return false;
    }
    return true;
}

fn check_dist<R: std::fmt::Debug + std::cmp::PartialOrd + FromPrimitive>(
    function: &str,
    n: R,
    p: R,
    policy: &ErrorPolicy,
) -> bool {
    check_success_fraction(function, p, policy) && check_n(function, n, policy)
}

fn check_dist_and_k<R: Copy + std::fmt::Debug + std::cmp::PartialOrd + FromPrimitive>(
    function: &str,
    n: R,
    p: R,
    k: R,
    policy: &ErrorPolicy,
) -> bool {
    if check_dist(function, n, p, policy) == false {
        return false;
    }
    if k < FromPrimitive::from_u64(0).unwrap() || !isfinite(&k) {
        raise_domain_error(
            function,
            "Number of Successes argument is %1%, but must be >= 0 !",
            k,
            policy,
        );
        return false;
    }
    if k > n {
        raise_domain_error(
            function,
            "Number of Successes argument is %1%, but must be <= Number of Trials !",
            k,
            policy,
        );
        return false;
    }
    return true;
}

/// cdf - Cumulative Distribution Function Binomial.
/// The random variate k is the number of successes in n trials.
/// k argument may be integral, signed, or unsigned, or floating point.
/// If necessary, it has already been promoted from an integral type.
/// Returns the sum of the terms 0 through k of the Binomial Probability Density/Mass:
///
///   i=k
///   --  ( n )   i      n-i
///   >   |   |  p  (1-p)
///   --  ( i )
///   i=0
/// The terms are not summed directly instead
/// the incomplete beta integral is employed,
/// according to the formula:
/// P = I[1-p]( n-k, k+1).
///   = 1 - I[p](k + 1, n - k)
pub fn cdf<
    R: Copy
        + std::fmt::Debug
        + std::cmp::PartialOrd
        + FromPrimitive
        + ToPrimitive
        + std::ops::Sub<Output = R>
        + std::ops::Add<Output = R>,
>(
    dist: &Binomial<R>,
    k: R,
) -> R {
    //FIXME: dummy
    let n: R = dist.trials();
    let p: R = dist.success_fraction();

    // Panics if something's wrong
    check_dist_and_k(
        "boost::math::cdf(binomial_distribution<%1%> const&, %1%)",
        n,
        p,
        k,
        &policy(),
    );
    if k == n {
        return FromPrimitive::from_u64(1).unwrap();
    }
    if p == FromPrimitive::from_u64(0).unwrap() {
        return FromPrimitive::from_u64(1).unwrap();
    }
    if p == FromPrimitive::from_u64(1).unwrap() {
        return FromPrimitive::from_u64(0).unwrap();
    }

    let beta_res = beta_inc(
        (k + FromPrimitive::from_u64(1).unwrap()).to_f64().unwrap(),
        (n - k).to_f64().unwrap(),
        p.to_f64().unwrap(),
    );
    return FromPrimitive::from_f64(beta_res).unwrap();
    //return beta_inc(k + 1, n - k, p);
}

#[derive(Debug)]
pub struct Binomial<R>
//where
//R: std::fmt::Debug + std::cmp::PartialOrd + FromPrimitive,
{
    m_n: R, // FIXME: should be int?
    m_p: R, // success_fraction
}

impl<R> Binomial<R>
where
    R: Copy
        + std::fmt::Debug
        + std::cmp::PartialOrd
        + FromPrimitive
        + ToPrimitive
        + std::ops::Sub<Output = R>
        + std::ops::Add<Output = R>,
{
    pub fn new(n: R, p: R) -> Binomial<R> {
        // Check will panic if checks fail
        check_dist("binomial_distribution", n, p, &policy());
        Binomial { m_n: n, m_p: p }
    }

    pub fn success_fraction(&self) -> R {
        self.m_p
    }

    pub fn trials(&self) -> R {
        self.m_n
    }
}
