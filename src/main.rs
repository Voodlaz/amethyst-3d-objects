use amethyst::{
    assets::{ Handle, Loader},
    core::{math::Vector3, Transform, TransformBundle},
    ecs::WorldExt,
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

mod game;
mod objects;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?)
                .with_plugin(RenderShaded3D::default())
                .with_plugin(RenderSkybox::with_colors(
                    Srgb::new(0.82, 0.51, 0.50),
                    Srgb::new(0.82, 0.51, 0.50),
                )),
        )?;

    let mut game = Application::new(assets_dir, game::Game, game_data)?;
    game.run();
    Ok(())
}
