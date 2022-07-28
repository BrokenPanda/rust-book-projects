fn main() {
    dbg!(working_items_per_minute(5));
}

// Exercism
// Assembly Line
    pub fn production_rate_per_hour(speed: u8) -> f64 {
        let rate: f64;
        let speed = speed as u32;
        const RATE_PER_HOUR: u32 = 221;
        if speed <= 4 {
            rate = (RATE_PER_HOUR * speed) as f64;
        }
        else if speed <= 8 {
            rate = (RATE_PER_HOUR * speed) as f64 * 0.9;
        }
        else if speed > 8 {
            rate = (RATE_PER_HOUR * speed) as f64 * 0.77;
        }
        else {
            rate = 0.0;
        }
        rate
    }
    
    fn working_items_per_minute(speed: u8) -> u32 {
        (production_rate_per_hour(speed) / 60.0) as u32
    }
//
//