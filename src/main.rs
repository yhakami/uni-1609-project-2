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
    
    const PI: f32 = 3.14159265359;
    const max_torque: f32 = 1.18; // 2.16 Ncm; 1.18 Ncm
    const max_speed: f32 = 9.7*(10 as f32); // RPM

    println!("Max torque: {} Ncm", torque(max_torque, gear_ratio(5.0, 6.0)));
    println!("Gear ratio: {}", gear_ratio(5.0, 6.0));
}