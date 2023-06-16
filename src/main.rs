use geng::prelude::{*, ron::de};

const CONTROLS_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];
const CONTROLS_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const CONTROLS_FIRE: [geng::Key; 3] = [geng::Key::W, geng::Key::Up, geng::Key::Space];
const ANIMATION_TIME: f32 =  0.2;

#[derive(geng::asset::Load)]
struct Assets {
    #[load(postprocess = "pixelate")]
    tombstone: ugli::Texture,
    #[load(postprocess = "pixelate")]
    crossbow: ugli::Texture,
    #[load(postprocess = "pixelate")]
    skeleton: ugli::Texture,
    #[load(path = "font/Lacquer-Regular.ttf")]
    font: geng::Font,
}

fn pixelate(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}

struct Tombstone {
    position: vec2<f32>,
}

struct Game {
    geng: Geng,
    camera: geng::PixelPerfectCamera,
    tombstones: [Tombstone; 4],
    assets: Assets,
    player_pos: vec2<f32>,
    enemy_sprite: f32,
    timer: Timer,
    animation_time: f32,
}

impl Game {
    fn new(geng: &Geng, assets: Assets) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::PixelPerfectCamera,
            assets,
            player_pos: vec2(130.0,80.0),
            enemy_sprite: 0.0,
            animation_time: 0.3,
            timer: Timer::new(),

            // spawn tombstones
            tombstones: {
                let tombstone_0 = Tombstone {
                    position: vec2(160.0, 180.0),
                };
                let tombstone_1 = Tombstone {
                    position: vec2(310.0, 180.0),
                };
                let tombstone_2 = Tombstone {
                    position: vec2(470.0, 180.0),
                };
                let tombstone_3 = Tombstone {
                    position: vec2(620.0, 180.0),
                };
                [tombstone_0, tombstone_1, tombstone_2, tombstone_3]
            },
        }
    }

    fn draw_score(&mut self, framebuffer: &mut ugli::Framebuffer, position: vec2<f32>, score: u32) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Text::unit(
                &self.assets.font,
                &("SCORE: ".to_owned() + &(score.to_string()).to_owned()),
                Rgba::RED,
            )
            .scale_uniform(18.0)
            .translate(position),
        );
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.animation_time -= delta_time;
         // player move
        if self.geng.window().is_key_pressed(geng::Key::A) {
            self.player_pos.x -= delta_time*200.0;
            if self.player_pos.x <= 35.0 {self.player_pos.x = 35.0}
        }

        if self.geng.window().is_key_pressed(geng::Key::D) {
            self.player_pos.x += delta_time*200.0;
            if self.player_pos.x >= 765.0 {self.player_pos.x = 765.0}
        }

        if self.animation_time < 0.0 {
            self.enemy_sprite += 0.25;
            if self.enemy_sprite == 1.0 {self.enemy_sprite = 0.0}
            self.animation_time = ANIMATION_TIME;
        }

    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(
            framebuffer,
            Some(Rgba {
                r: 0.,
                g: 0.15,
                b: 0.,
                a: 0.,
            }),
            None,
            None,
        );



        for tombstone in &self.tombstones {
            self.geng.draw2d().draw2d(
                framebuffer,
                &self.camera,
                &draw2d::TexturedQuad::unit(&self.assets.tombstone)
                    .scale_uniform(48.0)
                    .translate(tombstone.position),
            )
        }

        // self.geng.draw2d().draw2d(
        //     framebuffer,
        //     &self.camera,
        //     &draw2d::TexturedQuad::unit(&self.assets.crossbow)
        //         .scale_uniform(48.0)
        //         .translate(vec2(130.0, 80.0))
        //         .sub_texture(Aabb2::point(vec2(0., 0.5)).extend_positive(vec2::splat(0.5))),
        // );

        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
                .scale_uniform(38.0)
                .translate(self.player_pos)
                .sub_texture(Aabb2::point(vec2(0.5, 0.5)).extend_positive(vec2::splat(0.5))),
        );

        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
                .scale_uniform(38.0)
                .translate(vec2(self.player_pos.x, self.player_pos.y+25.0))
                .sub_texture(Aabb2::ZERO.extend_positive(vec2::splat(0.5))),
        );

        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.skeleton)
                .scale_uniform(38.0)
                .translate(vec2(400.0, 400.0))
                .sub_texture(Aabb2::point(vec2(self.enemy_sprite,0.0)).extend_positive(vec2(0.25,1.0))),
        );

        let score = 420;
        let position = vec2(400.0, 770.0);
        self.draw_score(framebuffer, position, score)
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        title: "RIP".to_owned(),
        window_size: Some(vec2(800, 800)),
        ..default()
    });
    for x in (0..100).step_by(25){
        println!("{}", x as f32/100.);
    }
    geng.clone().run_loading(async move {
        let assets = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        Game::new(&geng, assets)
    });
}
