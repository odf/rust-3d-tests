use std::collections::{BTreeMap, BTreeSet};


type OrientedEdge = (usize, usize);


pub fn opposite(e: &OrientedEdge) -> OrientedEdge {
    (e.1, e.0)
}


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

        todo!()
    }
}


fn extract_cycles(items: Vec<usize>, advance: BTreeMap<usize, usize>)
    -> Vec<Vec<usize>>
{
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    let mut result = vec![];

    for v in items {
        if !seen.contains(&v) {
            let mut cycle = vec![v];
            let mut w = v;
            loop {
                match advance.get(&w) {
                    Some(&u) if u == v => break,
                    Some(&u) => {
                        cycle.push(u);
                        seen.insert(u);
                        w = u;
                    },
                    None => {
                        cycle.clear();
                        break;
                    }
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
    use crate::mesh::cyclic_pairs;

    #[test]
    fn test_cyclic_pairs() {
        assert_eq!(cyclic_pairs(&vec![1, 2, 3]), vec![(1, 2), (2, 3), (3, 1)]);
    }
}
