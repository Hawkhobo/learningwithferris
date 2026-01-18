fn main() {
    // nothing here, but compiles
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        let second: u32 = numbers.1;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
