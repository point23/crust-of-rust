// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

/**
* @note
*
* This tutorial is actually mainly about "LIFETIME"
* Doc lifetime: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
*
* Basics Struts
* - str:    char, char, char ...
* - &str: &[char, char, char]
* - String: vec<char> (on the Heap)
*
* Convert
* - String -> &str   (cheap -- AsRef)
* - &str   -> String (expensive -- memcpy)
*
* Std Impl
* Lib str: https://doc.rust-lang.org/std/primitive.str.html
* Lit Split: https://doc.rust-lang.org/std/str/struct.Split.html
*/

pub trait Delimiter {
    fn locate_itself(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimiter for &str {
    fn locate_itself(&self, s: &str) -> Option<(usize, usize)> {
        let start = s.find(self)?;

        return Some((start, start + self.len()));
    }
}

impl Delimiter for char {
    fn locate_itself(&self, s: &str) -> Option<(usize, usize)> {
        for (it_idx, it) in s.char_indices() {
            if it == *self {
                return Some((it_idx, it_idx + self.len_utf8()));
            }
        }

        return None;
    }
}

pub struct StrSplit<'h, D> {
    remainder: Option<&'h str>,
    delimiter: D,
}

impl<'h, D> StrSplit<'h, D> {
    pub fn new(haystack: &'h str, delimiter: D) -> Self {
        return Self {
            remainder: Some(haystack),
            delimiter,
        };
    }
}

impl<'h, D> Iterator for StrSplit<'h, D>
where
    D: Delimiter,
{
    type Item = &'h str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((start, end)) = self.delimiter.locate_itself(remainder) {
            let letter = &remainder[..start];
            *remainder = &remainder[end..];
            return Some(letter);
        } else {
            return self.remainder.take();
        }
    }
}

pub fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    StrSplit::new(s, c).next().expect("StrSplit always ret")
}

#[test]
fn simple_str() {
    let haystack = "a b c";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c"].into_iter()));
}

#[test]
fn empty_tail() {
    let haystack = "a b c ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", ""].into_iter()));
}

#[test]
fn get_head() {
    assert!(until_char("abc", 'b').eq("a"));
}
