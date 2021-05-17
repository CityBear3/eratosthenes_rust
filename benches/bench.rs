#![feature(test)]

extern crate test;

use bench_prime::{sieve_of_eratosthenes, sieve_of_eratosthenes_2};
use test::Bencher;

#[bench]
fn bench_1(b: &mut Bencher) {
    b.iter(|| {
        sieve_of_eratosthenes(10000000)
    });
}


#[bench]
fn bench_2(b: &mut Bencher) {
    b.iter(|| {
        sieve_of_eratosthenes_2(10000000)
    });
}