pub mod primitives {
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    pub struct Vertex {
        pub position: [f32; 3], //Use the same name in the vertex shader
        pub color: [f32; 3],    //Use the same name in the vertex shader
    }
    vulkano::impl_vertex!(Vertex, position, color);

    pub struct Triangle {
        pub mesh: [Vertex; 3],
    }

    impl Triangle {
        pub fn new() -> Triangle {
            Triangle {
                mesh: [
                    Vertex {
                        position: [-0.5, 0.5, 0.0],
                        color: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.0, -0.5, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                ],
            }
        }
    }

    pub struct Rectangle {
        pub mesh: [Vertex; 6],
    }

    impl Rectangle {
        pub fn new() -> Rectangle {
            Rectangle {
                mesh: [
                    Vertex {
                        position: [-0.5, -0.5, 0.0],
                        color: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, -0.5, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [0.5, 0.5, 0.0],
                        color: [1.0, 0.0, 0.0],
                    },
                    Vertex {
                        position: [-0.5, 0.5, 0.0],
                        color: [0.0, 1.0, 0.0],
                    },
                    Vertex {
                        position: [-0.5, -0.5, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                ],
            }
        }
    }
}
