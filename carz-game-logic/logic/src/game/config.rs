use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub mass: f64,                             // kg
	pub inertia_scale: f64,                    // Multiply by mass for inertia
	pub half_width: f64,                       // Centre to side of chassis (metres)
	pub center_of_gravity_to_front: f64,       // Centre of gravity to front of chassis (metres)
	pub center_of_gravity_to_rear: f64,        // Centre of gravity to rear of chassis
	pub center_of_gravity_to_front_axle: f64,  // Centre gravity to front axle
	pub center_of_gravity_to_rear_axle: f64,   // Centre gravity to rear axle
	pub center_of_gravity_height: f64,         // Centre gravity height
	pub wheel_radius: f64,                     // Includes tire (also represents height of axle)
	pub wheel_width: f64,                      // Used for render only
	pub tire_grip: f64,                        // How much grip tires have
	pub lock_grip: f64,                        // % of grip available when wheel is locked
	pub engine_force: f64,
	pub brake_force: f64,
	pub e_brake_force: f64,
	pub weight_transfer: f64,                  // How much weight is transferred during acceleration/braking
	pub max_steer: f64,                        // Maximum steering angle in radians
	pub corner_stiffness_front: f64,
	pub corner_stiffness_rear: f64,
	pub air_resist: f64,	                   // air resistance (* vel)
    pub roll_resist: f64	                   // rolling resistance force (* vel)
}

