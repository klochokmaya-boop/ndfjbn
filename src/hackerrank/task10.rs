
use std::collections::HashMap;

pub fn sock_merchant(_n: i32, ar: &[i32]) -> i32 {
    let mut color_counts = HashMap::new();

    for &color in ar {
        *color_counts.entry(color).or_insert(0) += 1;
    }

    color_counts.values().map(|&count| count / 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_case() {
        let ar = [10, 20, 20, 10, 10, 30, 50, 10, 20];
        assert_eq!(sock_merchant(9, &ar), 3);
    }

    #[test]
    fn test_no_pairs() {
        let ar = [1, 2, 3, 4, 5];
        assert_eq!(sock_merchant(5, &ar), 0);
    }

    #[test]
    fn test_all_same_color() {
        let ar = [1, 1, 1, 1, 1];
        assert_eq!(sock_merchant(5, &ar), 2);
    }

    #[test]
    fn test_empty() {
        let ar = [];
        assert_eq!(sock_merchant(0, &ar), 0);
    }
}