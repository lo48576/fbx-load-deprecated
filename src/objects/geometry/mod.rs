//! Contains `/Objects/Geometry` node-related stuff.

pub use self::mesh::{Mesh, VertexIndex, MappingMode, ReferenceMode, LayerElement};
pub use self::shape::Shape;

use std::io::Read;
use fbx_binary_reader::EventReader;
use definitions::Definitions;
use error::Result;
use node_loader::{NodeLoader, NodeLoaderCommon, RawNodeInfo};
use objects::properties::ObjectProperties;
use self::mesh::MeshLoader;
use self::shape::ShapeLoader;

pub mod mesh;
pub mod shape;


#[derive(Debug, Clone)]
pub enum Geometry {
    Mesh(Mesh),
    Shape(Shape),
}

#[derive(Debug)]
pub enum GeometryLoader<'a> {
    Mesh(MeshLoader<'a>),
    Shape(ShapeLoader<'a>),
}

impl<'a> GeometryLoader<'a> {
    pub fn new(definitions: &'a Definitions, obj_props: &'a ObjectProperties<'a>) -> Option<Self> {
        match obj_props.subclass {
            "Mesh" => Some(GeometryLoader::Mesh(MeshLoader::new(definitions, obj_props))),
            "Shape" => Some(GeometryLoader::Shape(ShapeLoader::new(definitions, obj_props))),
            v => {
                warn!("Unknown subclass ({}) for `/Objects/Geometry`", v);
                None
            },
        }
    }
}

impl<'a> NodeLoaderCommon for GeometryLoader<'a> {
    type Target = Option<Geometry>;

    fn on_finish(self) -> Result<Self::Target> {
        Ok(match self {
            GeometryLoader::Mesh(loader) => try!(loader.on_finish()).map(Geometry::Mesh),
            GeometryLoader::Shape(loader) => try!(loader.on_finish()).map(Geometry::Shape),
        })
    }
}

impl<'a, R: Read> NodeLoader<R> for GeometryLoader<'a> {
    fn on_child_node(&mut self, reader: &mut EventReader<R>, node_info: RawNodeInfo) -> Result<()> {
        match *self {
            GeometryLoader::Mesh(ref mut loader) => loader.on_child_node(reader, node_info),
            GeometryLoader::Shape(ref mut loader) => loader.on_child_node(reader, node_info),
        }
    }
}
