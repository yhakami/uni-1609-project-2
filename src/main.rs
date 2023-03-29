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
// plan: 5 > 6 > 7 > 10 teeth >> ended up using something else.
fn time_to_reach_top(
    weight: f32,
    torque: f32,
    speed: f32,
    gear_ratio: f32,
    distance: f32
) -> f32
{
    let force = weight * 9.81; // 9.81 in this context is G.
    let torque = torque * gear_ratio; // 0.044 kg/cm
    let speed = speed * gear_ratio;
    let time = distance / speed;
    return time;
}
fn main()
{
    
    const PI: f32 = 3.14159265359;
    const max_torque: f32 = 1.18; // 2.16 Ncm; 1.18 Ncm
    const max_speed: f32 = 9.7*(10 as f32); // RPM
    const weight: f32 = 4.4;
    const motor_torque: f32 = 0.5;

    println!("Max torque: {} Ncm", torque(max_torque, gear_ratio(5.0, 6.0)));
    println!("Gear ratio: {}", gear_ratio(5.0, 6.0)); // starting to think I should've used Python instead.
}