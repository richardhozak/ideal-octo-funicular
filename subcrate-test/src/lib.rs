
#[cfg(feature = "one")]
pub fn test_print_function() {
    println!("feature one");
}

#[cfg(feature = "two")]
pub fn test_print_function() {
    println!("feature two");
}

#[cfg(not(any(feature = "one", feature = "two")))]
pub fn test_print_function() {
    println!("nofeature");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
