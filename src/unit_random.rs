use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref UNIT_UNIFORM: Uniform<f32> = { Uniform::new(0.0, 1.0) };
}

pub fn unit_random() -> f32 {
    thread_rng().sample(*UNIT_UNIFORM)
}
