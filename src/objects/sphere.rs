use amethyst::{
    error::Error,
    assets::{Format as AssetFormat},
    prelude::*,
    renderer::{
        palette::{Srgb},
        rendy::{
            mesh::{MeshBuilder, Normal, Position, TexCoord},
        },
        types::{MeshData},
    }
};

use std::io::Cursor;
use obj::{load_obj, Obj};

#[derive(Clone, Debug)]
pub struct ObjMesh;

impl AssetFormat<MeshData> for ObjMesh {
    fn name(&self) -> &'static str {
        "OBJ"
    }

    /// Reads the given bytes and produces asset data.
    fn import_simple(&self, bytes: Vec<u8>) -> Result<MeshData, Error> {
        let input = Cursor::new(bytes);
        let sphere: Obj = load_obj(input)?;

        let capacity = sphere.vertices.len() * 3;
        let mut pos = Vec::with_capacity(capacity);
        let mut norm = Vec::with_capacity(capacity);
        let mut tex = Vec::with_capacity(capacity);

        for vertex in sphere.vertices {
            pos.push(Position([
                vertex.position[0],
                vertex.position[1],
                vertex.position[2],
            ]));

            norm.push(Normal([
                vertex.normal[0],
                vertex.normal[1],
                vertex.normal[2]
            ]));
            tex.push(TexCoord([0.0, 0.0]));
        }

        Ok(MeshBuilder::new()
            .with_vertices(pos)
            .with_vertices(norm)
            .with_vertices(tex)
            .into())
    }
}
