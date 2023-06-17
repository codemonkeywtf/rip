use geng::prelude::*;

const CONTROLS_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];
const CONTROLS_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const CONTROLS_FIRE: [geng::Key; 3] = [geng::Key::W, geng::Key::Up, geng::Key::Space];
const ANIMATION_TIME: f32 =  0.3;
const BOLT_SPEED: f32 = 400.0;

enum PlayerSprite {
    Bolt(vec2<f32>),
    Loaded(vec2<f32>),
    Empty(vec2<f32>),
}

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
    position: [vec2<f32>; 4],
}

struct Player {
    position: vec2<f32>,
    has_bolt: bool,
    sprite: [vec2<f32>; 3],
    bolt_flight_pos: vec2<f32>,
    lives: usize,
}

struct Game {
    geng: Geng,
    camera: geng::PixelPerfectCamera,
    tombstones: Tombstone,
    assets: Assets,
    player: Player,
    enemy_sprite: f32,
    animation_time: f32,
    speed: f32,
    score: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            // spawn player
                position: vec2(130.0, 80.0),
                has_bolt: true,
                lives: 3,
                sprite: [
                    vec2(0.0, 0.0),     // bolt
                    vec2(0.0,0.5),  // !loaded
                    vec2(0.5, 0.5), // loaded
                ],
                bolt_flight_pos: vec2::ZERO,
        }
    }
}

impl Tombstone {
    fn new() -> Self {
        Self {
            position: [
                vec2(160.0, 180.0),
                vec2(310.0, 180.0),
                vec2(470.0, 180.0),
                vec2(620.0, 180.0),
            ]
        }
    }
}

impl Game {
    fn new(geng: &Geng, assets: Assets) -> Self {
        Self {
            geng: geng.clone(),
            camera: geng::PixelPerfectCamera,
            assets,
            enemy_sprite: 0.0,
            animation_time: 0.3,
            speed: 5.,
            score: 420.0,

            // spawn assets
            player: Player::new(),
            tombstones: Tombstone::new(),
        }
    }

    fn draw_score(&mut self, framebuffer: &mut ugli::Framebuffer, position: vec2<f32>, score: f32) {
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

    fn draw_tombstones(&mut self, framebuffer: &mut ugli::Framebuffer, positions: [vec2<f32>; 4]){
        for position in positions {
            self.geng.draw2d().draw2d(
                framebuffer,
                &self.camera,
                &draw2d::TexturedQuad::unit(&self.assets.tombstone)
                .scale_uniform(48.0)
                .translate(position),
                )
        }
    }

    fn draw_player(&mut self, framebuffer: &mut ugli::Framebuffer,  position: vec2<f32>, sprite: vec2<f32>){
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
            .scale_uniform(38.0)
            .translate(position)
            .sub_texture(Aabb2::point(sprite).extend_positive(vec2::splat(0.5))),
            );
    }

    fn draw_bolt(&mut self, framebuffer: &mut ugli::Framebuffer, position: vec2<f32>, sprite: vec2<f32>){
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
            .scale_uniform(38.0)
            .translate(vec2(position.x, position.y+25.0))
            .sub_texture(Aabb2::point(sprite).extend_positive(vec2::splat(0.5))),
            );
    }
}

impl geng::State for Game {

    fn handle_event(&mut self, event: geng::Event) {
        if matches!(
            event,
            geng::Event::KeyDown {
                key: geng::Key::Space
            }
        ){
            if self.player.has_bolt {
                self.player.has_bolt = false;
                self.player.bolt_flight_pos = self.player.position;
            }
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
         // player move
        if self.geng.window().is_key_pressed(geng::Key::A) {
            self.player.position.x -= delta_time*200.0;
            if self.player.position.x <= 35.0 {self.player.position.x = 35.0}
        }

        if self.geng.window().is_key_pressed(geng::Key::D) {
            self.player.position.x += delta_time*200.0;
            if self.player.position.x >= 765.0 {self.player.position.x = 765.0}
        }

        // enemy animation
        self.animation_time -= delta_time;
        if self.animation_time < 0.0 {
            self.enemy_sprite += 0.25;
            if self.enemy_sprite == 1.0 {self.enemy_sprite = 0.0}
            self.animation_time = ANIMATION_TIME;
        }

        if !self.player.has_bolt {
            self.player.bolt_flight_pos.y += BOLT_SPEED *delta_time;
        }

        if self.player.bolt_flight_pos.y > 810.0 {
            self.player.has_bolt = true;
        }

        self.speed -= delta_time;
        if self.speed < 0.0 {
            self.speed = 5.;
            self.score += 20.;
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


        // self.geng.draw2d().draw2d(
        //     framebuffer,
        //     &self.camera,
        //     &draw2d::TexturedQuad::unit(&self.assets.crossbow)
        //         .scale_uniform(48.0)
        //         .translate(vec2(130.0, 80.0))
        //         .sub_texture(Aabb2::point(vec2(0., 0.5)).extend_positive(vec2::splat(0.5))),
        // );


        let player: vec2<f32>;
        if self.player.has_bolt {
            player = self.player.sprite[2];
            self.player.bolt_flight_pos = self.player.position;
        } else {
            player = self.player.sprite[1];
        }

        self.draw_player(framebuffer, self.player.position, player);
        self.draw_bolt(framebuffer, self.player.bolt_flight_pos, self.player.sprite[0]);

        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.skeleton)
                .scale_uniform(38.0)
                .translate(vec2(400.0, 400.0))
                .sub_texture(Aabb2::point(vec2(self.enemy_sprite,0.0)).extend_positive(vec2(0.25,1.0))),
        );

        self.draw_score(framebuffer, vec2(400.0, 770.0), self.score);
        self.draw_tombstones(framebuffer, self.tombstones.position);
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

    geng.clone().run_loading(async move {
        let assets = geng
            .asset_manager()
            .load(run_dir().join("assets"))
            .await
            .expect("Failed to load assets");
        Game::new(&geng, assets)
    });
}
