use std::collections::HashMap;

pub fn gen_prime(x: u32) -> Vec<u32> {
    let mut primes = Vec::<u32>::new();
    let mut st: HashMap<u32, bool> = HashMap::new();
    for i in 2..x + 1 {
        if !st.contains_key(&i) {
            primes.push(i);
        }
        let mut j = 0;
        while primes[j] <= x / i {
            st.insert(primes[j] * i, true);
            if i % primes[j] == 0 {
                break;
            }
            j += 1;
        }
    }
    primes
}
