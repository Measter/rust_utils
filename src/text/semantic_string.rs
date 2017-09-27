use std::ascii::AsciiExt;
use std::convert::From;
use std::cmp::Ordering;
use std::ops::Deref;

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq)]
enum StringPart<'a> {
    Text(&'a str),
    Number(u64),
}

impl<'a> Ord for StringPart<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::StringPart::*;
        match (self, other) {
            (&Text(ref a), &Text(ref b)) => a.cmp(b),
            (&Number(ref a), &Number(ref b)) => a.cmp(b),
            (&Text(_), &Number(_)) => Ordering::Less,
            (&Number(_), &Text(_)) => Ordering::Greater,
        }
    }
}

impl<'a> PartialOrd for StringPart<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SemanticString<'a> {
    pub raw: &'a str,
    parts: Vec<StringPart<'a>>,
}

impl<'a> Ord for SemanticString<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.raw.len().cmp(&other.raw.len()) {
            Ordering::Equal => {
                for (a,b) in self.parts.iter().zip(other.parts.iter()) {
                    if a.cmp(b) != Ordering::Equal {
                        return a.cmp(b)
                    }
                }

                Ordering::Equal
            },
            i @ _ => i,
        }
    }
}

impl<'a> PartialOrd for SemanticString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> SemanticString<'a> {
    pub fn new(raw: &'a str) -> SemanticString {
        let mut parts = vec![];
        for (is_num, mut group) in &raw.char_indices().group_by(|&(_, c)| c.is_numeric() && c.is_ascii()) {
            let first_index = if let Some((i, _)) = group.next() {
                i
            } else {
                continue;
            };

            let last_index = if let Some((i, _)) = group.last() {
                 i
            } else {
                first_index+1
            };

            let part = &raw[first_index..last_index];
            
            let part = if is_num {
                StringPart::Number(part.parse().expect(&format!("tried to parse {} as an int", part))) // If this fails, things have gone badly wrong
            } else {
                StringPart::Text(part)
            };

            parts.push(part);
        }

        SemanticString {
            raw: raw,
            parts: parts,
        }
    }
}

impl<'a> From<&'a str> for SemanticString<'a> {
    fn from(raw: &'a str) -> SemanticString<'a> {
        SemanticString::new(raw)
    }
}

impl<'a> Deref for SemanticString<'a> {
    type Target = &'a str;
    fn deref(&self) -> &&'a str {
        &self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed() {
        let strings = ["foo11bar", "foo2bar",];

        let mut sem_strings: Vec<_> = strings.iter().map(|x| SemanticString::new(x)).collect();
        sem_strings.sort();
        let orig: Vec<_> = sem_strings.iter().map(|x| x.raw).collect();

        assert_eq!(orig, vec!["foo2bar", "foo11bar"]);
    }

    #[test]
    fn text() {
        let strings = ["foo", "bar"];

        let mut sem_strings: Vec<_> = strings.iter().map(|x| SemanticString::new(x)).collect();
        sem_strings.sort();
        let orig: Vec<_> = sem_strings.iter().map(|x| x.raw).collect();

        assert_eq!(orig, vec!["bar", "foo"] );
    }

    #[test]
    fn numbers() {
        let strings = ["2", "10", "1"];

        let mut sem_strings: Vec<_> = strings.iter().map(|x| SemanticString::new(x)).collect();
        sem_strings.sort();
        let orig: Vec<_> = sem_strings.iter().map(|x| x.raw).collect();

        assert_eq!(orig, vec!["1", "2", "10"]);
    }

    #[test]
    fn mixed_2() {
        let strings = ["test", "2"];

        let mut sem_strings: Vec<_> = strings.iter().map(|x| SemanticString::new(x)).collect();
        sem_strings.sort();
        let orig: Vec<_> = sem_strings.iter().map(|x| x.raw).collect();

        assert_eq!(orig, vec!["2", "test"]);
    }
    
    #[test]
    fn empty() {
        let string = "";
        let sem_string = SemanticString::new(string);

        assert_eq!(string, sem_string.raw);
    }
}