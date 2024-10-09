mod quick_table;
mod quick_table_rest;
pub use quick_table_rest::{
    QuickHandle,
};

pub use quick_table::{
    QuickTableProtocol,
    QuickPair,
    QuickTableResponse,
    QuickKey,
    QuickTableOverWrite,
    QuickTableResult,
    QuickValue,
    QuickCodable,
    HashableF64,
    QuickError,
};

//VX:TODO RM
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
