use std::io::{self, BufRead};

fn longest_palindrome_subseq(s: &str) -> String {
    let n = s.len();
    let mut dp = vec![vec![(0, 0); n]; n];

    for i in 0..n {
        dp[i][i] = (1, i);
    }

    for len in 2..=n {
        for i in 0..=n-len {
            let j = i + len - 1;
            if s.chars().nth(i) == s.chars().nth(j) {
                dp[i][j] = (dp[i+1][j-1].0 + 2, i);
            } else {
                let (a, b) = dp[i+1][j];
                let (c, d) = dp[i][j-1];
                if a > c {
                    dp[i][j] = (a, b);
                } else {
                    dp[i][j] = (c, d);
                }
            }
        }
    }


    let mut result = String::new();
    let (mut len, mut i) = dp[0][n-1];
    let mut j = i + len - 1;
    while len > 0 {
        result.push(s.chars().nth(i).unwrap());
        if i == j {
            break;
        }
        i += 1;
        j -= 1;
        len -= 2;
    }
    result.push_str(&result.chars().rev().collect::<String>());

    result
}

fn main() {
    let stdin = io::stdin();
    let mut count = 0;

    loop {
        println!("Please input a string:");
        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let s = line.trim();

        if s.is_empty() {
            break;
        }

        let lps = longest_palindrome_subseq(s);
        println!("The longest palindrome subsequence is: {}", lps);

        count += 1;
        if count == 5 {
            break;
        }
    }
}
