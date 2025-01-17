use crate::{creds, session::Error};

pub(crate) struct Range {
    max: usize,
    set: Vec<usize>,
    current: usize,
    elements: usize,
}

impl Range {
    pub fn new(min: usize, max: usize, set: Vec<usize>) -> Result<Self, Error> {
        if set.is_empty() {
            if min > max {
                return Err(
                    "left side of range expression can't be greater than the right side".to_owned(),
                );
            }

            let elements = max - min + 1;
            Ok(Self {
                max,
                current: min,
                elements,
                set: vec![],
            })
        } else {
            let elements = set.len();
            Ok(Self {
                max: 0,
                current: 0,
                set,
                elements,
            })
        }
    }
}

impl creds::Iterator for Range {
    fn search_space_size(&self) -> usize {
        self.elements
    }
}

impl std::iter::Iterator for Range {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.set.is_empty() {
            if self.current <= self.max {
                let ret = self.current;
                self.current += 1;
                Some(ret.to_string())
            } else {
                None
            }
        } else if self.current < self.elements {
            let ret = self.set[self.current];
            self.current += 1;
            Some(ret.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::creds::{iterator, Expression};

    #[test]
    fn can_handle_min_max_range() {
        let expected = vec!["1", "2", "3", "4", "5"];
        let gen = iterator::new(Expression::Range {
            min: 1,
            max: 5,
            set: vec![],
        })
        .unwrap();
        let tot = gen.search_space_size();
        let vec: Vec<String> = gen.collect();

        assert_eq!(tot, expected.len());
        assert_eq!(vec, expected);
    }

    #[test]
    fn can_handle_set_range() {
        let expected = vec!["1", "666", "2", "234", "5", "19"];
        let gen = iterator::new(Expression::Range {
            min: 0,
            max: 0,
            set: vec![1, 666, 2, 234, 5, 19],
        })
        .unwrap();
        let tot = gen.search_space_size();
        let vec: Vec<String> = gen.collect();

        assert_eq!(tot, expected.len());
        assert_eq!(vec, expected);
    }
}
