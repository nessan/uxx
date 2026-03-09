<div style="text-align: center;" align="center">

# `uxx` <!-- omit in toc -->

[![Latest Version][version_img]][crate_link]
[![License][license_img]][license_file]
[![Documentation][docs_img]][docs_link]
[![Crate Downloads][downloads_img]][crate_link]

</div>

## Overview

This crate provides functions that map an index to an unsigned integer with a fixed number of set bits, and vice versa. That is, one function takes an index and efficiently returns a unique unsigned integer with a specified number of set bits; the other takes an unsigned integer with a specified number of set bits and returns the equivalent index.

The two principal functions are:

| Function                           | Description                                                                                                                                                                                              |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`uxx_for_index`][`uxx_for_index`] | Given an index `i`, a required number of set bits `s`, and a bit size `xx` for the output, this function returns a `u64` with precisely `s` bits set to 1 in i lowest `xx` bits and all others set to 0. |
| [`index_for_uxx`][`index_for_uxx`] | This is the inverse function that is passed a `u64`, where only the lower `xx` bits are _active_, and returns a unique index for that `uxx`.                                                             |

A few extra convenience functions are described below.

## Motivation

Consider games that use a standard deck of 52 cards, where play starts with some number of those being given to each player. For example, in contract Bridge or Whist, each player is given 13 cards face down. In five-card draw poker, players are given 5 cards face down. The order in which the cards arrived is not relevant. The player picks up the hand and immediately sorts it. It doesn't matter if the King of Spades arrived first or third in the deal; all that matters is that it is in the hand when you pick it up.

How many 13-card hands of Bridge are there anyway? Well, there are 52 possibilities for the first card, 51 for the next, and so on. So, _if the order of arrival mattered_, the number of possible hands is 52&times;51&times;...&times;40, which is roughly 4&times;10^21. However, as we just noted, in Bridge and many other card games, we are given all the cards in a heap and have no interest in which came first or last. The hand has 13 cards, which can have arrived in any of 13&times;12&times;...&times;1 ways.

Therefore, the number of possible Bridge hands is reduced to (52&times;51&times;...&times;40) / (13&times;12&times;...&times;1), which is roughly `635` billion—still a large number, but some ten orders of magnitude smaller than if the deal order mattered.

In a nutshell, for this Bridge example, we want to efficiently index those 635 billion card hands.

That is, we want a pair of functions:

- Given an index in the valid range [0, 635 billion), we want a function that returns a unique "hand" of 13 cards,
- Given a hand of 13 cards, we want a function that returns a unique index in the valid range [0, 635 billion).

This begs the question of how we are representing a hand of cards.

A typical scheme is to use a 52-bit unsigned integer, where set bits correspond to the presence of specific cards in the hand. Perhaps we might use Bridge order, where the lowest bit is used for the two of clubs, the second-lowest for the three of clubs, and so on up to the bit at position 51, which stands for the Ace of Spades. Any predetermined correspondence between individual cards and bit positions can be used.

A full Bridge hand is represented as a single 52-bit unsigned integer with 13 bits set to 1 and the others set to 0.

OK, you probably are wondering about that hypothetical 52-bit unsigned integer? In practice, we use a 64-bit integer and consider only its lowest 52 bits.

Then, for Bridge, we want a pair of functions:

- Given an index in the valid range [0, 635 billion), we want a function that returns a unique 64-bit unsigned integer with 13 bits set to 1 in the lower 52 bits and all other bits set to 0.
- Given a 64-bit unsigned integer with 13 bits set to 1 in the lower 52 bits, we want a function that returns a unique index in the valid range [0, 635 billion).

## Principal Functions

With that motivation in mind, there are two principal functions that are provided by this crate:

| Function                           | Description                                                                                                                                                                                              |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`uxx_for_index`][`uxx_for_index`] | Given an index `i`, a required number of set bits `s`, and a bit size `xx` for the output, this function returns a `u64` with precisely `s` bits set to 1 in i lowest `xx` bits and all others set to 0. |
| [`index_for_uxx`][`index_for_uxx`] | This is the inverse function that is passed a `u64`, where only the lower `xx` bits are _active_, and returns a unique index for that "hand".                                                            |

The total number of ways we can choose `s` bits from `xx` bits is given by the binomial coefficient `C(xx,s)`. The `uxx_for_index` function maps each index in `[0, C(xx,s))` to a unique `uxx` with exactly `s` set bits in the lower `xx` bits. All other bits in the result will be 0. The mapping is such that, as the index increases, the resulting uxx also increases.

For example, if `xx = 52` and `s = 13,` the result from `uxx_for_index(i,13,52)` will be a unique `u64` in which only the lower 52 bits are used, and among those, exactly 13 bits are set to 1. That can be viewed as a representation of a hand of 13 cards.

The inverse function is `index_for_uxx,` which is passed a `u64` and the number of active bits `xx` to consider. The output will be a unique non-negative integer that is less than `C(xx,s)`, where `C` is the binomial coefficient, and `s` is the number of set bits in the lower `xx` bits of the input.

## Extra Functions

We provide a few extra convenience functions for common use-cases:

| Function                           | Description                                                                                                                                  |
| ---------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| [`u64_for_index`][`u64_for_index`] | Given an index `i`, a required number of set bits `s,` this function returns a `u64` with precisely `s` bits set to 1.                       |
| [`u52_for_index`][`u52_for_index`] | Given an index `i`, a required number of set bits `s,` this function returns a `u64` with precisely `s` bits set to 1 in the lowest 52 bits. |
| [`index_for_u64`][`index_for_u64`] | This is the inverse function for [`u64_for_index`][`u64_for_index`].                                                                         |
| [`index_for_u52`][`index_for_u52`] | This is the inverse function for [`u52_for_index`][`u52_for_index`].                                                                         |

As we noted above, a `u52` is a `u64` that uses only the lower 52 bits. This is useful for applications like representing subsets or hands of a standard 52-card deck. The set bits in the result correspond to the chosen cards from a universe of 52 cards. For example, we might use the lowest 13 bits to represent the 13 ranks of clubs, the next 13 bits for diamonds, then hearts, and finally spades (Bridge order). In this case, the index `i` represents a specific hand of cards, and the resulting `u52` has bits set corresponding to those cards.

## Combinadics

The indexing scheme is based on the combinatorial number system, also known as the combinadic system.

It relies on a piece of mathematics that proves that every non-negative integer has a _unique_ representation as a sum of binomial coefficients. That is, given a fixed number of digits `d > 0`, every non-negative integer `s` can be written as a sum:

```
s = C(s[1], 1) + C(s[2], 2) + ... + C(s[d], d)
```

where the `s[1], s[2], ..., s[d]` are unique, non-negative, and increasing, `s[d] > s[d-1] > ... > s_1 > s_1` &ge; `0`.

We interpret the `s[i]` as bit locations to be set in a `u64`.

## Why `u64`s?

The algorithm we use to go from a combinadic index `i` to its corresponding unsigned integer with `s` set bits is a classic greedy one, where we look for binomial coefficients that bracket `i`. That gives us one bit to set in our output integer, and we can then shrink `i` and repeat the process.

So, at each step, we need to compute binomial coefficients, and our implementation provides a very fast `C(n,k)` calculation when `n` &le; `64`.

| Function                      | Description                            |
| ----------------------------- | -------------------------------------- |
| [`binomial(n,k)`][`binomial`] | Returns `C(n,k)` using a table lookup. |

The function is fast because it uses a table lookup, which is restricted to `n` &le; `64`. The restriction that `xx` must be at most `64` is acceptable, as our primary use case is `xx = 52`!

## Installation

To include the `uxx` crate in your Rust project, run the following Cargo command in your project directory:

```sh
cargo add uxx
```

### Copyright and License

Copyright (c) 2026-present [Nessan Fitzmaurice][] <br>
You can use this software under the [MIT License][]

<!-- Badges -->

[crate_link]: https://crates.io/crates/uxx "Crate Link"
[docs_link]: https://docs.rs/uxx/latest/uxx "Documentation"
[docs_img]: https://img.shields.io/docsrs/uxx/latest.svg?style=for-the-badge "Documentation Display"
[downloads_img]: https://img.shields.io/crates/dv/uxx.svg?style=for-the-badge "Crate Downloads"
[license_file]: https://github.com/nessan/uxx/blob/main/LICENSE.txt "License File"
[license_img]: https://img.shields.io/crates/l/uxx.svg?style=for-the-badge "License Display"
[version_img]: https://img.shields.io/crates/v/uxx?color=f46623&style=for-the-badge "uxx version badge"

<!-- General Links -->

[MIT License]: https://opensource.org/license/mit
[Nessan Fitzmaurice]: mailto:nzznfitz+gh@icloud.com

<!-- Links to docs.rs -->

[`uxx_for_index`]: https://docs.rs/uxx/latest/uxx/fn.uxx_for_index.html
[`u64_for_index`]: https://docs.rs/uxx/latest/uxx/fn.u64_for_index.html
[`u52_for_index`]: https://docs.rs/uxx/latest/uxx/fn.u52_for_index.html
[`index_for_uxx`]: https://docs.rs/uxx/latest/uxx/fn.index_for_uxx.html
[`index_for_u64`]: https://docs.rs/uxx/latest/uxx/fn.index_for_u64.html
[`index_for_u52`]: https://docs.rs/uxx/latest/uxx/fn.index_for_u52.html
[`binomial`]: https://docs.rs/uxx/latest/uxx/fn.binomial.html
