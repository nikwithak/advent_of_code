use std::{
    collections::{BTreeMap, HashMap},
    hash::Hasher,
    ops::{Deref, DerefMut},
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash_fn() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_sample_part_1() {
        assert_eq!(part_1("inputs/day_15_sample.txt"), 1320)
    }
    #[test]
    fn test_part_1_final() {
        assert_eq!(part_1("inputs/day_15.txt"), 507666)
    }
    #[test]
    fn test_part_2_sample() {
        assert_eq!(part_2("inputs/day_15_sample.txt"), 145)
    }
    #[test]
    fn test_part_2_final() {
        assert_eq!(part_2("inputs/day_15.txt"), 145)
    }
}

struct OrderedMapEntry<V> {
    value: V,
    insert_id: usize,
}
impl<V> Deref for OrderedMapEntry<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<V> DerefMut for OrderedMapEntry<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[derive(Default)]
struct OrderedHashMap<K, V> {
    map: HashMap<K, OrderedMapEntry<V>>,
    i_next: usize,
}

impl<K, V> Deref for OrderedHashMap<K, V> {
    type Target = HashMap<K, OrderedMapEntry<V>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K, V> DerefMut for OrderedHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K, V> OrderedHashMap<K, V>
where
    K: Eq + PartialEq + core::hash::Hash,
    V: Default,
{
    fn get_or_insert(
        &mut self,
        key: K,
        default: V,
    ) -> &mut V {
        let ret = self.map.entry(key).or_insert_with(|| OrderedMapEntry {
            value: default,
            insert_id: self.i_next,
        });
        self.i_next += 1;
        ret
    }

    fn get_or_default(
        &mut self,
        key: K,
    ) -> &mut V {
        self.get_or_insert(key, Default::default())
    }
}

fn part_2(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    // I think it wants me to use a linkedlist...
    // I bet the input sequence is designed to cause exponential growth if we have to scan the array each time
    let mut boxes: HashMap<u8, OrderedHashMap<&str, u8>> = Default::default();

    for s in input.split(",") {
        let i_operation = s.find(&['-', '=']).unwrap();
        let operation = &s.as_bytes()[i_operation];
        let label = &s[0..i_operation];
        let box_id = hash(label);

        match *operation as char {
            '-' => {
                // Go to relevant box
                let container = boxes.entry(box_id as u8).or_default();

                // Remove the lens with the label
                let _ = container.remove(&label);
            },
            '=' => {
                let focal_length: u8 = s[(i_operation + 1)..s.len()].parse().unwrap();
                *boxes.entry(box_id as u8).or_default().get_or_default(&label) = focal_length;
            },
            _ => panic!(),
        }
    }

    let val = boxes
        .iter()
        .map(|(box_id, box_contents)| (box_id, box_contents))
        .map(|(box_id, box_contents)| {
            let mut sorted_contents = box_contents.iter().collect::<Vec<_>>();
            sorted_contents.sort_by(|lhs, rhs| lhs.1.insert_id.cmp(&rhs.1.insert_id));
            let total_value = sorted_contents
                .iter()
                .enumerate()
                .map(|(i, (_, focal_length))| {
                    println!("i: {}", &i);
                    println!("focal_length: {}", **focal_length.deref());
                    println!("box_id: {}", &box_id);
                    (i + 1) * **focal_length.deref() as usize * (*box_id as usize + 1) as usize
                })
                .reduce(|acc, v| acc + v);
            total_value
        })
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .reduce(|acc, val| acc + val);
    val.unwrap()
}

fn part_1(filename: &str) -> usize {
    let input = std::fs::read_to_string(filename).unwrap();
    input.split(",").map(|s| hash(s)).reduce(|acc, v| acc + v).unwrap()
}

fn hash(input: &str) -> usize {
    let mut current_val = 0;
    for c in input.as_bytes() {
        current_val += *c as usize;
        current_val *= 17;
        current_val %= 256;
    }
    current_val
}
fn main() {}
