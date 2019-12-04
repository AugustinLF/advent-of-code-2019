use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_map(string: &str) ->(i8, HashMap<i8, i32>) {
    let mut map = HashMap::new();
    let mut count = 0;
    for cell in string.trim().split(',') {
        let cell = cell.parse::<i32>().unwrap();
        map.insert(count, cell);
        count = count + 1;
    }

    return (count, map);
}

fn get_at_index(map: &HashMap<i8, i32>, index: i8) -> i32 {
    return *map.get(&index).unwrap()
}

struct AssociatedValues {
    first: i32,
    second: i32,
    target_position: i8
}

fn get_associated_values(index: i8, map: &HashMap<i8, i32>) -> AssociatedValues {
    let first_position = get_at_index(map, index + 1) as i8;
    let second_position = get_at_index(map, index + 2) as i8;
    let target_position = get_at_index(map, index + 3) as i8;
    let first = get_at_index(map, first_position);
    let second = get_at_index(map, second_position);

    return AssociatedValues {
        first,
        second,
        target_position
    }
}

fn iterate_and_update(mut map: HashMap<i8, i32>, size: i8) -> HashMap<i8, i32> {
    let mut count = 0;
    while count < size {
        let value: i32 = get_at_index(&map, count);
        if value == 99 {
            return map;
        } else if value == 1 {
            let values = get_associated_values(count, &map);
            let new_value = values.first + values.second;
            map.insert(values.target_position, new_value);
            count = count + 4;
        } else if value == 2 {
            let values = get_associated_values(count, &map);
            let new_value = values.first * values.second;
            map.insert(values.target_position, new_value);
            count = count + 4;
        } else {
            panic!("Wrong opcode");
        }
    }
    panic!("Should have found 99");
}

fn string_to_string(string: &str) -> String {
    let (size, map) = get_map(string);
    let map = iterate_and_update(map, size);
    let mut output = String::new();

    let mut index = 0;
    while index < size {
        if index != 0 {
            output.push_str(",");
        }
        output.push_str(&get_at_index(&map, index).to_string());
        index += 1;
    }

    return output
}

fn find_first_from_string(string: &str) -> i32 {
    let (size, map) = get_map(string);
    let map = iterate_and_update(map, size);
    return get_at_index(&map, 0)
}

fn exercise_2_1()-> i32 {
    let mut f = File::open("./inputs/input2").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    let (size, mut map) = get_map(&content);
    map.insert(1, 12);
    map.insert(2, 2);
    let map = iterate_and_update(map, size);
    return get_at_index(&map, 0)
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_map() {
        let (size, map) = get_map("1,0,0,0,99");
        assert_eq!(*map.get(&0).unwrap(), 1);
        assert_eq!(size, 5);
    }

    #[test]
    fn test_find_first_from_string() {
        assert_eq!(find_first_from_string("1,0,0,0,99"), 2);
        assert_eq!(find_first_from_string("2,3,0,3,99"), 2);
        assert_eq!(find_first_from_string("2,4,4,5,99,0"), 2);
        assert_eq!(find_first_from_string("1,1,1,4,99,5,6,0,99"), 30);
    }

    #[test]
    fn test_string_to_string() {
        assert_eq!(string_to_string("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(string_to_string("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(string_to_string("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(string_to_string("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
        assert_eq!(string_to_string("1,9,10,3,2,3,11,0,99,30,40,50"), "3500,9,10,70,2,3,11,0,99,30,40,50");
    }

    #[test]
    fn test_exercise_2_1(){
        assert_eq!(exercise_2_1(), 3166704)
    }
}
