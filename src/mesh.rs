use std::collections::{BTreeMap, BTreeSet};


type OrientedEdge = (usize, usize);


pub fn opposite(e: &OrientedEdge) -> OrientedEdge {
    (e.1, e.0)
}


#[derive(Debug)]
pub struct Mesh<T> {
    vertices: Vec<T>,
    at_vertex: Vec<OrientedEdge>,
    along_face: Vec<OrientedEdge>,
    along_boundary_component: Vec<OrientedEdge>,
    to_face: BTreeMap<OrientedEdge, usize>,
    to_boundary_component: BTreeMap<OrientedEdge, usize>,
    next: BTreeMap<OrientedEdge, OrientedEdge>
}


impl<T> Mesh<T> {
    pub fn empty() -> Mesh<T> {
        Mesh {
            vertices: vec![],
            at_vertex: vec![],
            along_face: vec![],
            along_boundary_component: vec![],
            to_face: BTreeMap::new(),
            to_boundary_component: BTreeMap::new(),
            next: BTreeMap::new(),
        }
    }

    fn from_oriented_faces_unchecked(
        vertices: Vec<T>,
        face_lists: Vec<Vec<usize>>
    ) -> Mesh<T> {
        let oriented_edges_lists: Vec<_> = face_lists.iter()
            .map(cyclic_pairs).collect();

        let oriented_edges: Vec<_> = oriented_edges_lists.iter()
            .flatten().cloned().collect();

        let oriented_edge_set: BTreeSet<_> = oriented_edges.iter()
            .cloned().collect();

        let boundary_edges: Vec<_> = oriented_edges.iter()
            .filter(|e| !oriented_edge_set.contains(&opposite(e)))
            .cloned().collect();

        let boundary_lists = extract_cycles(
            boundary_edges.iter().map(|&(v, _)| v).collect(),
            boundary_edges.iter().map(opposite).collect()
        );

        let at_vertex: Vec<_> = oriented_edges.iter()
            .map(|&(v, w)| (v, (v, w)))
            .collect::<BTreeMap<_, _>>().values().cloned()
            .collect();

        let to_face: BTreeMap<_, _> = oriented_edges_lists.iter()
            .enumerate()
            .flat_map(|(i, f)| f.iter().map(move |&e| (e, i)))
            .collect();

        let along_face: Vec<_> = to_face.iter()
            .map(|(&e, &f)| (f, e))
            .collect::<BTreeMap<_, _>>().iter()
            .map(|(_, &e)| e)
            .collect();

        let to_boundary_component: BTreeMap<_, _> = boundary_lists.iter()
            .enumerate()
            .flat_map(|(i, b)| b.iter().map(move |&e| (e, i)))
            .collect();

        let along_boundary_component: Vec<_> = to_boundary_component.iter()
            .map(|(&e, &b)| (b, e))
            .collect::<BTreeMap<_, _>>().iter()
            .map(|(_, &e)| e)
            .collect();

        let next: BTreeMap<_, _> = oriented_edges_lists.iter()
            .chain(boundary_lists.iter())
            .flat_map(cyclic_pairs)
            .collect();

        Mesh {
            vertices,
            at_vertex,
            along_face,
            along_boundary_component,
            to_face,
            to_boundary_component,
            next,
        }
    }

    pub fn from_oriented_faces(
        vertices: Vec<T>,
        face_lists: Vec<Vec<usize>>
    ) -> Result<Mesh<T>, String> {
        let defined_vertices: BTreeSet<_> = (0..vertices.len()).collect();
        let seen_vertices: BTreeSet<_> = face_lists.iter()
            .flatten().cloned().collect();

        if seen_vertices.iter().any(|v| !defined_vertices.contains(v)) {
            Err("an undefined vertex appears in a face".to_string())
        } else if defined_vertices.iter().any(|v| !seen_vertices.contains(v)) {
            Err("some vertex does not appear in any faces".to_string())
        } else if face_lists.iter().any(|f| f.len() < 2) {
            Err("some face has fewer than two vertices".to_string())
        } else {
            Ok(Mesh::from_oriented_faces_unchecked(vertices, face_lists))
        }
    }
}


fn extract_cycles<T: Copy + Ord>(items: Vec<T>, advance: BTreeMap<T, T>)
    -> Vec<Vec<(T, T)>>
{
    let mut seen: BTreeSet<T> = BTreeSet::new();
    let mut result = vec![];

    for v in items {
        if !seen.contains(&v) {
            let mut cycle = vec![];
            let mut w = v;
            loop {
                if let Some(&u) = advance.get(&w) {
                    cycle.push((w, u));
                    seen.insert(u);
                    w = u;
                    if w == v {
                        break;
                    }
                } else {
                    cycle.clear();
                    break;
                }
            }
            result.push(cycle);
        }
    }

    result
}


fn cyclic_pairs<T: Copy>(indices: &Vec<T>) -> Vec<(T, T)> {
    let mut result = vec![];
    for i in 0..(indices.len() - 1) {
        result.push((indices[i], indices[i + 1]))
    }
    result.push((indices[indices.len() - 1], indices[0]));
    result
}


#[cfg(test)]
mod test {
    use super::*;

    fn octahedron_vertices() -> Vec<String> {
        vec![
            "front".to_string(),
            "right".to_string(),
            "top".to_string(),
            "back".to_string(),
            "left".to_string(),
            "bottom".to_string(),
        ]
    }

    fn octahedron() -> Mesh<String> {
        Mesh::from_oriented_faces(
            octahedron_vertices(),
            vec![
                vec![ 0, 1, 2 ],
                vec![ 1, 0, 5 ],
                vec![ 2, 1, 3 ],
                vec![ 0, 2, 4 ],
                vec![ 3, 5, 4 ],
                vec![ 5, 3, 1 ],
                vec![ 4, 5, 0 ],
                vec![ 3, 4, 2 ],
            ],
        ).unwrap()
    }

    fn octahedron_open() -> Mesh<String> {
        Mesh::from_oriented_faces(
            octahedron_vertices(),
            vec![
                vec![ 1, 0, 5 ],
                vec![ 2, 1, 3 ],
                vec![ 0, 2, 4 ],
                vec![ 5, 3, 1 ],
                vec![ 4, 5, 0 ],
                vec![ 3, 4, 2 ],
            ],
        ).unwrap()
    }

    #[test]
    fn test_cyclic_pairs() {
        assert_eq!(cyclic_pairs(&vec![1, 2, 3]), vec![(1, 2), (2, 3), (3, 1)]);
    }

    #[test]
    fn test_from_unchecked() {
        let octa = octahedron_open();
        println!("vertices: {:?}", octa.vertices);
        println!("at_vertex: {:?}", octa.at_vertex);
        println!("along_face: {:?}", octa.along_face);
        println!("along_boundary_component: {:?}", octa.along_boundary_component);
        println!("to_face: {:?}", octa.to_face);
        println!("to_boundary_component: {:?}", octa.to_boundary_component);
        println!("next: {:?}", octa.next);
    }
}
