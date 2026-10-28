use std::collections::{HashMap, VecDeque};

use crate::models::translation::TranslationResult;

/// Cache FIFO em memória (nunca gravado em disco) com capacidade fixa.
pub struct TranslationCache {
    capacity: usize,
    order: VecDeque<String>,
    map: HashMap<String, TranslationResult>,
}

impl TranslationCache {
    pub fn new(capacity: usize) -> Self {
        Self { capacity, order: VecDeque::new(), map: HashMap::new() }
    }

    pub fn get(&self, key: &str) -> Option<TranslationResult> {
        self.map.get(key).cloned()
    }

    pub fn put(&mut self, key: String, value: TranslationResult) {
        if self.map.insert(key.clone(), value).is_none() {
            self.order.push_back(key);
            while self.order.len() > self.capacity {
                if let Some(oldest) = self.order.pop_front() {
                    self.map.remove(&oldest);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(text: &str) -> TranslationResult {
        TranslationResult {
            original_text: text.into(),
            translated_text: text.into(),
            detected_language: None,
            target_language: "pt".into(),
        }
    }

    #[test]
    fn evicts_oldest_entries() {
        let mut cache = TranslationCache::new(2);
        cache.put("a".into(), result("a"));
        cache.put("b".into(), result("b"));
        cache.put("c".into(), result("c"));
        assert!(cache.get("a").is_none());
        assert!(cache.get("b").is_some());
        assert!(cache.get("c").is_some());
    }
}
