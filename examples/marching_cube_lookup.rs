use cgmath::{point3, Point3};
use rust_3d_tests::mesh::Mesh;


fn main() {
    let cube = cube();
    let faces = cube.face_indices();

    for code in 0..256 {
        let inside: Vec<_> = (0..8).map(|bit| code & (1 << bit) > 0).collect();
        println!("{code} -> {inside:?}");

        //let mut edges = [];

        for f in faces.iter() {
            if let Some(p) = (0..4).position(|i|
                !inside[f[i]] && inside[f[(i + 1) % 4]]
            ) {
                let f: Vec<_> = f[p..].iter().chain(f[..p].iter()).cloned().collect();
                let in_idcs: Vec<_> = (0..4).filter(|&i| inside[f[i]]).collect();
                println!("  {f:?}: {in_idcs:?}");

                match in_idcs[..] {
                    [1] => {

                    },
                    [1, 2] => {

                    },
                    [1, 3] => {

                    },
                    [1, 2, 3] => {

                    },
                    _ => {
                        panic!()
                    },
                }
            }
        }
    }
}


fn cube() -> Mesh<Point3<f64>> {
    Mesh::from_oriented_faces(
        [
            point3(0.0, 0.0, 0.0), // 0
            point3(1.0, 0.0, 0.0), // 1
            point3(0.0, 1.0, 0.0), // 2
            point3(1.0, 1.0, 0.0), // 3
            point3(0.0, 0.0, 1.0), // 4
            point3(1.0, 0.0, 1.0), // 5
            point3(0.0, 1.0, 1.0), // 6
            point3(1.0, 1.0, 1.0), // 7
        ],
        [
            [0, 2, 3, 1],
            [2, 0, 4, 6],
            [3, 2, 6, 7],
            [1, 3, 7, 5],
            [0, 1, 5, 4],
            [4, 5, 7, 6],
        ]
    )
        .unwrap()
}
