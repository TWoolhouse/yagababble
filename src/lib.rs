use std::{collections::HashMap, hash::Hash};

use rand::prelude::*;

pub struct MarkovTable<T: Eq + Hash> {
    map: HashMap<(Option<T>, Option<T>), Vec<Option<T>>>,
}

impl<T: Eq + Hash> MarkovTable<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, key: (Option<T>, Option<T>), value: Option<T>) {
        self.map.entry(key).or_default().push(value);
    }
}

impl<T: Eq + Hash> Extend<((Option<T>, Option<T>), Option<T>)> for MarkovTable<T> {
    fn extend<I: IntoIterator<Item = ((Option<T>, Option<T>), Option<T>)>>(&mut self, iter: I) {
        for (key, value) in iter {
            self.add(key, value);
        }
    }
}

impl<T: Eq + Hash + Clone> Extend<T> for MarkovTable<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let mut w1 = None;
        let mut w2 = None;
        for word in iter {
            let word = Some(word);
            self.add((w1, w2.clone()), word.clone());
            w1 = w2;
            w2 = word;
        }
        self.add((w1, w2.clone()), None);
        self.add((w2, None), None);
    }
}

impl<T: Eq + Hash + Clone> FromIterator<T> for MarkovTable<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut table = Self::new();
        table.extend(iter);
        table
    }
}

impl<T: Eq + Hash + Clone> MarkovTable<T> {
    pub fn chain(&self) -> impl Iterator<Item = &T> {
        let mut rng = rand::thread_rng();
        let mut w1 = None;
        let mut w2 = None;
        std::iter::from_fn(move || {
            let next_word = self
                .map
                .get(&(w1.clone(), w2.clone()))
                .map(|words| {
                    words
                        .iter()
                        .flat_map(std::convert::identity)
                        .choose(&mut rng)
                })
                .flatten();
            match next_word {
                Some(word) => {
                    w1 = w2.replace(word.clone());
                    Some(word)
                }
                None => None,
            }
        })
    }
}
