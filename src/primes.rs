#[derive(Debug, Clone)]
pub struct Primes {
    primes: Vec<bool>
}

impl Primes {
    pub fn new(n: usize) -> Self {
        let n = n / 2 + n % 2 - 1;
        let mut step = 3;
        let mut skip = 0;
        let mut primes = (0..n).map(|_| true).collect::<Vec<bool>>();

        while step + skip < n {
            for i in (skip..n).step_by(step).skip(1) {
                primes[i] = false;
            }

            if let Some((next_idx, _)) = primes.iter()
                .enumerate()
                .skip(skip + 1)
                .find(|(_idx, prime)| **prime) {
                    skip = next_idx;
                    step = next_idx * 2 + 3;
            } else {
                break
            }
        }

        Self{ primes }
    }

    pub fn iter_primes<'a>(&'a self) -> impl DoubleEndedIterator<Item = usize> + 'a {
        self.primes
            .iter()
            .enumerate()
            .filter_map(|(idx, is_prime)| if *is_prime { Some(Self::prime(idx)) } else { None })
    }

    pub fn divisors<'a>(&'a self, number: usize) -> impl Iterator<Item = usize> + 'a {
        let max_div = (number as f64).sqrt() as usize;

        let primes = if max_div > 2 {
            let idx = Self::idx(max_div);
            let primes = self.primes[..=idx]
                .iter()
                .enumerate()
                .filter_map(|(idx, is_prime)| if *is_prime { Some(Self::prime(idx)) } else { None });

            Some(primes)
        } else {
            None
        };

        Some(2).into_iter()
            .chain(primes.into_iter().flatten())
            .filter(move |prime| number % prime == 0)
    }

    pub fn nearest_prime(&self, number: usize) -> Option<usize> {
        if number < 2 { return None }
        if number == 2 { return Some(2) }

        let idx = Self::idx(number) + 1;

        self.primes.iter()
            .enumerate()
            .take(idx)
            .rev()
            .filter(|(_, prime)| **prime)
            .next()
            .map(|(idx, _)| Self::prime(idx))
    }

    fn prime(idx: usize) -> usize {
        idx * 2 + 3
    }

    fn idx(prime: usize) -> usize {
        (prime - 3) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(100 => Some(97))]
    #[test_case(98 => Some(97))]
    #[test_case(97 => Some(97))]
    fn test_primes(primes_till: usize) -> Option<usize> {
        let primes = Primes::new(primes_till);
        let next = primes.iter_primes().rev().next();

        next
    }

    #[test_case(0 => vec![2])] // special case, 0 is not recommended to put there
    #[test_case(1 => Vec::<usize>::new())]
    #[test_case(5 => Vec::<usize>::new())]
    #[test_case(6 => vec![2])]
    #[test_case(9 => vec![3])]
    #[test_case(24 => vec![2, 3])]
    #[test_case(25 => vec![5])]
    #[test_case(48 => vec![2, 3])]
    #[test_case(49 => vec![7])]
    #[test_case(120 => vec![2, 3, 5])]
    #[test_case(121 => vec![11])]
    fn test_divisors(n: usize) -> Vec<usize> {
        let primes = Primes::new(100);

        primes.divisors(n).collect()
    }

    #[test_case(1 => None)]
    #[test_case(2 => Some(2))]
    #[test_case(3 => Some(3))]
    #[test_case(7 => Some(7))]
    #[test_case(100 => Some(97))]
    #[test_case(97 => Some(97))]
    fn test_nearest_prime(n: usize) -> Option<usize> {
        let primes = Primes::new(100);

        primes.nearest_prime(n)
    }
}
