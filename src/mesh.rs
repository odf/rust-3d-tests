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

        let boundary_lists = boundary_cycles(boundary_edges);

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

    pub fn from_oriented_faces<
        I1: IntoIterator<Item=T>,
        I2: IntoIterator<Item=usize>,
        I3: IntoIterator<Item=I2>
    >(
        vertices: I1,
        face_lists_in: I3
    ) -> Result<Mesh<T>, String> {
        let vertices: Vec<_> = vertices.into_iter().collect();
        let face_lists: Vec<Vec<_>> = face_lists_in.into_iter()
            .map(|f| f.into_iter().collect())
            .collect();

        let defined_vertices: BTreeSet<_> = (0..vertices.len()).collect();

        let seen_vertices: BTreeSet<_> = face_lists.iter()
            .filter(|f| f.len() > 0)
            .flatten().cloned().collect();

        let oriented_edges: Vec<_> = face_lists.iter()
            .filter(|f| f.len() > 0)
            .map(cyclic_pairs)
            .flatten().collect();

        let oriented_edge_set: BTreeSet<_> = oriented_edges.iter()
            .cloned().collect();

        let boundary_vertices: Vec<_> = oriented_edges.iter()
            .filter(|e| !oriented_edge_set.contains(&opposite(e)))
            .map(|&(v, _)| v)
            .collect();

        if seen_vertices.iter().any(|v| !defined_vertices.contains(v)) {
            Err("an undefined vertex appears in a face".to_string())
        } else if defined_vertices.iter().any(|v| !seen_vertices.contains(v)) {
            Err("some vertex does not appear in any faces".to_string())
        } else if face_lists.iter().any(|f| f.len() < 2) {
            Err("some face has fewer than two vertices".to_string())
        } else if face_lists.iter().any(|f| !all_unique(f)) {
            Err("a vertex appears more than once in the same face".to_string())
        } else if !all_unique(boundary_vertices) {
            Err("a vertex appears more then once in a boundary".to_string())
        } else if !all_unique(oriented_edges) {
            Err("an oriented edge appears more than once".to_string())
        } else {
            Ok(Mesh::from_oriented_faces_unchecked(vertices, face_lists))
        }
    }

    pub fn vertices<'a>(&'a self) -> &'a Vec<T> {
        &self.vertices
    }

    fn vertices_in_face(&self, start: OrientedEdge) -> Vec<usize> {
        canonical_circular(
            trace_cycle(start, |e| self.next.get(&e).copied())
                .iter()
                .map(|&(v, _)| v)
                .collect()
        )
    }

    pub fn face_indices(&self) -> Vec<Vec<usize>> {
        self.along_face.iter()
            .map(|&e| self.vertices_in_face(e))
            .collect::<BTreeSet<_>>().into_iter()
            .collect()
    }

    pub fn boundary_indices(&self) -> Vec<Vec<usize>> {
        self.along_boundary_component.iter()
            .map(|&e| self.vertices_in_face(e))
            .collect::<BTreeSet<_>>().into_iter()
            .collect()
    }

    pub fn edge_indices(&self) -> Vec<(usize, usize)> {
        self.next.keys()
            .map(|&(u, v)| (u.min(v), u.max(v)))
            .collect::<BTreeSet<_>>().into_iter()
            .collect()
    }

    fn vertex_neighbors(&self, start: OrientedEdge) -> Vec<usize> {
        canonical_circular(
            trace_cycle(start, |e| self.next.get(&opposite(&e)).copied())
                .iter()
                .map(|&(_, w)| w)
                .rev()
                .collect()
        )
    }

    pub fn neighbor_indices(&self) -> Vec<Vec<usize>> {
        self.at_vertex.iter()
            .map(|&e| self.vertex_neighbors(e))
            .collect::<BTreeSet<_>>().into_iter()
            .collect()
    }
}


impl<T: Clone> Mesh<T> {
    pub fn triangulate(&self) -> Result<Self, String> {
        Self::from_oriented_faces(
            self.vertices.clone(),
            self.face_indices().iter().flat_map(triangulate)
        )
    }
}


fn boundary_cycles(boundary_edges: Vec<OrientedEdge>)
    -> Vec<Vec<(usize, usize)>>
{
    let items: Vec<_> = boundary_edges.iter().map(|&(v, _)| v).collect();
    let advance: BTreeMap<_, _> = boundary_edges.iter().map(opposite).collect();

    let mut seen: BTreeSet<usize> = BTreeSet::new();
    let mut result = vec![];

    for v in items {
        if !seen.contains(&v) {
            let cycle = trace_cycle(v, |v| advance.get(&v).copied());
            seen.extend(&cycle);
            result.push(cyclic_pairs(&cycle));
        }
    }

    result
}


fn trace_cycle<T: Copy + PartialEq>(v: T, advance: impl Fn(T) -> Option<T>)
    -> Vec<T>
{
    let mut cycle = vec![];
    let mut w = v;

    loop {
        if let Some(u) = advance(w) {
            cycle.push(w);
            w = u;
            if w == v {
                break;
            }
        } else {
            cycle.clear();
            break;
        }
    }

    cycle
}


fn triangulate<T: Copy>(corners: &Vec<T>) -> Vec<Vec<T>> {
    (1..(corners.len() - 1))
        .map(|i| vec![corners[0], corners[i], corners[i + 1]])
        .collect()
}


fn cyclic_pairs<T: Copy>(indices: &Vec<T>) -> Vec<(T, T)> {
    let mut result = vec![];
    for i in 0..(indices.len() - 1) {
        result.push((indices[i], indices[i + 1]))
    }
    result.push((indices[indices.len() - 1], indices[0]));
    result
}


fn canonical_circular<T: Copy + Ord>(list: Vec<T>) -> Vec<T> {
    if list.is_empty() {
        vec![]
    } else {
        (0..list.len()).map(|k|
            list.iter().skip(k).chain(list.iter().take(k)).cloned().collect()
        ).min().unwrap()
    }
}


fn all_unique<T: Ord, I: IntoIterator<Item=T>>(items: I) -> bool {
    let mut seen: BTreeSet<T> = BTreeSet::new();

    for x in items.into_iter() {
        if seen.contains(&x) {
            return false;
        }
        seen.insert(x);
    }

    true
}


#[cfg(test)]
mod test {
    use super::*;

    fn octahedron_vertices() -> [String; 6] {
        [
            "front".to_string(),
            "right".to_string(),
            "top".to_string(),
            "back".to_string(),
            "left".to_string(),
            "bottom".to_string(),
        ]
    }

    fn octahedron_faces() -> [Vec<usize>; 8] {
        [
            vec![ 0, 1, 2 ],
            vec![ 1, 0, 5 ],
            vec![ 2, 1, 3 ],
            vec![ 0, 2, 4 ],
            vec![ 3, 5, 4 ],
            vec![ 5, 3, 1 ],
            vec![ 4, 5, 0 ],
            vec![ 3, 4, 2 ],
        ]
    }

    #[test]
    fn test_cyclic_pairs() {
        assert_eq!(cyclic_pairs(&vec![1, 2, 3]), vec![(1, 2), (2, 3), (3, 1)]);
    }

    #[test]
    fn test_empty() {
        let mesh = Mesh::<i32>::empty();

        assert_eq!(mesh.vertices(), &[]);
        assert_eq!(mesh.edge_indices(), []);
        assert_eq!(mesh.face_indices(), [[]; 0]);
        assert_eq!(mesh.boundary_indices(), [[]; 0]);
        assert_eq!(mesh.neighbor_indices(), [[]; 0]);
    }

    #[test]
    fn test_without_boundary() {
        let octa = Mesh::from_oriented_faces(
            octahedron_vertices(), octahedron_faces()
        ).unwrap();

        assert_eq!(
            octa.edge_indices(),
            [
                ( 0, 1 ),
                ( 0, 2 ),
                ( 0, 4 ),
                ( 0, 5 ),
                ( 1, 2 ),
                ( 1, 3 ),
                ( 1, 5 ),
                ( 2, 3 ),
                ( 2, 4 ),
                ( 3, 4 ),
                ( 3, 5 ),
                ( 4, 5 ),
            ]
        );

        assert_eq!(
            octa.face_indices(),
            [
                [ 0, 1, 2 ],
                [ 0, 2, 4 ],
                [ 0, 4, 5 ],
                [ 0, 5, 1 ],
                [ 1, 3, 2 ],
                [ 1, 5, 3 ],
                [ 2, 3, 4 ],
                [ 3, 5, 4 ],
            ]
        );

        assert_eq!(octa.boundary_indices(), [[]; 0]);

        assert_eq!(
            octa.neighbor_indices(),
            [
                [ 0, 1, 3, 4 ],
                [ 0, 2, 3, 5 ],
                [ 0, 4, 3, 1 ],
                [ 0, 5, 3, 2 ],
                [ 1, 2, 4, 5 ],
                [ 1, 5, 4, 2 ],
            ]
        );
    }

    #[test]
    fn test_with_boundary() {
        let octa = Mesh::from_oriented_faces(
            octahedron_vertices(),
            [
                [ 1, 0, 5 ],
                [ 2, 1, 3 ],
                [ 0, 2, 4 ],
                [ 5, 3, 1 ],
                [ 4, 5, 0 ],
                [ 3, 4, 2 ],
            ],
        ).unwrap();

        assert_eq!(
            octa.edge_indices(),
            [
                ( 0, 1 ),
                ( 0, 2 ),
                ( 0, 4 ),
                ( 0, 5 ),
                ( 1, 2 ),
                ( 1, 3 ),
                ( 1, 5 ),
                ( 2, 3 ),
                ( 2, 4 ),
                ( 3, 4 ),
                ( 3, 5 ),
                ( 4, 5 ),
            ]
        );

        assert_eq!(
            octa.face_indices(),
            [
                [ 0, 2, 4 ],
                [ 0, 4, 5 ],
                [ 0, 5, 1 ],
                [ 1, 3, 2 ],
                [ 1, 5, 3 ],
                [ 2, 3, 4 ],
            ]
        );

        assert_eq!(octa.boundary_indices(), [[ 0, 1, 2 ], [ 3, 5, 4 ]]);

        assert_eq!(
            octa.neighbor_indices(),
            [
                [ 0, 1, 3, 4 ],
                [ 0, 2, 3, 5 ],
                [ 0, 4, 3, 1 ],
                [ 0, 5, 3, 2 ],
                [ 1, 2, 4, 5 ],
                [ 1, 5, 4, 2 ],
            ]
        );
    }

    #[test]
    fn test_undefined_vertex() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices()[1..].into_iter(),
                octahedron_faces()
            ).is_err()
        );
    }

    #[test]
    fn test_unreferenced_vertex() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices().into_iter().chain(["off".to_string()]),
                octahedron_faces()
            ).is_err()
        );
    }

    #[test]
    fn test_vertex_duplicate_in_face() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices(),
                [
                    vec![ 0, 1, 2, 0, 4, 5 ],
                    vec![ 1, 0, 5 ],
                    vec![ 2, 1, 3 ],
                    vec![ 0, 2, 4 ],
                    vec![ 3, 5, 4 ],
                    vec![ 5, 3, 1 ],
                    vec![ 3, 4, 2 ],
                ]
            ).is_err()
        );
    }

    #[test]
    fn test_vertex_duplicate_in_boundary() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices(),
                [
                    [ 1, 0, 5 ],
                    [ 2, 1, 3 ],
                    [ 0, 2, 4 ],
                    [ 3, 5, 4 ],
                    [ 5, 3, 1 ],
                    [ 3, 4, 2 ],
                ]
            ).is_err()
        );
    }

    #[test]
    fn test_empty_face() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices(),
                octahedron_faces().into_iter().chain([vec![]])
            ).is_err()
        );
    }

    #[test]
    fn test_one_gon() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices(),
                octahedron_faces().into_iter().chain([vec![0]])
            ).is_err()
        );
    }

    #[test]
    fn test_single_two_gon() {
        assert!(
            Mesh::from_oriented_faces(['a', 'b'], [[0, 1]]).is_ok()
        );
    }

    #[test]
    fn test_orientation_mismatch() {
        assert!(
            Mesh::from_oriented_faces(
                octahedron_vertices(),
                octahedron_faces().into_iter().skip(1).chain([vec![0, 2, 1]])
            ).is_err()
        );
    }

    #[test]
    fn test_duplicate_edge() {
        assert!(
            Mesh::from_oriented_faces(
                ['a', 'b', 'c', 'd'],
                [
                    [ 2, 0, 1 ],
                    [ 0, 2, 3 ],
                    [ 2, 0, 3 ],
                    [ 0, 2, 1 ],
                ]
                ).is_err()
        );
    }

    #[test]
    fn test_vertices_method() {
        let mesh = Mesh::from_oriented_faces(
            octahedron_vertices(),
            octahedron_faces()
        ).unwrap();

        assert_eq!(mesh.vertices(), &octahedron_vertices());
    }

    #[test]
    fn test_triangulate() {
        assert_eq!(
            triangulate(&vec![0, 1, 2, 3]),
            vec![vec![0, 1, 2], vec![0, 2, 3]]
        );

        assert_eq!(
            triangulate(&vec![3, 2, 1, 0]),
            vec![vec![3, 2, 1], vec![3, 1, 0]]
        );
    }

    #[test]
    fn test_triangulate_mesh() {
        let mesh = Mesh::from_oriented_faces(
            ['a', 'b', 'c', 'd', 'e', 'f'],
            [
                vec![2, 1, 0],
                vec![3, 4, 5],
                vec![0, 1, 4, 3],
                vec![1, 2, 5, 4],
                vec![2, 0, 3, 5],
            ]
        ).unwrap().triangulate().unwrap();

        assert_eq!(mesh.vertices(), &['a', 'b', 'c', 'd', 'e', 'f']);
        assert_eq!(
            mesh.face_indices(),
            [
                [0, 1, 4],
                [0, 2, 1],
                [0, 3, 5],
                [0, 4, 3],
                [0, 5, 2],
                [1, 2, 5],
                [1, 5, 4],
                [3, 4, 5],
            ]
        );
    }
}
