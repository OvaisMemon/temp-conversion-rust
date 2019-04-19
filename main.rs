fn main() {
    let temp = 98.6;
    let degree = 'C';

    let result = temp_conv(temp, degree);

    println!("The equivalent of {}{} is: {}", temp, degree, result);
}


fn temp_conv(value:f32, degree:char) -> f32 {
    let mut temperature:f32 = 0.0;

    if degree == 'C' {
        temperature = (value - 32.0) * (5.0/9.0);
    }
    else if degree == 'F' {
        temperature = (value * 9.0/5.0) + 32.0;
    }
    else {
        temperature = 0.0;
    };

    temperature
}