use std::str::Bytes;

struct Solution;

// old try at doing a linear solution
// use std::{
//     collections::{BTreeMap, HashMap, hash_map::Entry},
//     iter::Peekable,
//     str::Bytes,
// };

// enum Matcher {
//     Single(Option<u8>),
//     ZeroOrMore(Option<u8>),
// }

// impl Solution {
//     fn next_matcher(bytes: &mut Peekable<Bytes>) -> Option<Matcher> {
//         let match_byte = bytes.next()?;
//         if let Some(next) = bytes.peek()
//             && next == &b'*'
//         {
//             bytes.next();
//             Some(Matcher::ZeroOrMore(if match_byte == b'.' {
//                 None
//             } else {
//                 Some(match_byte)
//             }))
//         } else {
//             Some(Matcher::Single(if match_byte == b'.' {
//                 None
//             } else {
//                 Some(match_byte)
//             }))
//         }
//     }

//     pub fn is_match(s: String, p: String) -> bool {
//         let mut p = p.bytes().peekable();
//         let mut s = s.bytes().peekable();
//         let mut byte = s.next();
//         let mut matcher = Self::next_matcher(&mut p);
//         let mut zero_or_multiple_map = HashMap::new();
//         let mut zero_or_multiple_any_array = vec![];
//         while byte.is_some() || matcher.is_some() {
//             match matcher {
//                 None => return false,
//                 Some(Matcher::Single(match_byte)) => {
//                     if byte.is_none_or(|b| match_byte.is_some_and(|v| b != v))
//                         && !(zero_or_multiple_map.contains_key(&match_byte)
//                             || (zero_or_multiple_any_array
//                                 .last()
//                                 .is_some_and(|(v, _)| match_byte.is_none_or(|mb| mb == *v))))
//                     {
//                         return false;
//                     }
//                     let mut next_byte = true;
//                     if let Entry::Occupied(mut e) = zero_or_multiple_map.entry(match_byte) {
//                         *e.get_mut() -= 1;
//                         if *e.get() == 0 {
//                             e.remove();
//                         }
//                         next_byte = false;
//                     }
//                     let mut remove_last = false;
//                     if match_byte.is_none()
//                         && let Some((_, last)) = zero_or_multiple_any_array.last_mut()
//                     {
//                         if *last > 0 {
//                             *last -= 1;
//                         } else {
//                             remove_last = true;
//                         }
//                         next_byte = false;
//                     }
//                     if remove_last {
//                         zero_or_multiple_any_array.pop();
//                     }
//                     if next_byte {
//                         byte = s.next();
//                     }
//                     matcher = Self::next_matcher(&mut p);
//                 }
//                 Some(Matcher::ZeroOrMore(match_byte)) => {
//                     if byte.is_none_or(|b| match_byte.is_some_and(|v| b != v)) {
//                         matcher = Self::next_matcher(&mut p);
//                     } else {
//                         let map_byte = match_byte.or(byte);
//                         let matches = *zero_or_multiple_map.get(&map_byte).unwrap_or(&0);
//                         zero_or_multiple_map.clear();
//                         zero_or_multiple_map.insert(map_byte, matches + 1);
//                         if match_byte.is_none() {
//                             if let Some((last_value, last_key)) =
//                                 zero_or_multiple_any_array.last_mut()
//                             {
//                                 if byte == Some(*last_value) {
//                                     *last_key += 1;
//                                 } else if let Some(b) = byte {
//                                     zero_or_multiple_any_array.push((b, 1));
//                                 }
//                             } else if let Some(b) = byte {
//                                 zero_or_multiple_any_array.push((b, 1));
//                             }
//                         }
//                         byte = s.next();
//                     }
//                 }
//             }
//         }
//         true
//     }
// }

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        Self::is_match_bytes(s.as_bytes(), p.as_bytes())
    }

    fn is_match_bytes(s: &[u8], p: &[u8]) -> bool {
        let mut dp = vec![vec![false; s.len() + 1]; p.len() + 1];

        for p_idx in 0..=p.len() {
            for s_idx in 0..=s.len() {
                dp[p_idx][s_idx] = if p_idx == 0 {
                    s_idx == 0
                } else if s_idx == 0 {
                    p[p_idx - 1] == b'*' && dp[p_idx - 2][0]
                } else if p[p_idx - 1] == b'*' {
                    (p[p_idx - 2] == b'.' || p[p_idx - 2] == s[s_idx - 1]) && dp[p_idx][s_idx - 1]
                        || dp[p_idx - 2][s_idx]
                } else {
                    (p[p_idx - 1] == b'.' || p[p_idx - 1] == s[s_idx - 1])
                        && dp[p_idx - 1][s_idx - 1]
                };
            }
        }

        dp[p.len()][s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_001() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()))
    }

    #[test]
    fn test_case_002() {
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()))
    }

    #[test]
    fn test_case_003() {
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
    }

    #[test]
    fn test_case_004() {
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
    }

    #[test]
    fn test_case_005() {
        assert!(Solution::is_match(
            "mississippi".to_string(),
            "mis*is*ip*.".to_string()
        ));
    }

    #[test]
    fn test_case_006() {
        assert!(!Solution::is_match(
            "mississippi".to_string(),
            "mis*is*p*.".to_string()
        ));
    }

    #[test]
    fn test_case_007() {
        assert!(!Solution::is_match("ab".to_string(), ".*c".to_string()));
    }

    #[test]
    fn test_case_008() {
        assert!(!Solution::is_match("aaa".to_string(), "aaaa".to_string()));
    }

    #[test]
    fn test_case_009() {
        assert!(Solution::is_match("aaa".to_string(), "a*a".to_string()));
    }

    #[test]
    fn test_case_010() {
        assert!(!Solution::is_match("aaa".to_string(), "ab*a".to_string()));
    }

    #[test]
    fn test_case_011() {
        assert!(Solution::is_match(
            "aaa".to_string(),
            "ab*a*c*a".to_string()
        ));
    }

    #[test]
    fn test_case_012() {
        assert!(!Solution::is_match(
            "aaba".to_string(),
            "ab*a*c*a".to_string()
        ));
    }

    #[test]
    fn test_case_013() {
        assert!(!Solution::is_match("a".to_string(), ".*..a*".to_string()));
    }

    #[test]
    fn test_case_014() {
        assert!(Solution::is_match("ab".to_string(), ".*..".to_string()));
    }

    #[test]
    fn test_case_015() {
        assert!(Solution::is_match("bbbba".to_string(), ".*a*a".to_string()));
    }

    #[test]
    fn test_case_016() {
        assert!(Solution::is_match(
            "abbbcd".to_string(),
            "ab*bbbcd".to_string()
        ));
    }

    #[test]
    fn test_case_017() {
        assert!(Solution::is_match(
            "abcdede".to_string(),
            "ab.*de".to_string()
        ));
    }
}
