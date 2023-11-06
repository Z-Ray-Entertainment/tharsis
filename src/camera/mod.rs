pub mod camera {
    use nalgebra_glm::{TMat4, identity, perspective, half_pi, ortho};

    #[derive(Debug, Clone)]
    pub struct MVP {
        pub model: TMat4<f32>,
        pub view: TMat4<f32>,
        pub projection: TMat4<f32>,
    }

    impl MVP {
        pub fn new() -> MVP {
            MVP {
                model: identity(),
                view: identity(),
                projection: identity(),
            }
        }
    }

    fn calc_aspect_ratio(width: f32, height: f32) -> f32{
        return width / height;
    }

    pub fn calc_perspective(width: f32, height: f32) -> TMat4<f32> {
        let aspect = calc_aspect_ratio(width, height);
        return perspective(aspect, half_pi(), 0.01, 100.0);
    }

    pub fn calc_ortho(width: f32, height: f32) -> TMat4<f32> {
        return ortho(0.0, width, 0.0, height, 0.01, 100.0);
    }
}