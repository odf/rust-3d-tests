use cgmath::vec3;
use three_d::Mat4;


fn main() {
    wrapper();
}


#[cfg(not(feature = "pprof"))]
fn wrapper() {
    run();
}


#[cfg(feature = "pprof")]
fn wrapper() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build().unwrap();

    build_mesh();

    if let Ok(report) = guard.report().build() {
        let file = std::fs::File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}


fn run() {
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
        vec3(2.0, 3.0, 10.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        three_d::degrees(20.0),
        0.1,
        1000.0,
    );

    let mut control = three_d::OrbitControl::new(
        vec3(0.0, 0.0, 0.0), 1.0, 1000.0
    );

    let models = build_mesh(&context);

    let mut sun = three_d::DirectionalLight::new(
        &context,
        2.0,
        three_d::Srgba::WHITE,
        three_d::vec3(-1.0, -0.5, -1.0)
    );

    sun.generate_shadow_map(2048, &models);

    let ambient = three_d::AmbientLight::new(
        &context,
        0.1,
        three_d::Srgba::WHITE,
    );

    window.render_loop(move |mut frame_input| {
        // This ensures a correct viewport after a window resize.
        camera.set_viewport(frame_input.viewport);

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        frame_input.screen()
            .clear(three_d::ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&camera, &models, &[&sun, &ambient]);

        // Ensures a valid return value.
        three_d::FrameOutput::default()
    });
}


fn build_mesh(context: &three_d::Context)
    -> Vec<three_d::Gm<three_d::InstancedMesh, three_d::PhysicalMaterial>>
{
    let mut result = vec![];

    let shift = |x, y, z| Mat4::from_translation(vec3(x, y, z));

    let rot_y = Mat4::from_angle_y(three_d::degrees(90.0));
    let rot_z = Mat4::from_angle_z(three_d::degrees(90.0));
    let center = shift(-1.0, 0.0, 0.0) * Mat4::from_nonuniform_scale(2.0, 0.1, 0.1);

    let strut_instances = three_d::Instances {
        transformations: vec![
            shift(0.0, -1.0, -1.0) * center,
            shift(0.0, -1.0,  1.0) * center,
            shift(0.0,  1.0, -1.0) * center,
            shift(0.0,  1.0,  1.0) * center,

            rot_y * shift(0.0, -1.0, -1.0) * center,
            rot_y * shift(0.0, -1.0,  1.0) * center,
            rot_y * shift(0.0,  1.0, -1.0) * center,
            rot_y * shift(0.0,  1.0,  1.0) * center,

            rot_z * shift(0.0, -1.0, -1.0) * center,
            rot_z * shift(0.0, -1.0,  1.0) * center,
            rot_z * shift(0.0,  1.0, -1.0) * center,
            rot_z * shift(0.0,  1.0,  1.0) * center,
        ],
        ..Default::default()
    };

    result.push(three_d::Gm::new(
        three_d::InstancedMesh::new(
            &context,
            &strut_instances,
            &three_d::CpuMesh::cylinder(24)
        ),
        three_d::PhysicalMaterial {
            albedo: three_d::Srgba::GREEN,
            metallic: 0.0,
            roughness: 0.5,
            ..Default::default()
        }
    ));

    let scale = Mat4::from_scale(0.105);

    let ball_instances = three_d::Instances {
        transformations: vec![
            shift(-1.0, -1.0, -1.0) * scale,
            shift(-1.0, -1.0,  1.0) * scale,
            shift(-1.0,  1.0, -1.0) * scale,
            shift(-1.0,  1.0,  1.0) * scale,
            shift( 1.0, -1.0, -1.0) * scale,
            shift( 1.0, -1.0,  1.0) * scale,
            shift( 1.0,  1.0, -1.0) * scale,
            shift( 1.0,  1.0,  1.0) * scale,
        ],
        ..Default::default()
    };

    result.push(three_d::Gm::new(
        three_d::InstancedMesh::new(
            &context,
            &ball_instances,
            &three_d::CpuMesh::sphere(24)
        ),
        three_d::PhysicalMaterial {
            albedo: three_d::Srgba::RED,
            metallic: 0.0,
            roughness: 0.5,
            ..Default::default()
        }
    ));

    result
}
