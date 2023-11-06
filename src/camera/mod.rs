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

    pub fn calc_presentation(width: f32, height: f32, ortho: bool) -> TMat4<f32>{
        if ortho {
            return calc_ortho(width, height);
        } else {
            return calc_perspective(width, height);
        }
    }

    fn calc_aspect_ratio(width: f32, height: f32) -> f32{
        if height <= 0.0 {
            return width / 1.0;
        } else {
            return width / height;
        }
    }

    fn calc_perspective(width: f32, height: f32) -> TMat4<f32> {
        let aspect = calc_aspect_ratio(width, height);
        return perspective(aspect, half_pi(), 0.01, 100.0);
    }

    fn calc_ortho(width: f32, height: f32) -> TMat4<f32> {
        return ortho(0.0, width, height, 0.0, 1.0, 100.0);
    }
}