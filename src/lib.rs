pub fn sieve_of_eratosthenes(n: usize) -> usize {
    let mut primes: Vec<usize> = Vec::new();
    let mut is_prime = vec![true; n + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    for p in (0..n + 1).into_iter() {
        if !is_prime[p] {
            continue;
        }
        primes.push(p);

        let mut q = p * p;
        let mut r: Vec<usize> = Vec::new();
        while q < n + 1 {
            r.push(q);
            q += p;
        }

        for i in r.into_iter() {
            is_prime[i] = false;
        }
    }

    primes.len()
}

pub fn sieve_of_eratosthenes_2(n: usize) -> usize {
    let mut primes: Vec<usize> = Vec::new();
    let mut is_prime = vec![true; n + 1];

    is_prime[0] = false;
    is_prime[1] = false;

    let list = 0..n + 1;
    let list = list.into_iter();
    list.for_each(|p| {
        if is_prime[p] == true {
            primes.push(p);

            let mut q = p * p;
            let mut r: Vec<usize> = Vec::new();
            while q < n + 1 {
                r.push(q);
                q += p;
            }

            r.iter().for_each(|i| {
                is_prime[*i] = false;
            })
        }
    });

    primes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(4, sieve_of_eratosthenes(10));
    }

    #[test]
    fn test_2() {
        assert_eq!(4, sieve_of_eratosthenes_2(10));
    }
}
