#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod undirected;
mod node;
pub mod directed;
pub mod traversal;