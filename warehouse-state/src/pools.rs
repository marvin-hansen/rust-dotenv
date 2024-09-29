
#[cfg(test)]
mod tests {

    #[test]
    fn test_available_parallelism() {
        let foo =  env!("SOME_ENV");
        let number = foo.parse::<i32>().unwrap_or_else(|_| panic!("{} is not a valid i32", foo));

        assert_eq!(number, 42);

        println!("{}", foo);
    }
}
