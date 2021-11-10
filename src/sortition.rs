//use std::collections::binary_heap;

use crate::binomial;
use rug::integer::Order;
use rug::{Float, Integer};
//use sha2::Sha512;

// FROM VRF GO CODE:
// VrfOutput is a 64-byte pseudorandom value that can be computed from a VrfProof.
// The VRF scheme guarantees that such output will be unique
// VrfOutput [64]byte

// FROM SORTITION GO CODE:
// DigestSize is the number of bytes in the preferred hash Digest used here.
//const DigestSize = sha512.Size256

// Digest represents a 32-byte value holding the 256-bit Hash digest.
//type Digest [DigestSize]byte

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

pub fn sortition_binomial_cdf_walk(n: f64, p: f64, ratio: f64, money: u64) -> u64 {
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
}

// func BenchmarkSortition(b *testing.B) {
//     b.StopTimer()
//     keys := make([]crypto.Digest, b.N)
//     for i := 0; i < b.N; i++ {
//         rand.Read(keys[i][:])
//     }
//     b.StartTimer()
//     for i := 0; i < b.N; i++ {
//         Select(1000000, 1000000000000, 2500, keys[i])
//     }
// }

// func TestSortitionBasic(t *testing.T) {
//     partitiontest.PartitionTest(t)
//     hitcount := uint64(0)
//     const N = 1000
//     const expectedSize = 20
//     const myMoney = 100
//     const totalMoney = 200
//     for i := 0; i < N; i++ {
//         var vrfOutput crypto.Digest
//         rand.Read(vrfOutput[:])
//         selected := Select(myMoney, totalMoney, expectedSize, vrfOutput)
//         hitcount += selected
//     }
//     expected := uint64(N * expectedSize / 2)
//     var d uint64
//     if expected > hitcount {
//         d = expected - hitcount
//     } else {
//         d = hitcount - expected
//     }
//     // within 2% good enough
//     maxd := expected / 50
//     if d > maxd {
//         t.Errorf("wanted %d selections but got %d, d=%d, maxd=%d", expected, hitcount, d, maxd)
//     }
// }
