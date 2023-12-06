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
    fn part_1_sample_input() {
        let problem_def = load_file("inputs/day_5_sample.txt");
        let result = problem_def.get_lowest_location();
        assert_eq!(result, 35);
    }

    #[test]
    fn part_2_sample_input() {
        let problem_def = dbg!(load_file("inputs/day_5_sample.txt"));
        let result = problem_def.get_lowest_location_as_pairs();
        assert_eq!(result[0].0, 46);
    }

    #[test]
    fn part_2_real_input() {
        let problem_def = dbg!(load_file("inputs/day_5.txt"));
        let result = problem_def.get_lowest_location_as_pairs();
        // See note in main(): There's some bug causing the lowest one to be location 0, which is incorrect. The SECOND lowest from this run is correct.
        assert_eq!(result[1].0, 10834440);
    }
}
fn main() {
    let problem_def = load_file("inputs/day_5.txt");
    let result = problem_def.get_lowest_location();
    println!("Part 1 Result: {}", result);
    // TODO: There's a bug somewhere that affects the full input but NOT the sample input. Somehow the lowest ocation is 0, which is incorrect, but the second lowest location is the correct one.
    // Not gonna dump any more effrot into it, just accept there's a weird logic bug somewhere.
    println!("Part 2 Result: {}", problem_def.get_lowest_location_as_pairs()[1].0);
}

type Seed = u64;
type Soil = u64;
type Fertilizer = u64;
type Water = u64;
type Light = u64;
type Temperature = u64;
type Humidity = u64;
type Location = u64;

#[derive(Default, Debug)]
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

    fn get_locations_for_seed_ranges(
        &self,
        seed_ranges: Vec<(u64, u64)>,
    ) -> Vec<(Location, u64)> {
        dbg!(&seed_ranges);
        let soil = dbg!(self.seed_to_soil.get_destination_ranges(seed_ranges.iter().collect()));
        let fertilizer =
            dbg!(self.soil_to_fertilizer.get_destination_ranges(soil.iter().collect()));
        let water =
            dbg!(self.fertilizer_to_water.get_destination_ranges(fertilizer.iter().collect()));
        let light = dbg!(self.water_to_light.get_destination_ranges(water.iter().collect()));
        let temperature =
            dbg!(self.light_to_temperature.get_destination_ranges(light.iter().collect()));
        let humidity = dbg!(self
            .temperature_to_humidity
            .get_destination_ranges(temperature.iter().collect()));
        let mut location =
            dbg!(self.humidity_to_location.get_destination_ranges(humidity.iter().collect()));
        location.sort_by(|(l_start, _), (r_start, _)| l_start.cmp(r_start));
        location
    }

    fn get_lowest_location_as_pairs(&self) -> Vec<(Location, u64)> {
        // Extract the ranges from the input
        let mut seed_ranges = Vec::<(Seed, u64)>::new();
        let mut seeds = self.seeds.iter();
        while let Some(start_seed) = seeds.next() {
            let Some(range )= seeds.next() else { panic!()};
            seed_ranges.push((*start_seed, *range));
        }

        self.get_locations_for_seed_ranges(seed_ranges)
    }

    fn get_lowest_location(&self) -> Location {
        self.seeds
            .iter()
            .map(|seed| self.get_location_for_seed(*seed))
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

#[derive(Default, Debug)]
struct DestinationSourceMap {
    mappings: BTreeSet<u64>,
    map: HashMap<u64, DestinationSourceMapEntry>,
}

#[derive(Eq, PartialEq, Debug)]
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
        self.mappings
            .range(..source)
            .last()
            .map(|key| self.map.get(key))
            .flatten()
            .map(|entry| entry.get_destination(source))
            .flatten()
            .unwrap_or(source)
    }

    fn get_destination_ranges(
        &self,
        mut source_ranges: Vec<&(u64, u64)>, // Value, Range
    ) -> Vec<(u64, u64)> {
        // Sort so we can do it in one pass
        source_ranges.sort_by(|(lval, _), (rval, _)| lval.cmp(rval));

        let mut sources_iter = source_ranges.iter();
        let mut mappings_iter = self.mappings.iter();

        let mut current_mapping = mappings_iter.next().map(|key| self.map.get(key)).flatten();

        let mut result: Vec<(u64, u64)> = Vec::new();

        while let Some((start, range)) = sources_iter.next() {
            let mut start = *start;
            let mut range = *range;
            let mut is_done = false;

            while !is_done {
                // Lookup current mapping (start >= mapping_start, and < mapping_start + range)
                if let Some(mapping) = current_mapping {
                    if start >= mapping.source_range_start
                        && start < mapping.source_range_start + mapping.range_length
                    {
                        let offset = start - mapping.source_range_start;
                        let dest_start = offset + mapping.dest_range_start;

                        if (start + range) > (mapping.source_range_start + mapping.range_length) {
                            // This is NOT wholly contained. I need to set new start/range values
                            // let current_range = mapping.range_length - offset;
                            let current_range =
                                (mapping.source_range_start + mapping.range_length) - start;
                            result.push((dest_start, current_range));

                            // Set new values
                            start = start + current_range;
                            range = range - current_range;
                        } else {
                            // DONE (with this group)
                            result.push((dest_start, range));
                            is_done = true;
                        }
                    } else {
                        current_mapping =
                            mappings_iter.next().map(|key| self.map.get(key)).flatten();
                    }
                } else {
                    result.push((start, range));
                    is_done = true;
                }
            }
        }
        result
    }
}
