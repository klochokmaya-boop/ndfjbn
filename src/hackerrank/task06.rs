pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String {
    if v1 <= v2 {
        if x1 == x2 {
            return String::from("YES");
        }
        return String::from("NO");
    }

    if (x2 - x1) % (v1 - v2) == 0 {
        String::from("YES")
    } else {
        String::from("NO")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kangaroo_case_1() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_kangaroo_case_2() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_kangaroo_same_speed() {
        assert_eq!(kangaroo(0, 2, 4, 2), "NO");
    }
}
