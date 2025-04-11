use three_d_asset as asset;
use three_d::{self, Geometry};


#[allow(dead_code)]
pub fn main() {
    // On the web, this creates a canvas instead.
    let window = three_d::Window::new(three_d::WindowSettings {
        title: "Rust 3d Test".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    let context = window.gl();

    let mut camera = three_d::Camera::new_perspective(
        window.viewport(),
        asset::vec3(0.0, 0.0, 8.0),
        asset::vec3(0.0, 0.0, 0.0),
        asset::vec3(0.0, 1.0, 0.0),
        asset::degrees(25.0),
        0.1,
        10.0,
    );

    // Model construction also transfers the mesh data to the GPU.
    let mut model = three_d::Gm::new(
        three_d::Mesh::new(&context, &tetra_mesh()),
        three_d::PhysicalMaterial {
            albedo: asset::Srgba::BLUE,
            metallic: 0.0,
            roughness: 0.5,
            ..Default::default()
        }
    );

    model.set_animation(|time|
        asset::Mat4::from_angle_y(asset::radians(time * 0.001))
    );

    let sun = three_d::DirectionalLight::new(
        &context,
        2.0,
        asset::Srgba::WHITE,
        asset::vec3(1.0, -1.0, -1.0)
    );

    let ambient = three_d::AmbientLight::new(
        &context,
        0.1,
        asset::Srgba::WHITE,
    );

    window.render_loop(move |frame_input| {
        // This ensures a correct viewport after a window resize.
        camera.set_viewport(frame_input.viewport);

        model.animate(frame_input.accumulated_time as f32);

        frame_input.screen()
            .clear(three_d::ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &model, &[&sun, &ambient]);

        // Ensures a valid return value.
        three_d::FrameOutput::default()
    });
}


fn tetra_mesh() -> three_d::CpuMesh {
    let positions: Vec<_> = vec![
        ( 1.0,  1.0,  1.0),
        ( 1.0, -1.0, -1.0),
        (-1.0,  1.0, -1.0),
        (-1.0, -1.0,  1.0)
    ].iter().map(|&(x, y, z)| asset::vec3(x, y, z)).collect();

    let indices = vec![
        0, 1, 2,
        1, 0, 3,
        2, 1, 3,
        0, 2, 3,
    ];

    let mut mesh = three_d::CpuMesh {
        positions: asset::Positions::F32(positions),
        indices: asset::Indices::U8(indices),
        ..Default::default()
    };
    mesh.compute_normals();
    mesh
}
