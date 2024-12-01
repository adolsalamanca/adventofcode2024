use std::collections::HashMap;
#[derive(Clone)]
pub struct LocationsMap {
    m: Vec<LocationTuple>,
    similarity_destiny: HashMap<u32, u32>
}

impl LocationsMap {
    fn new(m: Vec<LocationTuple>) -> LocationsMap {
        let mut s:HashMap<u32, u32> = HashMap::new();
        for t in m.iter() {
            *s.entry(t.to).or_insert(0) += 1;
        }
        LocationsMap {
            m, similarity_destiny: s
        }
    }

    pub fn total_distance(self) -> u32 {
        let mut d:u32 = 0;
        for item in self.m {
            d += item.distance();
        }
        d
    }
    pub fn similarity_score_destiny(self) -> u32 {
        let mut s:u32 = 0;
        for item in self.m {
            // Check how many times that number appears
            if let Some(multiplier) = self.similarity_destiny.get(&item.from) {
                s += multiplier * item.from;
            }
        }
        s
    }
}

#[derive(Clone,Debug)]
struct LocationTuple {
    from: u32,
    to: u32
}

impl LocationTuple {
    fn distance(&self) -> u32 {
        if self.to >= self.from {
            self.to -  self.from
        } else {
            self.from -  self.to
        }
    }
}

pub fn vectors_to_locations_map(from: Vec<u32>, to: Vec<u32>) -> LocationsMap {
    let mut locs_array: Vec<LocationTuple> = Vec::new();
    for i in 0..from.len() {
        locs_array.push(LocationTuple { from: from[i], to: to[i] });
    }
    LocationsMap::new(locs_array)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::read::vectors_from_str;
    use crate::solutions::locations::LocationTuple;

    #[test]
    fn should_calculate_location_tuples_distance() {
        let t = LocationTuple {from: 3, to: 5};
        assert_eq!(t.distance(), 2);
    }

    #[test]
    fn should_calculate_distances_in_reverse_order_too() {
        let t = LocationTuple {from: 4, to: 1};
        assert_eq!(t.distance(), 3);
    }

    #[test]
    fn should_retrieve_total_distance_from_input(){
        let input_str: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
        let (mut from, mut to): (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        match vectors_from_str(input_str) {
            Ok((from_arr, to_arr)) => {
                from = from_arr;
                to = to_arr;
            }
            Err(e) => eprintln!("Error processing string input: {}", e),
        }

        let locs_map = vectors_to_locations_map(from, to);

        assert_eq!(11, locs_map.total_distance());
    }
    #[test]
    fn should_calculate_similarity_score(){
        let input_str: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
        let (mut from, mut to): (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        match vectors_from_str(input_str) {
            Ok((from_arr, to_arr)) => {
                from = from_arr;
                to = to_arr;
            }
            Err(e) => eprintln!("Error processing string input: {}", e),
        }

        let locs_map = vectors_to_locations_map(from, to);

        assert_eq!(31, locs_map.similarity_score_destiny());
    }


}