//Written by Yazeed M. Hakami, for BE1609 Project 2's calculations.
#![warn(dead_code)]
fn gear_ratio(teeth_input: f32, teeth_output: f32) -> f32
{
   return teeth_input / teeth_output;
}

fn torque(torque_input: f32, gear_ratio: f32) -> f32
{
    return torque_input * gear_ratio;
}

//5.5; for the wheel's bore length.
// plan: 5 > 6 > 7 > 10 teeth

fn main()
{
    //in the parenthesis, please put the number of teeth on the teeth_input and teeth_output
    let teeth_input = (52) as f32;
    let teeth_output = (11) as f32;
    let ratio = gear_ratio(teeth_input, teeth_output);
    println!("Gear ratio: {}", ratio);
}