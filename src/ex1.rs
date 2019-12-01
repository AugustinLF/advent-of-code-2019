use std::fs::File;
use std::io::prelude::*;

fn exercise_1() -> i32 {
    let mut f = File::open("./inputs/input1").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    let mut total = 0;

    for freq in content.split_whitespace() {
        if let Ok(freq_number) = freq.parse::<i32>() {
            total += get_fuel_for_mass(freq_number)
        }
    }

    return total
}

fn exercise_1_2() -> i32 {
    let mut f = File::open("./inputs/input1").expect("file not found");

    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    let mut total = 0;

    for freq in content.split_whitespace() {
        if let Ok(freq_number) = freq.parse::<i32>() {
            let needed_fuel = get_fuel_for_mass(freq_number);
            total += needed_fuel + compensate_for_fuel(needed_fuel)
        }
    }

    return total
}

fn compensate_for_fuel(mass: i32) -> i32 {
    let fuel = get_fuel_for_mass(mass);

    if fuel <= 0 {
        return 0
    } else {
        return fuel + compensate_for_fuel(fuel);
    }
}


fn get_fuel_for_mass(mass: i32) -> i32 {
    return (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_fuel_for_mass() {
        assert_eq!(2, get_fuel_for_mass(12));
        assert_eq!(2, get_fuel_for_mass(14));
    }
    #[test]
    fn test_exercise_1() {
        assert_eq!(3398090, exercise_1());
    }
    #[test]
    fn test_exercise_1_2() {
        assert_eq!(5094261, exercise_1_2());
    }
}
