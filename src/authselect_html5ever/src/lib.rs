
mod element;
mod selector;

pub use element::ElementRef;
pub use selector::Selector;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
