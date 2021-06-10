use std::collections::HashMap;

use super::gen_prime::gen_prime;

pub fn get_factor(x: u32) -> HashMap<u32, u32> {
    let mut list: HashMap<u32, u32> = HashMap::new();
    let primes = gen_prime(x);
    let mut number = x;
    for i in primes.iter() {
        while x % i == 0 {
            number = number / i;
            if list.contains_key(i) {
                let value = list.get_mut(i).unwrap();
                *value += 1;
            } else {
                list.insert(*i, 1);
            }
        }
    }

    list
}
