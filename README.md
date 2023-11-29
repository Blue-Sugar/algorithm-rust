# 競プロ用ライブラリ

## digraph
1. weighted-graph
    1. max-flow-graph
    2. mod (floyd-warshall)

## graph
1. tree
    1. segment-tree
    2. lazy-segment-tree
    3. binary-index-tree
    4. mod (parent, children, distance from root)
2. grid
    1. valued-grid (neighbor4 and neighbor8) modの実装をoverrideしたい。
    2. mod (neighbor4 and 8) 

## math
1. algebra
    1. monoid
        1. mod (monoid trait)
        2. range-query (range-max, range-min, range-gcd)
2. erathosthenes (is_prime, prime-factorization, devidors)
3. gcd (gcd, lcm, is_coprime, ax + by = gcd(a, b), chinese-remainder-theorem)
4. modint
    1. mod (power, inverse, ax + by = gcd(a, b))
    1. finite-field (div)
5. prime_factorization (dividors, prime-factorization)
6. complex
    1. mod (omega, operator)
7. formal-power-series
    1. mod (mul using fft)
    2. complex-formal-power-series (fft, inv_fft)
8. rational
    1. mod (operator, approx)
    2. repeating-decimal (repeating-decimal <-> rational) too slow

## set
1. disjoint-set-union 

## shared
1. binary-search (lower-bound, lower-bound-in-vec)
2. change-min-max (chmin, chmax)

## string
1. mod (is-padindrome, run-length-encoding)