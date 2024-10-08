pub fn quick_table_client_add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = quick_table_client_add(2, 2);
        assert_eq!(result, 4);
    }
}
