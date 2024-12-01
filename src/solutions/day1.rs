#[allow(unused_imports)]
use crate::data;

struct LocationTuples {
    from: u8,
    to: u8
}

impl LocationTuples {
    fn distance(&self) -> u8 {
        self.to.wrapping_sub(self.from)
    }
}

pub(crate) fn main(){
    println!("Day1 finished.");

}

#[cfg(test)]
mod tests {
    use crate::solutions::day1::LocationTuples;

    #[test]
    fn should_retrieve_location_tuples_distance() {
        let t = LocationTuples {from: 3, to: 5};
        assert_eq!(t.distance(), 2);
    }
    #[test]
    fn should_not_panic_calculating_negative_tuples_distance() {
        let t = LocationTuples {from: 3, to: 1};
        let _ = t.distance();
    }
}