pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let min_val = *a.iter().max().unwrap_or(&1);
    let max_val = *b.iter().min().unwrap_or(&100);
    let mut count = 0;

    for x in min_val..=max_val {
        let is_a_factor = a.iter().all(|&ai| x % ai == 0);
        let is_factor_of_b = b.iter().all(|&bi| bi % x == 0);

        if is_a_factor && is_factor_of_b {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_x() {
        let a = vec![2, 4];
        let b = vec![16, 32, 96];
        assert_eq!(get_total_x(&a, &b), 3);
    }
}