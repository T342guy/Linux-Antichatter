use std::collections::HashMap;

struct ChatterFilter {
    threshold_ms: u64,
    last_press_keystroke: HashMap<u32, u64>, // last accepted keystroke value. u32 keycode, u64 timestamp
}

impl ChatterFilter {
    fn new(threshold_ms: u64) -> Self {
        Self {
            threshold_ms,
            last_press_keystroke: HashMap::new(),
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
        true
    }
}

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
}