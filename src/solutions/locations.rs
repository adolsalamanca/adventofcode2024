pub struct LocationsMap {
    map: Vec<LocationTuples>
}

impl LocationsMap {
    fn new(m: Vec<LocationTuples>) -> LocationsMap {
        LocationsMap { map: m }
    }

    pub fn total_distance(self) -> u32 {
        let mut d:u32 = 0;
        for item in self.map {
            d += item.distance();
        }
        d
    }
}

#[derive(Debug)]
struct LocationTuples {
    from: u32,
    to: u32
}

impl LocationTuples {
    fn distance(&self) -> u32 {
        // to - from
        // to >= from
        if self.to >= self.from {
            return self.to -  self.from
        } else {
            return self.from -  self.to
        }
    }
}

pub fn vectors_to_locations_map(from: Vec<u32>, to: Vec<u32>) -> LocationsMap {
    let mut locs_array: Vec<LocationTuples> = Vec::new();
    for i in 0..from.len() {
        locs_array.push(LocationTuples { from: from[i], to: to[i] });
    }
    let locs_map = LocationsMap::new(locs_array);
    locs_map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::load::read_vectors_from_str;
    use crate::solutions::locations::LocationTuples;

    #[test]
    fn should_calculate_location_tuples_distance() {
        let t = LocationTuples {from: 3, to: 5};
        assert_eq!(t.distance(), 2);
    }

    #[test]
    fn should_calculate_distances_in_reverse_order_too() {
        let t = LocationTuples {from: 4, to: 1};
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
        match read_vectors_from_str(input_str) {
            Ok((from_arr, to_arr)) => {
                from = from_arr;
                to = to_arr;
            }
            Err(e) => eprintln!("Error processing string input: {}", e),
        }

        let locs_map = vectors_to_locations_map(from, to);

        assert_eq!(11, locs_map.total_distance());
    }
}