/*
    Given an array of strings words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.

    You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.

    Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.

    For the last line of text, it should be left-justified, and no extra space is inserted between words.

    Note:

    A word is defined as a character sequence consisting of non-space characters only.
    Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
    The input array words contains at least one word.

    Constraints:

    1 <= words.length <= 300
    1 <= words[i].length <= 20
    words[i] consists of only English letters and symbols.
    1 <= maxWidth <= 100
    words[i].length <= maxWidth
 */

pub struct Solution;

impl Solution {
    fn left_justify(words: &[String], max_width: usize) -> String {
        let mut line = words.join(" ");
        for _ in line.len()..max_width {
            line.push(' ');
        }
        line
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut lines = Vec::new();
        let max_width = max_width as usize;
        let mut n = words[0].len();
        let mut i = 0;
        let mut j = 1;
        while j < words.len() {
            let dn = words[j].len();
            if n + dn < max_width { 
                n += dn + 1; j += 1;
                continue; 
            }
            let w = j - i;
            if w > 1 {
                let s = max_width - (n - (w - 1));
                let s0 = s / (w - 1);
                let ds = s - (w - 1) * s0;
                let mut line = String::with_capacity(max_width);
                for k in 0..w {
                    line.push_str(words[i + k].as_str());
                    if k == w - 1 { break; }
                    let s = if k < ds { s0 + 1 } else { s0 };
                    for _ in 0..s { line.push(' '); }
                }
                lines.push(line);
            } else {
                let line = Solution::left_justify(&words[i..j], max_width);
                lines.push(line);
            }
            n = dn; i = j; j += 1;
        }
        let line = Solution::left_justify(&words[i..j], max_width);
        lines.push(line);
        lines
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /*
        Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
        Output:
        [
        "This    is    an",
        "example  of text",
        "justification.  "
        ]
     */
    #[test]
    fn example1() {
        let words = vec!["This", "is", "an", "example", "of", "text", "justification."].iter().map(|s| s.to_string() ).collect();
        let max_width = 16;

        let result = Solution::full_justify(words, max_width);

        let expected: Vec<String> = vec![
            "This    is    an",
            "example  of text",
            "justification.  "
        ].iter().map(|s| s.to_string() ).collect();
        assert_eq!(result, expected);
    }

    /*
        Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
        Output:
        [
        "What   must   be",
        "acknowledgment  ",
        "shall be        "
        ]
        Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
        Note that the second line is also left-justified because it contains only one word.
     */
    #[test]
    fn example2() {
        let words = vec!["What","must","be","acknowledgment","shall","be"].iter().map(|s| s.to_string() ).collect();
        let max_width = 16;

        let result = Solution::full_justify(words, max_width);

        let expected: Vec<String> = vec![
            "What   must   be",
            "acknowledgment  ",
            "shall be        "
        ].iter().map(|s| s.to_string() ).collect();
        assert_eq!(result, expected);
    }

    /*
        Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
        Output:
        [
        "Science  is  what we",
        "understand      well",
        "enough to explain to",
        "a  computer.  Art is",
        "everything  else  we",
        "do                  "
        ]
     */
    #[test]
    fn example3() {
        let words = vec!["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].iter().map(|s| s.to_string() ).collect();
        let max_width = 20;

        let result = Solution::full_justify(words, max_width);

        let expected: Vec<String> = vec![
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  "
        ].iter().map(|s| s.to_string() ).collect();
        assert_eq!(result, expected);
    }
}