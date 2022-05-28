const LOWER: f64 = 0.0;
const UPPER: f64 = 300.0;
const STEP: f64 = 20.0;

fn main() {
    // Formula for Fahrenheit to Celsius is °C = [(°F-32)×5]/9
    let mut farenheit: f64 = LOWER;
    let mut celsius: f64;
    while farenheit <= UPPER {
        celsius = ((farenheit - 32.0) * 5.0) / 9.0;
        println!("{}°F is {}°C", farenheit, celsius);
        farenheit += STEP;
    }
}
