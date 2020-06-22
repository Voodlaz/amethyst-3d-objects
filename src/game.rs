use amethyst::{
    assets::{ Handle, Loader},
    core::{math::Vector3, Transform, TransformBundle},
    ecs::WorldExt,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        mtl::{Material, MaterialDefaults},
        palette::{Srgb, Srgba},
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        rendy::{texture::palette::load_from_srgba},
        types::{DefaultBackend, Mesh},
        RenderingBundle,
    },
    utils::application_root_dir,
};

use crate::objects::{camera, light, sphere};
pub struct Game;

impl SimpleState for Game {
    fn on_start(&mut self,data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;
        world.insert(0usize);

        camera::init_camera(world);
        light::init_light(world);

        // Add custom cube object to scene
        let (mesh, mtl) = {
            let mat_defaults = world.read_resource::<MaterialDefaults>();
            let loader = world.read_resource::<Loader>();

            let meshes = &world.read_resource();
            let textures = &world.read_resource();
            let materials = &world.read_resource();

            let mesh: Handle<Mesh> = loader.load("sphere_mesh.obj", sphere::ObjMesh, (), meshes);
            let albedo = loader.load_from_data(
                load_from_srgba(Srgba::new(0.1, 0.5, 0.3, 1.0)).into(),
                (),
                textures,
            );
            let mat: Handle<Material> = loader.load_from_data(
                Material {
                    albedo,
                    ..mat_defaults.0.clone()
                },
                (),
                materials,
            );

            (mesh, mat)
        };

        let mut transform = Transform::default();
        transform.set_translation_xyz(-5.0, 0.0, 0.0);

        transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
        world
            .create_entity()
            .with(mesh)
            .with(mtl)
            .with(transform)
            .build();
    }
    }
