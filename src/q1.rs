
enum TrafficLight {
    Green,
    Yellow,
    Red,
}

trait LightDuration {
   fn duration_time(&self) -> u32;
}

impl LightDuration for TrafficLight {
    fn duration_time(&self) -> u32 {
        match self {
            TrafficLight::Green => 25,
            TrafficLight::Yellow => 5,
            TrafficLight::Red => 30,
        }
    }
}

fn print_duration(light: &impl LightDuration) {
    println!("{}", light.duration_time())
}

fn main() {
    print_duration(&TrafficLight::Green);
    print_duration(&TrafficLight::Yellow);
    print_duration(&TrafficLight::Red);
}

