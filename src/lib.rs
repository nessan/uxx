#![doc = include_str!("../README.md")]

/// Returns the single unique `uxx` value that corresponds to the given combinadic index `index`.
/// The result is a `u64` but only the lower `xx` bits will be used, and exactly `s` bits of those will be set to 1.
///
/// The total number of ways we can choose `s` bits from `xx` bits is given by the binomial coefficient `C(xx, s)`.
/// This function maps each "combinadic" index in `[0, C(xx, s))` to a unique `uxx` with exactly `s` set bits in the
/// lower `xx` bits. All other bits in the result will be 0. The mapping is such that as the `index` increases, the
/// resulting `uxx` also increases in value.
///
/// For example, if `xx = 52` and `s = 13` the result will be a unique `u64` where only the lower 52 bits are used, and
/// among those, exactly 13 bits are set to 1.
///
/// # Panics
/// - If the index is out of range, meaning it is not in the interval `[0, C(xx, s))`.
/// - If the required number of set bits `s` is greater than the total number of output bits `xx`.
/// - We rely on a fast binomial function that uses a lookup table limited to `xx <= 64`. If `xx` is greater than 64, we
///   panic.
///
/// # Examples
/// ```
/// use combinadics::*;
/// assert_eq!(uxx_for_index(0, 2, 4), 0b0011);
/// assert_eq!(uxx_for_index(1, 2, 4), 0b0101);
/// assert_eq!(uxx_for_index(2, 2, 4), 0b0110);
/// ```
pub fn uxx_for_index(index: u64, s: u32, xx: u32) -> u64 {
    // Our binomial function returns uses a lookup table so it is very fast but only good for small values.
    assert!(xx <= 64, "xx must be at most 64");

    // We cannot choose more bits than we have available, so we check that s is at most xx.
    assert!(s <= xx, "s must be at most xx");

    // The index should run from 0 to C(xx, s) - 1, so we check that it is in range.
    let index_max = binomial(xx, s);
    assert!(index < index_max, "the index must be less than the max allowed: C(xx,s)");

    // Start with an output that has no bits set.
    let mut result: u64 = 0;

    // The combinadic representation of `index` is given by a unique sequence of integers d_s > d_{s-1} > ... > d_1 >= 0
    // such that index == C(d_s, s) + ... + C(d_1, 1), where s is the number of set bits. We can find these d_s values
    // by means of a greedy algorithm, starting from the largest digit d_s down to d_1.

    // How much of the index is left to account for.
    let mut remainder = index;

    // We know for sure that `index < C(xx, s)`
    let mut d_max = xx;

    // iteratively starting from s down to 1.
    for digit in (1..=s).rev() {
        // Find `d_lo` such that C(d_lo,digit) <= remainder < C(d_lo+1,digit) using a binary search.

        // We know that C(digit-1, digit) == 0, so we can start the binary search from digit-1 on the left.
        let mut d_lo = digit - 1;

        // We know that C(d_max, digit) > remainder, so we can start the binary search from d_max-1 on the right.
        let mut d_hi = d_max - 1;

        // Binary search to find the largest d such that C(d, digit) <= remainder.
        while d_lo < d_hi {
            let mid = d_lo + (d_hi - d_lo + 1) / 2;
            let binom = binomial(mid, digit);
            if binom <= remainder {
                // We can jump the lower bound up to mid, since we know that C(mid, digit) <= remainder.
                d_lo = mid;
            }
            else {
                // We can jump the upper bound down to mid-1, since we know that C(mid, digit) > remainder.
                d_hi = mid - 1;
            }
        }

        // At this point, d_lo == d_hi and C(d_lo, digit) <= remainder < C(d_lo+1, digit).
        // Therefore `d_lo` is the digit we are looking for, and we set that bit in the result.
        result |= 1u64 << d_lo;

        // Update the remainder and d_max for the next iteration.
        remainder -= binomial(d_lo, digit);
        d_max = d_lo;
    }

    result
}

/// Returns the unique combinadic index corresponding to the passed `uxx` value.
///
/// The input is a `u64` but we only consider the lower `xx` bits of that input. The result is a non-negative integer
/// less than `C(xx, s)`, where `C` is the binomial coefficient, and `s` is the number of set bits in the lower `xx`
/// bits of the input.
///
/// This is the inverse of [`uxx_for_index`].
///
/// # Panics
/// We rely on a fast binomial function that uses a lookup table limited to `xx <= 64`. If `xx` is greater than 64, we
/// panic.
///
/// # Examples
/// ```
/// use combinadics::*;
/// assert_eq!(index_for_uxx(0b0011, 4), 0);
/// assert_eq!(index_for_uxx(0b0101, 4), 1);
/// assert_eq!(index_for_uxx(0b0110, 4), 2);
/// ```
pub fn index_for_uxx(uxx: u64, xx: u32) -> u64 {
    // Our binomial function relies on a lookup table that supports xx <= 64.
    assert!(xx <= 64, "xx must be at most 64");

    // The active region of the input is the lower `xx` bits.
    let mut uxx = uxx;
    if xx < 64 {
        uxx = uxx & ((1u64 << xx) - 1);
    }

    // If set bits are at positions d_1 < d_2 < ... < d_s, then index = sum_{k=1}^s C(d_k, k).
    let mut index = 0u64;
    let mut digit = 1u32;
    while uxx != 0 {
        let d = uxx.trailing_zeros() as u32;
        index += binomial(d, digit);
        uxx &= uxx - 1;
        digit += 1;
    }

    index
}

// --------------------------------------------------------------------------------------------------------------------
// Some specialized versions of `uxx_for_index` & `index_for_uxx` for the common cases of `uxx = u64` and `uxx = u52`.
// --------------------------------------------------------------------------------------------------------------------

/// Returns the unique `u64` corresponding to the given combinadic index `i`. It will have exactly `s` bits set to 1.
///
/// - `i` is the combinadic index, a non-negative integer less than `C(64,s)`, where `C` is the binomial  coefficient.
/// - `s` is the number of set bits in the result `u64`.
///
/// # Panics
/// Panics if `s > 64` or if `i >= C(64,s)`.
///
/// # Examples
/// ```
/// use combinadics::*;
/// let clubs = 0b1_1111_1111_1111;
/// let spades = clubs << (52 - 13);
/// assert_eq!(uxx_for_index(0, 13, 52), clubs);
/// let max_index = binomial(52, 13) as u32 - 1;
/// assert_eq!(uxx_for_index(max_index, 13, 52), spades);
/// ```
pub fn u64_for_index(i: u64, s: u32) -> u64 { uxx_for_index(i, s, 64) }

/// Returns the `u52` corresponding to the given combinadic index `i`.
/// The result is a `u64` but only the lower 52 bits will be used, and among those, exactly `s` bits will be set to 1.
///
/// - `i` is the combinadic index, a non-negative integer less than `C(52,s)`, where `C` is the binomial  coefficient.
/// - `s` is the number of set bits in the result `u52`.
///
/// # Note
/// A `u52` is a `u64` where only the lower 52 bits are used. This is useful for applications like representing subsets
/// or hands of a standard 52-card deck. The set bits in the result correspond to the chosen cards from a universe of
/// 52 cards. For example, we might use the lowest 13 bits to represent the 13 ranks of clubs, the next 13 bits for
/// diamonds, then hearts, and finally spades (Bridge order). In this case, the combinadic index `i` represents a
/// specific hand of cards, and the resulting `u52` has bits set corresponding to those cards.
///
/// # Panics
/// Panics if `s > 52` or if `i >= C(52,s)`.
///
/// # Examples
/// ```
/// use combinadics::*;
/// let clubs = 0b1_1111_1111_1111;
/// let spades = clubs << (52 - 13);
/// assert_eq!(uxx_for_index(0, 13, 52), clubs);
/// let max_index = binomial(52, 13) - 1;
/// assert_eq!(uxx_for_index(max_index, 13, 52), spades);
/// ```
pub fn u52_for_index(i: u64, s: u32) -> u64 { uxx_for_index(i, s, 52) }

/// Returns the unique `combinadic index` corresponding to the given 64-bit integer.
/// The index depends on the number of set bits in the input, and the position of those set bits.
///
/// This is the inverse of [`u64_for_index`].
///
/// # Examples
/// ```
/// use combinadics::*;
/// let clubs: u64 = 0b1_1111_1111_1111;
/// let spades: u64 = clubs << (52 - 13);
/// assert_eq!(index_for_u64(clubs), 0);
/// let spades_index = binomial(52, 13) - 1;
/// assert_eq!(index_for_u64(spades), spades_index);
/// ```
pub fn index_for_u64(u: u64) -> u64 { index_for_uxx(u, 64) }

/// Returns the unique `combinadic index` corresponding to the given 52-bit integer.
/// The input is a `u64` but only the lower 52 bits are considered. The index depends on the number of set bits in the
/// input, and the position of those set bits.
///
/// This is the inverse of [`u52_for_index`].
///
/// # Note
/// A `u52` is a `u64` where only the lower 52 bits are used. This is useful for applications like representing subsets
/// or hands of a standard 52-card deck. The set bits in the result correspond to the chosen cards from a universe of
/// 52 cards. For example, we might use the lowest 13 bits to represent the 13 ranks of clubs, the next 13 bits for
/// diamonds, then hearts, and finally spades (Bridge order). In this case, the combinadic index `i` represents a
/// specific hand of cards, and the resulting `u52` has bits set corresponding to those cards.
///
/// # Examples
/// ```
/// use combinadics::*;
/// let clubs: u64 = 0b1_1111_1111_1111;
/// let spades: u64 = clubs << (52 - 13);
/// assert_eq!(index_for_u52(clubs), 0);
/// let spades_index = binomial(52, 13) - 1;
/// assert_eq!(index_for_u52(spades), spades_index);
/// ```
pub fn index_for_u52(u: u64) -> u64 { index_for_uxx(u, 52) }

// --------------------------------------------------------------------------------------------------------------------
// Helper functions to compute the binomial coefficient `C(n,k)`` for small `n` using a precomputed table.
// --------------------------------------------------------------------------------------------------------------------

/// Returns the binomial coefficient "n choose k" for `0 <= n <= 64`.
/// This is done by a lookup in a precomputed table. The table is built at compile time using a const function, so this
/// function runs in O(1) time. However, the table only supports `n` up to 64.
///
/// # Panics
/// In debug mode, panics if `n > 64`.
///
/// # Examples
/// ```
/// use combinadics::*;
/// assert_eq!(binomial(5, 2), 10);
/// assert_eq!(binomial(10, 3), 120);
/// assert_eq!(binomial(20, 10), 184756);
/// ```
pub fn binomial(n: u32, k: u32) -> u64 {
    // Edge case: There is no possible choice of k elements from n elements if k > n.
    if k > n {
        return 0;
    }

    // Edge cases: We don't store entries for k=0 and k=n in the table, since those are always 1.
    if k == 0 || k == n {
        return 1;
    }

    // Validate input---our table only supports n up to 64.
    debug_assert!(n <= 64, "n must be at most 64");

    // Lookup in the precomputed table
    BINOMIAL_TABLE[table_index_for(n, k)]
}

// Returns the index into the BINOMIAL_TABLE for given n and k.
// Assumes n >= 2 and 1 <= k <= n-1 as we deal with the edge cases (k == 0 and k >= n) separately.
const fn table_index_for(n: u32, k: u32) -> usize {
    // Rows n = 0 and n = 1 have no stored entries. For n >= 2, store k in 1..n-1.
    (((n - 2) * (n - 1)) / 2 + (k - 1)) as usize
}

// We have precomputed the binomial coefficients for n = 2..=64 and k = 1..n-1.
// The total number of stored entries is sum_{k=2}^{64} (k-1) = 2016.
// You can index into the table using the `table_index_for` function above.
const BINOMIAL_TABLE: [u64; 2016] = build_binomial_table();

// Builds the binomial table at **compile time**.
const fn build_binomial_table() -> [u64; 2016] {
    let mut table = [0u64; 2016];
    let mut n = 0u32;
    while n <= 64 {
        if n >= 2 {
            let mut k = 1u32;
            while k < n {
                let left = if k == 1 { 1 } else { table[table_index_for(n - 1, k - 1)] };
                let right = if k == n - 1 { 1 } else { table[table_index_for(n - 1, k)] };
                table[table_index_for(n, k)] = left + right;
                k += 1;
            }
        }
        n += 1;
    }
    table
}

// --------------------------------------------------------------------------------------------------------------------
// Round-trip test to check that the `uxx_for_index` and `index_for_uxx` functions are inverses of each other.
// --------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // This test checks that for all valid combinations of `xx` and `s`, and for all valid indices, the functions
    // `uxx_for_index` and `index_for_uxx` are inverses of each other. That is, if we take an index, convert it to a
    // `uxx` using `uxx_for_index`, and then convert it back to an index using `index_for_uxx`, we should get the
    // original index back. This is a comprehensive test that covers all cases for `xx` from 0 to 16 and all valid `s`
    // for each `xx`.
    fn round_trip() {
        for xx in 0..=16 {
            for s in 0..=xx {
                let max = binomial(xx, s);
                for index in 0..max {
                    let uxx = uxx_for_index(index, s, xx);
                    assert_eq!(index_for_uxx(uxx, xx), index);
                }
            }
        }
    }
}
