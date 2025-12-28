
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

pub fn mb_to_inhg(mb: f64) -> f64 {
    // 1 mb = 0.0295299830714 inHg
    mb * 0.0295299830714
}

pub fn kph_to_mph(kph: f64) -> f64 {
    // 1 km = 0.62137119 miles
    kph * 0.62137119
}

pub fn degrees_to_cardinal(degrees: f64) -> &'static str {
    // 1. Normalize degrees to 0.0 - 360.0
    let degrees = degrees % 360.0;

    // 2. Define the 16 directions
    let directions = [
        "N", "NNE", "NE", "ENE",
        "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW",
        "W", "WNW", "NW", "NNW"
    ];

    // 3. Each segment is 22.5 degrees wide (360 / 16)
    // We add 11.25 (half a segment) so that "N" covers the range 348.75 to 11.25
    let index = ((degrees + 11.25) / 22.5) as usize;

    // 4. Use modulo 16 to wrap the 360/0 boundary back to "N"
    directions[index % 16]
}