fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics5() {
        let mut x = 100;
        let y = &mut x;
        *y += 100;

        assert_eq!(x, 200);
    }
}
