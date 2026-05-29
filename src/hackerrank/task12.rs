pub fn page_count(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    let from_back = (n / 2) - (p / 2);

    std::cmp::min(from_front, from_back)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_0() {
        assert_eq!(page_count(6, 2), 1);
    }

    #[test]
    fn test_sample_1() {
        assert_eq!(page_count(5, 4), 0);
    }

    #[test]
    fn test_even_total_target_last_odd() {
        assert_eq!(page_count(6, 5), 1);
    }

    #[test]
    fn test_odd_total_target_near_end() {
        assert_eq!(page_count(7, 4), 1);
    }

    #[test]
    fn test_single_page_book() {
        assert_eq!(page_count(1, 1), 0);
    }
}