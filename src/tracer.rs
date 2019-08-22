use crate::{Camera, World};

#[derive(Debug)]
enum TraceType {
    Fast,
    Incremental,
}

impl Default for TraceType {
    fn default() -> Self {
        TraceType::Fast
    }
}

trait TracerProgress {
    fn init(&mut self, finish: u32);
    fn increment(&mut self);
}

#[derive(Default)]
pub struct Tracer<P> {
    progress: Option<bool>,
    trace_type: TraceType,

    camera: Camera,
    world: World,
}

impl<P> Tracer<P> {
    pub fn new() -> Tracer<P> {
        Tracer::default()
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_trace_type(&mut self, trace_type: TraceType) {
        self.trace_type = trace_type;
    }

    pub fn set_world(&mut self, world: World) {
        self.world = world;
    }


}