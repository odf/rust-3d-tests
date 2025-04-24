use cgmath::{point3, EuclideanSpace, Point3};
use three_d_asset as asset;
use three_d::Geometry;

use rust_3d_tests::mesh::Mesh;


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
        asset::vec3(0.0, 2.0, 8.0),
        asset::vec3(0.0, 0.0, 0.0),
        asset::vec3(0.0, 1.0, 0.0),
        asset::degrees(25.0),
        0.1,
        10.0,
    );

    // Model construction also transfers the mesh data to the GPU.
    let mut model = three_d::Gm::new(
        three_d::Mesh::new(
            &context,
            &saddle_mesh()
                .subd(true).tightened(true)
                .subd(true).tightened(true)
                .subd(true).tightened(true)
                .to_cpu_mesh()
        ),
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


fn tetra_mesh() -> Mesh<Point3<f64>> {
    Mesh::from_oriented_faces(
        [
            point3( 1.0,  1.0,  1.0),
            point3( 1.0, -1.0, -1.0),
            point3(-1.0,  1.0, -1.0),
            point3(-1.0, -1.0,  1.0)
        ],
        [
            [0, 1, 2],
            [1, 0, 3],
            [2, 1, 3],
            [0, 2, 3],
        ]
    )
        .unwrap()
}


fn cube_mesh() -> Mesh<Point3<f64>> {
    Mesh::from_oriented_faces(
        [
            point3( 1.0,  1.0,  1.0), // 0
            point3( 1.0,  1.0, -1.0), // 1
            point3( 1.0, -1.0,  1.0), // 2
            point3( 1.0, -1.0, -1.0), // 3
            point3(-1.0,  1.0,  1.0), // 4
            point3(-1.0,  1.0, -1.0), // 5
            point3(-1.0, -1.0,  1.0), // 6
            point3(-1.0, -1.0, -1.0), // 7
        ],
        [
            [0, 4, 6, 2],
            [1, 3, 7, 5],
            [0, 2, 3, 1],
            [2, 6, 7, 3],
            [6, 4, 5, 7],
            [4, 0, 1, 5],
        ]
    )
        .unwrap()
}


fn saddle_mesh() -> Mesh<Point3<f64>> {
    Mesh::from_oriented_faces(
        [
            point3( 0.0,  0.0,  0.0), // 0
            point3( 1.0,  1.0,  1.0), // 1
            point3(-1.0,  1.0,  1.0), // 2
            point3(-1.0, -1.0,  1.0), // 3
            point3(-1.0, -1.0, -1.0), // 4
            point3( 1.0, -1.0, -1.0), // 5
            point3( 1.0,  1.0, -1.0), // 6
        ],
        [
            [0, 1, 2],
            [0, 2, 3],
            [0, 3, 4],
            [0, 4, 5],
            [0, 5, 6],
            [0, 6, 1],
        ]
    )
        .unwrap()
}


fn octa_mesh() -> Mesh<cgmath::Point3<f64>>  {
    Mesh::from_oriented_faces(
        [
            point3( 1.0,  0.0,  0.0),
            point3( 0.0,  1.0,  0.0),
            point3( 0.0,  0.0,  1.0),
            point3(-1.0,  0.0,  0.0),
            point3( 0.0, -1.0,  0.0),
            point3( 0.0,  0.0, -1.0),
        ],
        [
            [ 0, 1, 2 ],
            [ 1, 0, 5 ],
            [ 2, 1, 3 ],
            [ 0, 2, 4 ],
            [ 3, 5, 4 ],
            [ 5, 3, 1 ],
            [ 4, 5, 0 ],
            [ 3, 4, 2 ],
        ]
    )
        .unwrap()
}


trait ToCpuMesh {
    fn to_cpu_mesh(&self) -> three_d::CpuMesh;
}


impl ToCpuMesh for Mesh<Point3<f64>> {
    fn to_cpu_mesh(&self) -> three_d::CpuMesh {
        let trimesh = self.triangulate().unwrap();

        let positions: Vec<_> = trimesh.vertices().iter()
            .map(|&p| p.to_vec())
            .collect();

        let indices: Vec<_> = trimesh.face_indices().iter()
            .flatten()
            .map(|&x| x as u32)
            .collect();

        let mut mesh = three_d::CpuMesh {
            positions: asset::Positions::F64(positions),
            indices: asset::Indices::U32(indices),
            ..Default::default()
        };
        mesh.compute_normals();
        mesh
    }
}
