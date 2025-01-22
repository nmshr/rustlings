use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    fn parse(local: &str) -> String {
        let mut split_plus = local.split('+');
        let mut local = split_plus.next().unwrap_or("").to_string();
        local = local.replace('.', "");
        local
    }

    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut dd: HashMap<String, HashSet<String>> = HashMap::new();

        for em in emails {
            if let Some((local, domain)) = em.split_once('@') {
                let parsed_local = Solution::parse(local);
                dd.entry(domain.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(parsed_local);
            }
        }
        let count: i32 = dd.values().map(|set| set.len()).sum::<usize>() as i32;
        count
    }
}

fn main() {
    let emails = vec![
        "test.email+alex@leetcode.com".to_string(),
        "test.e.mail+bob.cathy@leetcode.com".to_string(),
        "testemail+david@lee.tcode.com".to_string(),
    ];

    let result = Solution::num_unique_emails(emails);
    println!("Number of unique emails: {}", result);
}
