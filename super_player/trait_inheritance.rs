// trait different from interface
// 1. trait can not be implemented by struct
// 2. can write trait implementation blocks anywhere, without having access to the actual type.
// 3. Cannot implicitly have return types as traits in a function like can return an interface as a return type in Java.

// The many forms of trait
// 1. Marker traits: define in the std::marker modeule. These trait don't have any methods.
// 2. Simple traits
// 3. Generic traits
// 4. Associated type traits
// 5. Inherited traits

trait Vehicle {
    fn get_price(&self) -> u32;
}

trait Car: Vehicle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    realease_year: u32,
}

impl TeslaRoadster {
    fn new(model: String, realease_year: u32) -> Self {
        TeslaRoadster {
            model,
            realease_year,
        }
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}

impl Vehicle for TeslaRoadster {
    fn get_price(&self) -> u32 {
        100_000
    }
}

fn main() {
    let roadster = TeslaRoadster::new("Tesla Roadster I".to_string(), 2019);
    println!("{}", roadster.model());
}
