
#[cfg(test)]
mod tests {

    #[test]
    fn test_available_parallelism() {
        let env_var="FOOBAR";
        let foo = dotenv::var(env_var)
            .unwrap_or_else(|_| panic!("{} is not configured properly", env_var))
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("{} is not a valid i32", env_var));
        println!("{}", foo);
    }
}
