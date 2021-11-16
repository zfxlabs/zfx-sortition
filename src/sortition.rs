//extern crate test;
use crate::binomial;
use rug::integer::Order;
use rug::{Float, Integer};

/// select runs the sortition function and returns the number of time the key was selected
pub fn select(money: u64, total_money: u64, expected_size: f64, vrf_output: &[u8; 32]) -> u64 {
    let binomial_n: f64 = money as f64;
    let binomial_p: f64 = expected_size / (total_money as f64);

    // Unsigned big-endian from bytes.
    let t: Integer = Integer::from_digits(&vrf_output[..], Order::Msf);

    let precision: u32 = (8 * (vrf_output.len() + 1)) as u32;
    let max_int = Integer::from_str_radix(
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        16, // Ensure base is 16.
    )
    .unwrap();
    let max = Float::with_val(precision, max_int);

    let h: Float = Float::with_val(precision, t);
    let cratio = (h / max).to_f64();

    return sortition_binomial_cdf_walk(binomial_n, binomial_p, cratio, money);
}

fn sortition_binomial_cdf_walk(n: f64, p: f64, ratio: f64, money: u64) -> u64 {
    for j in 0..money {
        // Get the cdf
        let boundary: f64 = binomial::cdf(n, p, j as f64);

        // Found the correct boundary, break
        if ratio <= boundary {
            return j;
        }
    }
    return money;
}

#[cfg(test)]
mod tests {

    use super::*;
    use rand::Rng;
    use serde::Deserialize;
    use serde_json;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct SelectTestCase {
        vrf: [u8; 32],
        selected: u64,
    }

    #[test]
    fn test_sortition_basic() {
        let mut hitcount: u64 = 0;
        const N: u64 = 1000;
        const EXPECTED_SIZE: u64 = 20;
        const MY_MONEY: u64 = 100;
        const TOTAL_MONEY: u64 = 200;

        for _i in 0..N {
            let vrf_output = rand::thread_rng().gen::<[u8; 32]>(); // Random 32byte byte slice
            let selected = select(MY_MONEY, TOTAL_MONEY, EXPECTED_SIZE as f64, &vrf_output);
            hitcount += selected;
        }

        let expected: u64 = N * (EXPECTED_SIZE / 2);
        let diff = expected as i64 - hitcount as i64;
        let d: u64 = num::abs(diff) as u64;

        let maxd = expected / 50;
        if d > maxd {
            panic!(
                "wanted {:?} selections but got {:?}, d={:?}, maxd={:?}",
                expected, hitcount, d, maxd
            );
        }
    }
    #[test]
    fn test_sortition_previously_failed_input() {
        test_sortition_fixed_input("./src/failed01.json");
    }
    #[test]
    fn test_sortition_previously_failed_input_2() {
        test_sortition_fixed_input("./src/failed02.json");
    }

    #[test]
    fn test_sortition_single_select() {
        const EXPECTED_SIZE: u64 = 20;
        const MY_MONEY: u64 = 100;
        const TOTAL_MONEY: u64 = 200;
        let vrf_output = b"12345678901234567890123456789012";
        let selected = select(MY_MONEY, TOTAL_MONEY, EXPECTED_SIZE as f64, &vrf_output);
        assert_eq!(7, selected);
    }

    fn test_sortition_fixed_input(filename: &str) {
        let data = fs::read_to_string(filename).expect("Unable to read file");
        let testcases: Vec<SelectTestCase> = serde_json::from_str(&data).expect("Unable to parse");

        for tc in testcases {
            test_sortition_single_fixed_input(tc);
        }
    }

    fn test_sortition_single_fixed_input(t: SelectTestCase) {
        const EXPECTED_SIZE: u64 = 20;
        const MY_MONEY: u64 = 100;
        const TOTAL_MONEY: u64 = 200;
        let selected = select(MY_MONEY, TOTAL_MONEY, EXPECTED_SIZE as f64, &t.vrf);
        assert_eq!(selected, t.selected);
    }
}
