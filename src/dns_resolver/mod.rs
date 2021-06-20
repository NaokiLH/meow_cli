pub mod resolver;

#[cfg(test)]
mod tests {
    use super::resolver::DnsWorker;

    #[test]
    fn work() {
        let t = DnsWorker::new("www.baidu.com");
        let ans = t.resolver();
        for i in ans.iter() {
            println!("{}", i);
        }
    }
}
