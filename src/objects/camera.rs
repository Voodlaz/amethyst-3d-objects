use amethyst::prelude::*;
use amethyst::renderer::Camera;
use amethyst::window::ScreenDimensions;
use amethyst::core::Transform;


pub fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, -20.0, 10.0);
    transform.prepend_rotation_x_axis(1.325_752_1);

    let (width, height) = {
    let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world.create_entity()
        .with(Camera::standard_3d(width, height))
        .with(transform)
        .build();
}
