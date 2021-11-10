use statrs::function::beta::beta_inc;

// Equivalent of policies::raise_domain_error
// Raised when more or more arguments are outside the defined range of the function.
// Defaults to boost::math::policies::domain_error<throw_on_error>
// When the action is set to throw_on_error then throws std::domain_error
fn raise_domain_error(function: &str, message: &str, val: f64) {
    panic!("{}:{}:{:?}", function, message, val);
}

fn isfinite(_n: &f64) -> bool {
    true
}

fn check_n(function: &str, n: f64) -> bool {
    if n < 0.0 || !isfinite(&n) {
        raise_domain_error(
            function,
            "Number of Trials argument is %1%, but must be >= 0 !",
            n,
        );
        return false;
    }
    return true;
}

fn check_success_fraction(function: &str, p: f64) -> bool {
    if p < 0.0 || p > 1.0 || !isfinite(&p) {
        raise_domain_error(
            function,
            "Success fraction argument is %1%, but must be >= 0 and <= 1 !",
            p,
        );
        return false;
    }
    return true;
}

fn check_dist(function: &str, n: f64, p: f64) -> bool {
    check_success_fraction(function, p) && check_n(function, n)
}

fn check_dist_and_k(function: &str, n: f64, p: f64, k: f64) -> bool {
    if check_dist(function, n, p) == false {
        return false;
    }
    if k < 0.0 || !isfinite(&k) {
        raise_domain_error(
            function,
            "Number of Successes argument is %1%, but must be >= 0 !",
            k,
        );
        return false;
    }
    if k > n {
        raise_domain_error(
            function,
            "Number of Successes argument is %1%, but must be <= Number of Trials !",
            k,
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
pub fn cdf(dist: &Binomial, k: f64) -> f64 {
    let n: f64 = dist.trials();
    let p: f64 = dist.success_fraction();

    // Panics if something's wrong
    check_dist_and_k(
        "boost::math::cdf(binomial_distribution<%1%> const&, %1%)",
        n,
        p,
        k,
    );
    if k == n {
        return 1.0;
    }
    if p == 0.0 {
        return 1.0;
    }
    if p == 1.0 {
        return 0.0;
    }

    return beta_inc(k + 1.0, n - k, p);
}

#[derive(Debug)]
pub struct Binomial {
    m_n: f64, // FIXME: should be int?
    m_p: f64, // success_fraction
}

impl Binomial {
    pub fn new(n: f64, p: f64) -> Binomial {
        // Check will panic if checks fail
        check_dist("binomial_distribution", n, p);
        Binomial { m_n: n, m_p: p }
    }

    pub fn success_fraction(&self) -> f64 {
        self.m_p
    }

    pub fn trials(&self) -> f64 {
        self.m_n
    }
}
