extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();

    let sprite_image = assets.join("red_64_64.png");
    let sprite_image: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &sprite_image,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&sprite_image, c.transform, g);
        });
    }
}
