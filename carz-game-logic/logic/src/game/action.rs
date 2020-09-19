// USE CLOSURE TO ROLL FOR THRUST/REVERSE AND LEFT/RIGHT for ai move
pub struct Action {
    thrust: bool,
    reverse: bool,
    left: bool,
    right: bool
}


impl Action {
    pub fn new() -> Action {
        Action {
            thrust: false,
            reverse: false,
            left: false,
            right: false
        }
    }
}

pub fn get_all_valid_action_combinations() -> Vec<String> {
    vec!["thrust".into(), "reverse".into(), "left".into(), "right".into()] 
}