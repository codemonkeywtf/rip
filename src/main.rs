use geng::prelude::*;

#[derive(geng::asset::Load)]
struct Assets {
    #[load(postprocess =  "pixelate")]
    tombstone: ugli::Texture,
}

fn pixelate(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}

struct Tombstone {
    position: vec2<f32>,
}

struct Game {
    geng: Geng,
    camera: geng::Camera2d,
    tombstones: [Tombstone; 4],
    assets: Assets,
    score: u32,
}

impl Game {
    fn new(geng: &Geng, assets: Assets) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: vec2(0.0, 0.0),
                rotation: 0.0,
                fov: 15.0,
            },
            assets,

            // spawn tombstones
            tombstones: {
                let tombstone_0 = Tombstone{position: vec2(-5.0,-4.5)};
                let tombstone_1 = Tombstone{position: vec2(-1.75,-4.5)};
                let tombstone_2 = Tombstone{position: vec2(1.75,-4.5)};
                let tombstone_3 = Tombstone{position: vec2(5.0,-4.5)};
                [tombstone_0, tombstone_1, tombstone_2, tombstone_3]
            },
           score: 0,
        }
    }
}


impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera = geng::Camera2d {
            center: vec2::ZERO,
            rotation: 0.,
            fov: 15.0,
        };
//        let tombstone = mat3::translate(vec2(-5.0,-4.5));
        for tombstone in &self.tombstones {
            let tombstone_position = mat3::translate(tombstone.position);
            let scale = mat3::scale_uniform(0.90);
            let assets = &self.assets;//.get();
            for (texture, matrix) in [
                (&assets.tombstone, tombstone_position),
            ] {
                self.geng.draw2d().draw2d(
                    framebuffer,
                    &camera,
                    &draw2d::TexturedQuad::unit(texture).transform(scale*matrix),
                    );
            }
        }

        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            &("SCORE: ".to_owned() + &self.score.to_string()),
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(vec2((800/2) as f32, 770.0)) * mat3::scale_uniform(32.0),
            Rgba::WHITE,
            );
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "RIP".to_owned(),
        window_size: Some(vec2(800,800)),
        ..default()
    });
    geng.clone().run_loading(async move {
        let assets = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        Game::new(&geng, assets)
    });
}
