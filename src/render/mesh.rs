// use std::rc::Rc;
use std::path::Path;

use gltf;
use gltf_importer;

use shader::Shader;
use render::math::*;
use render::{Primitive, Scene};

pub struct Mesh {
    pub index: usize, // glTF index
    pub primitives: Vec<Primitive>,
    // TODO: weights
    // pub weights: Vec<Rc<?>>
    pub name: Option<String>,

    pub bounds: Bounds,
}

impl Mesh {
    pub fn from_gltf(
        g_mesh: gltf::Mesh,
        scene: &mut Scene,
        buffers: &gltf_importer::Buffers,
        base_path: &Path,
    ) -> Mesh {
        let primitives: Vec<Primitive> = g_mesh.primitives()
            .enumerate()
            .map(|(i, g_prim)| {
                Primitive::from_gltf(g_prim, i, g_mesh.index(), scene, buffers, base_path)
            })
            .collect();

        let bounds = primitives.iter()
            .fold(Bounds::default(), |bounds, prim| prim.bounds.union(&bounds));

        Mesh {
            index: g_mesh.index(),
            primitives: primitives,
            name: g_mesh.name().map(|s| s.into()),
            bounds,
        }
    }

    pub fn draw(&self, shader: &mut Shader,
        model_matrix: &Matrix4, mvp_matrix: &Matrix4, camera_position: &Vector3)
    {
        for primitive in &self.primitives {
            unsafe { primitive.draw(shader, model_matrix, mvp_matrix, camera_position) }
        }
    }
}
