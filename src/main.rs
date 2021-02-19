use std::io;

fn main() {
    let mut input_weight = String::new();
    let result =  io::stdin().read_line(&mut input_weight);

    let mut mars_weight: f32 = calculate_weight_on_mars(100.0);

    mars_weight = mars_weight * 1000.0;

    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}
