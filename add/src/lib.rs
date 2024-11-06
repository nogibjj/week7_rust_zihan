// src/lib.rs

/// A simple function that returns a hello message
pub fn hello(name: &str) -> String {
    format!("Hello, {}! Welcome to the week7_test_tool.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let result = hello("Zihan");
        assert_eq!(result, "Hello, Zihan! Welcome to the week7_test_tool.");
    }
}
