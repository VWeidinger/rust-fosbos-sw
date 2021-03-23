pub const PI: f64 = 3.14159265358979323846264338327950288f64; // kannst du nutzen wie eine normale Variable

fn main() {
    // Aufgabe 1
    // Definiere Variable degree (Testwert 270.0 Grad -> Ergbnis sollten sein 4.71 )
    // Rufe Funktion to_radiant auf und speicher den Rückgabe Wert in einer Variable
    // Gebe den Wert in der Console aus, z.B: println!("{} Grad ist ein Bogenmaß von {}", degree, radiant);

    // Mögliche Zusatzaufgabe zur 1
    // Runde das Ergebnis, z.B: (radiant * 100.0).round() / 100.0 oder direkt in der Consolen Ausgabe println!("{:.2}", radiant);
    
    // Aufgabe 2
    // Definiere Variable celsius (Testwert 20° Celsius -> Fahrenheit 68 und Kelvin 293)
    // Rufe Funktion to_other_temperature_units auf und speicher den Rückgabe Wert in einem Tupel
    // Gebe die Werte in der Console aus, z.B: println!("{} Grad Celsius sind {} Fahrenheit und {} Kelvin", celsius, ...); -> Zugrif auf Tupel mit x.0, x.1, ...
}

// Aufgabe 1
// Definiere die Funktion to_radiant --> Achtung es kann nicht mit zwei unterschiedlichen Typen gerechnet werden(z.B.: i64 und f64 -> geht nicht), wähle deswegen einen sinnvollen Typ für die Parameter!
// Berechnung Bogenmaß: degree * PI / 180

// Aufgabe 2
// Definiere die Funktion to_other_temperature_units -> um mehrere Werte gleichzeitig zurückzugeben nutze einen Tupel
// Fahreheit celsius * 1.8 + 32.0; Kelvin celsius + 273.0
