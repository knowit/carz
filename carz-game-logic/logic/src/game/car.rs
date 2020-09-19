use vector2d::Vector2D;
use super::config::Config;
use super::action::Action;

pub struct Car {
    id: i32,
    control: Control,
    config: Config,
    heading: f64,
    position: Vector2D<i32>,
    velocity: Vector2D<f64>,            // m/s in world coords
	velocity_local: Vector2D<f64>,      // m/s in local car coords (x is forward y is sideways)
	accel: Vector2D<f64>,               // acceleration in world coords
	accel_local: Vector2D<f64>,         // accleration in local car coords
	abs_vel: f64,                        // absolute velocity m/s
	yaw_rate: f64,                       // angular velocity in radians
	steer: f64,	                        // amount of steering input (-1.0..1.0)
    steer_angle: f64,                    // actual front wheel steer angle (-maxSteer..maxSteer)
    action: Action,                     // Current action performed by car

	//  Other values to be computed from config
	inertia: f64,                       // will be = mass
	wheel_base: f64,                     // set from axle to CG lengths
	axle_weight_ratio_front: f64,          // % car weight on the front axle
	axle_weight_ratio_rear: f64,           // % car weight on the rear axle
}

impl Car {
    pub fn new_default(config: Config, control: Control, id: i32) -> Car {
        get_default_car(config, control, id)
    }

    pub fn set_action() {

    }
}


fn get_default_car(config: Config, control: Control, id: i32) -> Car {
    let inertia = config.mass * config.inertia_scale;
    let wheel_base = config.center_of_gravity_to_front_axle + config.center_of_gravity_to_rear_axle;         
	let axle_weight_ratio_front = config.center_of_gravity_to_rear_axle / wheel_base;
	let axle_weight_ratio_rear = config.center_of_gravity_to_front_axle / wheel_base;
    Car {
        id: id,
        control: control,
        config: config,
        heading: 0.0,
        position: Vector2D::new(100, 100),
        velocity: Vector2D::new(0.0, 0.0),          
        velocity_local: Vector2D::new(0.0, 0.0),    
        accel: Vector2D::new(0.0, 0.0),             
        accel_local: Vector2D::new(0.0, 0.0),       
        abs_vel: 0.0,                     
        yaw_rate: 0.0,                   
        steer: 0.0,	                      
        steer_angle: 0.0,
        action: Action::new(),                  
        inertia: inertia,                      
        wheel_base: wheel_base,                  
        axle_weight_ratio_front: axle_weight_ratio_front,     
        axle_weight_ratio_rear: axle_weight_ratio_rear     
    }
}

pub enum Control {
    HUMAN,
    AI
}

