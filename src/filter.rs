use std::collections:: {HashSet, HashMap};

struct ChatterFilter {
    threshold_ms: u64,
    last_press_keystroke: HashMap<u32, u64>, // last accepted keystroke value. u32 keycode, u64 timestamp
    keys_actively_down: HashSet<u32>,
}

impl ChatterFilter {
    fn new(threshold_ms: u64) -> Self {
        Self {
            threshold_ms,
            last_press_keystroke: HashMap::new(),
            keys_actively_down: HashSet::new(),
        }
    }

    // this decides true/false if/not chatter value.
    fn press(&mut self, key: u32, now_ms: u64) -> bool {
        if let Some(&previous) = self.last_press_keystroke.get(&key) {
            if now_ms - previous < self.threshold_ms { //decide chatter value. ex; now ms = 100 and prev = 110. now_ms - previous = 10 ms. if previous is greater than threshold, then return true
                return false;
            }
        }
        self.last_press_keystroke.insert(key, now_ms);
        self.keys_actively_down.insert(key);
        true
    }

    fn release(&mut self, key: u32) -> bool {
        return self.keys_actively_down.remove(&key); //return value if deleted or not, I think.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_rapid_repeat() {
        let mut f = ChatterFilter::new(50);
        assert_eq!(f.press(30, 1000), true);
        assert_eq!(f.press(30, 1010), false);
    }

    #[test]
    fn accepts_slow_repeat() {
        let mut f = ChatterFilter::new(50);
        assert_eq!(f.press(30, 1000), true);
        assert_eq!(f.press(30, 1200), true);
    }

    #[test]
    fn clean_press_release() { // unfinished test
        let mut f = ChatterFilter::new(50);
        assert!(f.press(30, 1000));
        assert!(f.release(30));
    }

    #[test]
    fn chatter_does_not_stick_key() {
        let mut f = ChatterFilter::new(50);
        assert!(f.press(30, 1000)); //accepted valid
        assert!(!f.press(30, 1010)); //bounce, dropped
        assert!(f.release(30)); //this is the real one and should continue
        assert!(!f.release(30)); // false is still dropped
    }

    #[test]
    fn rejected_press_is_not_marked_down() {
        let mut f = ChatterFilter::new(50);
        assert!(f.press(30, 1000));
        assert!(f.release(30));
        assert!(!f.press(30, 1010));  // too soon -> rejected
        assert!(!f.release(30));      // so nothing was marked down
    }
}