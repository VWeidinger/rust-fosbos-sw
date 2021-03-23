pub const PI: f64 = 3.14159265358979323846264338327950288f64;

fn main() {
    // 270 Grad -> Bogenmass(d * PI / 180) -> Ausgabe
    let degree= 270.0;
    let radiant = to_radiant(degree);
    // let y = (x * 100.0).round() / 100.0; oder println!("{:.2}", x);
    println!("{} Grad ist ein Bogenmaß von {}", degree, radiant);
    // 20° Celsius -> Fahrenheit(c * 1.8 + 32) und  Kelvin(c + 273)
    let celsius = 20.0;
    let tupel= convert_temperature(celsius);
    println!("{} Grad Celsius sind {} Fahrenheit und {} Kelvin", celsius, tupel.0, tupel.1);
}

// Fuktion Übung
fn to_radiant(degree: f64) -> f64 {
    degree * PI / 180.0
}

// Tupel Übung
fn convert_temperature(celsius: f64) -> (f64, f64) {
    (celsius * 1.8 + 32.0, celsius + 273.0) // Fahrenheit, Kelvin
}