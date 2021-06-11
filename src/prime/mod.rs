pub mod gen_prime;
pub mod get_factor;
#[cfg(test)]
mod tests {
    use crate::prime::get_factor;
    #[test]
    fn work() {
        let t = get_factor::get_factor(100);

        println!("{:?}", t);
    }
}
