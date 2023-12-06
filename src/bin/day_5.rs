use std::{
    collections::{BTreeSet, HashMap},
    fs::File,
    io::{BufRead, BufReader},
    thread::current,
};

#[cfg(test)]
mod tests {
    use crate::load_file;

    #[test]
    fn test_sample_input() {
        let problem_def = load_file("inputs/day_5_sample.txt");
        let result = problem_def.get_lowest_location();
        assert_eq!(result, 35);
    }
}
fn main() {
    let problem_def = load_file("inputs/day_5.txt");
    let result = problem_def.get_lowest_location();
    println!("Part 1 Result: {}", result);
}

type Seed = u64;
type Soil = u64;
type Fertilizer = u64;
type Water = u64;
type Light = u64;
type Temperature = u64;
type Humidity = u64;
type Location = u64;

#[derive(Default)]
struct ProblemDefinition {
    seeds: Vec<Seed>,
    seed_to_soil: DestinationSourceMap,
    soil_to_fertilizer: DestinationSourceMap,
    fertilizer_to_water: DestinationSourceMap,
    water_to_light: DestinationSourceMap,
    light_to_temperature: DestinationSourceMap,
    temperature_to_humidity: DestinationSourceMap,
    humidity_to_location: DestinationSourceMap,
}

impl ProblemDefinition {
    fn get_location_for_seed(
        &self,
        seed: Seed,
    ) -> Location {
        dbg!(&seed);
        let soil = dbg!(self.seed_to_soil.get_destination(seed));
        let fertilizer = dbg!(self.soil_to_fertilizer.get_destination(soil));
        let water = dbg!(self.fertilizer_to_water.get_destination(fertilizer));
        let light = dbg!(self.water_to_light.get_destination(water));
        let temperature = dbg!(self.light_to_temperature.get_destination(light));
        let humidity = dbg!(self.temperature_to_humidity.get_destination(temperature));
        let location = dbg!(self.humidity_to_location.get_destination(humidity));
        location as Location
    }

    fn get_lowest_location(&self) -> Location {
        self.seeds
            .iter()
            .map(|seed| self.get_location_for_seed(*seed))
            .map(|seed| dbg!(seed))
            .fold(None, |min: Option<Location>, location| {
                Some(min.unwrap_or(location).min(location))
            })
            .unwrap()
    }
}

fn load_file(filename: &str) -> ProblemDefinition {
    let reader = BufReader::new(File::open(filename).unwrap());
    let mut problem_def = ProblemDefinition::default();
    let mut current_map = None;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("seeds:") {
            let seeds: Vec<Seed> = line
                .split(':')
                .last()
                .map(|seeds_str| seeds_str.split(" ").filter(|s| !s.is_empty()))
                .unwrap()
                .map(|seed| seed.parse().unwrap())
                .collect();
            problem_def.seeds = seeds;
        } else {
            match dbg!(line.as_str()) {
                "" => {},
                "seed-to-soil map:" => current_map = Some(&mut problem_def.seed_to_soil),
                "soil-to-fertilizer map:" => {
                    current_map = Some(&mut problem_def.soil_to_fertilizer)
                },
                "fertilizer-to-water map:" => {
                    current_map = Some(&mut problem_def.fertilizer_to_water)
                },
                "water-to-light map:" => current_map = Some(&mut problem_def.water_to_light),
                "light-to-temperature map:" => {
                    current_map = Some(&mut problem_def.light_to_temperature)
                },
                "temperature-to-humidity map:" => {
                    current_map = Some(&mut problem_def.temperature_to_humidity)
                },
                "humidity-to-location map:" => {
                    current_map = Some(&mut problem_def.humidity_to_location)
                },
                _ => {
                    let mut values = line.split(" ").map(|val| val.parse().unwrap());
                    let Some(ref mut map) = current_map else { panic!() };
                    map.add_mapping(DestinationSourceMapEntry {
                        dest_range_start: values.next().unwrap(),
                        source_range_start: values.next().unwrap(),
                        range_length: values.next().unwrap(),
                    })
                },
            }
        }
    }

    problem_def
}

#[derive(Default)]
struct DestinationSourceMap {
    mappings: BTreeSet<u64>,
    map: HashMap<u64, DestinationSourceMapEntry>,
}

#[derive(Eq, PartialEq)]
struct DestinationSourceMapEntry {
    dest_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl DestinationSourceMapEntry {
    fn get_destination(
        &self,
        source: u64,
    ) -> Option<u64> {
        if source < self.source_range_start {
            return None;
        }
        let offset = source - self.source_range_start;
        if offset > self.range_length {
            return None;
        }
        Some(self.dest_range_start + offset)
    }
}

impl DestinationSourceMap {
    fn add_mapping(
        &mut self,
        entry: DestinationSourceMapEntry,
    ) {
        self.mappings.insert(entry.source_range_start);
        self.map.insert(entry.source_range_start, entry);
    }

    fn get_destination(
        &self,
        source: u64,
    ) -> u64 {
        let tmp = self.mappings.range(..source);
        let tmp = tmp.last();
        let tmp = tmp.map(|key| self.map.get(key));
        let tmp = tmp.flatten();
        let tmp = tmp.map(|entry| entry.get_destination(source));
        let tmp = tmp.flatten();
        let tmp = tmp.unwrap_or(source);
        tmp
    }
}
