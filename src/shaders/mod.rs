pub mod shaders {
    use nalgebra_glm::{TMat4, identity};

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

    pub mod vertex_shader {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
                #version 450
                layout(location = 0) in vec3 position;
                layout(location = 1) in vec3 color;

                layout(location = 0) out vec3 out_color;

                layout(set = 0, binding = 0) uniform MvpData {
                    mat4 model;
                    mat4 view;
                    mat4 projection;
                } uniforms;
    
                void main() {
                    mat4 worldview = uniforms.view * uniforms.model;
                    gl_Position = uniforms.projection * worldview * vec4(position, 1.0);
                    out_color = color;
                }
            ",
            types_meta: {
                use bytemuck::{Pod, Zeroable};

                #[derive(Clone, Copy, Zeroable, Pod)]
            },
        }
    }

    pub mod fragment_shader {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
                #version 450
                layout(location = 0) in vec3 in_color;

                layout(location = 0) out vec4 f_color;
    
                void main() {
                    f_color = vec4(in_color, 1.0);
                }
            "
        }
    }
}
