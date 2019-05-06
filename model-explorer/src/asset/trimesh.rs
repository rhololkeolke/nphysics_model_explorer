use amethyst::{
    core::nalgebra as ana,
    renderer::{MeshData, PosColor, PosNormTex, PosTex},
};
use itertools::izip;
use ncollide3d::procedural::TriMesh;

/// Convert a ncollide3d TriMesh to a MeshData asset that can be
/// displayed.
pub fn to_mesh_data(trimesh: &TriMesh<f32>) -> MeshData {
    // TODO(dschwab): How to deal with color vertex types?
    match (trimesh.normals.as_ref(), trimesh.uvs.as_ref()) {
        (Some(normals), Some(uvs)) => {
            let vertices: Vec<PosNormTex> = izip!(&trimesh.coords, normals, uvs)
                .map(|(coord, norm, uv)| PosNormTex {
                    position: ana::Vector3::new(coord.x, coord.y, coord.z),
                    tex_coord: ana::Vector2::new(uv.x, uv.y),
                    normal: ana::Vector3::new(norm.x, norm.y, norm.z),
                })
                .collect();
            MeshData::PosNormTex(vertices)
        }
        (Some(normals), None) => {
            let vertices: Vec<PosNormTex> = izip!(&trimesh.coords, normals)
                .map(|(coord, norm)| PosNormTex {
                    position: ana::Vector3::new(coord.x, coord.y, coord.z),
                    tex_coord: ana::Vector2::zeros(),
                    normal: ana::Vector3::new(norm.x, norm.y, norm.z),
                })
                .collect();
            MeshData::PosNormTex(vertices)
        }
        (None, Some(uvs)) => {
            let vertices: Vec<PosTex> = izip!(&trimesh.coords, uvs)
                .map(|(coord, uv)| PosTex {
                    position: ana::Vector3::new(coord.x, coord.y, coord.z),
                    tex_coord: ana::Vector2::new(uv.x, uv.y),
                })
                .collect();
            MeshData::PosTex(vertices)
        }
        (None, None) => {
            let vertices: Vec<PosColor> = trimesh
                .coords
                .iter()
                .map(|coord| PosColor {
                    position: ana::Vector3::new(coord.x, coord.y, coord.z),
                    color: [0.5, 0.5, 0.5, 1.0],
                })
                .collect();
            MeshData::PosColor(vertices)
        }
    }
}
