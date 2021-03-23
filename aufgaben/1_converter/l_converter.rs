pub const PI: f64 = 3.14159265358979323846264338327950288f64;

fn main() {
    let degree= 270.0;
    let radiant = to_radiant(degree);
    println!("{} Grad ist ein Bogenmaß von {:.2}", degree, radiant);
    
    let celsius = 20.0;
    let tupel= to_other_temperature_units(celsius);
    println!("{} Grad Celsius sind {} Fahrenheit und {} Kelvin", celsius, tupel.0, tupel.1);
}


fn to_radiant(degree: f64) -> f64 {
    degree * PI / 180.0
}


fn to_other_temperature_units(celsius: f64) -> (f64, f64) {
    (celsius * 1.8 + 32.0, celsius + 273.0) // Fahrenheit, Kelvin
}