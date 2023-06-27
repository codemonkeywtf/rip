use geng::prelude::*;

const CONTROLS_LEFT: [geng::Key; 2] = [geng::Key::A, geng::Key::Left];
const CONTROLS_RIGHT: [geng::Key; 2] = [geng::Key::D, geng::Key::Right];
const CONTROLS_FIRE: [geng::Key; 3] = [geng::Key::W, geng::Key::Up, geng::Key::Space];
const ANIMATION_TIME: f32 = 0.25;
const BOLT_SPEED: f32 = 400.0;
const MARCH_SPEED: f32 = 60.0;

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

impl Tombstone {
    fn new() -> Self {
        Self {
            position: [
                vec2(160.0, 150.0),
                vec2(310.0, 150.0),
                vec2(470.0, 150.0),
                vec2(620.0, 150.0),
            ],
        }
    }
}

struct Player {
    position: vec2<f32>,
    has_bolt: bool,
    sprite: [vec2<f32>; 3],
    bolt_flight_pos: vec2<f32>,
    lives: usize,
}

impl Player {
    fn new() -> Self {
        Self {
            // spawn player
            position: vec2(130.0, 50.0),
            has_bolt: true,
            lives: 3,
            sprite: [
                vec2(0.0, 0.0), // bolt
                vec2(0.0, 0.5), // !loaded
                vec2(0.5, 0.5), // loaded
            ],
            bolt_flight_pos: vec2::ZERO,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Skeleton {
    size: vec2<f32>,
    position: vec2<f32>,
    dead: bool,
    can_throw: bool,
    frame: [f32; 4],
}

impl Skeleton {
    fn init() -> Vec<Skeleton> {
        let mut skeletons: Vec<Skeleton> = Vec::new();

        for x in (125..675).step_by(50) {
            for y in (320..770).step_by(90) {
                skeletons.push(Skeleton {
                    size: vec2(40.0, 40.0),
                    position: vec2(x as f32, y as f32),
                    dead: false,
                    can_throw: false,
                    frame: [0.0, 0.25, 0.50, 0.75],
                })
            }
        }
        skeletons
    }
}

struct Game {
    geng: Geng,
    camera: geng::Camera2d, //PixelPerfectCamera,
    tombstones: Tombstone,
    assets: Assets,
    player: Player,
    skeletons: Vec<Skeleton>,
    skeleton_cell: usize,
    animation_time: f32,
    score: usize,
    dx: f32,
    dy: f32,
    fade_in_out: f32,
    fx: f32,
    game_over: bool,
}

impl Game {
    fn new(geng: &Geng, assets: Assets) -> Self {
        let size = vec2(800.0, 800.0);
        Self {
            geng: geng.clone(),
            camera: geng::Camera2d {
                center: size / 2.0,
                fov: size.y,
                rotation: Angle::ZERO,
            },
            assets,
            animation_time: ANIMATION_TIME,
            score: 420,

            // spawn assets
            skeletons: Skeleton::init(),
            skeleton_cell: 0,
            dx: 10.0,
            dy: 10.0,
            fade_in_out: 1.0,
            fx: 0.2,
            player: Player::new(),
            tombstones: Tombstone::new(),
            game_over: false,
        }
    }

    fn draw_score(&mut self, framebuffer: &mut ugli::Framebuffer, position: vec2<f32>) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Text::unit(
                &self.assets.font,
                "SCORE: ".to_string(),
                Rgba {
                    r: 0.8,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            )
            .scale_uniform(18.0)
            .translate(position),
        );
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Text::unit(&self.assets.font, &(self.score.to_string()), Rgba::RED)
                .scale_uniform(14.0)
                .translate(vec2(position.x + 100.0, position.y)),
        )
    }

    fn draw_game_over(&mut self, framebuffer: &mut ugli::Framebuffer, fade_in_out: f32) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::Text::unit(
                &self.assets.font,
                "G A M E  O V E R".to_string(),
                Rgba {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: fade_in_out,
                },
            )
            .scale_uniform(32.0)
            .translate(vec2(400.0, 400.0)),
        )
    }

    fn draw_skeletons(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        mut skeletons: Vec<Skeleton>,
        cell: usize,
    ) {
        for skeleton in &mut skeletons {
            self.geng.draw2d().draw2d(
                framebuffer,
                &self.camera,
                &draw2d::TexturedQuad::unit(&self.assets.skeleton)
                    .scale(skeleton.size)
                    .translate(skeleton.position)
                    .sub_texture(
                        Aabb2::point(vec2(skeleton.frame[cell], 0.0))
                            .extend_positive(vec2(0.25, 1.0)),
                    ),
            )
        }
    }

    fn update_skeletons(
        &mut self,
        skeletons: Vec<Skeleton>,
        skeleton_x_start: f32,
        skeleton_y_start: f32,
    ) {
        let start_x = skeleton_x_start as usize;
        let start_y = skeleton_y_start as usize;
    }

    fn draw_tombstones(&mut self, framebuffer: &mut ugli::Framebuffer, positions: [vec2<f32>; 4]) {
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

    fn draw_player(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        position: vec2<f32>,
        sprite: vec2<f32>,
    ) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
                .scale_uniform(38.0)
                .translate(position)
                .sub_texture(Aabb2::point(sprite).extend_positive(vec2::splat(0.5))),
        );
    }

    fn draw_bolt(
        &mut self,
        framebuffer: &mut ugli::Framebuffer,
        position: vec2<f32>,
        sprite: vec2<f32>,
    ) {
        self.geng.draw2d().draw2d(
            framebuffer,
            &self.camera,
            &draw2d::TexturedQuad::unit(&self.assets.crossbow)
                .scale_uniform(38.0)
                .translate(vec2(position.x, position.y + 25.0))
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
        ) {
            if self.player.has_bolt {
                self.player.has_bolt = false;
                self.player.bolt_flight_pos = self.player.position;
            }
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;

        if self.game_over {
            if self.fade_in_out <= 0.3 {
                self.fx = 0.4;
            }
            if self.fade_in_out >= 1.0 {
                self.fx = -0.4;
            }
            self.fade_in_out += delta_time * self.fx;
        }

        // player move
        if self.geng.window().is_key_pressed(geng::Key::A) {
            self.player.position.x -= delta_time * 200.0;
            if self.player.position.x <= 35.0 {
                self.player.position.x = 35.0
            }
        }

        if self.geng.window().is_key_pressed(geng::Key::D) {
            self.player.position.x += delta_time * 200.0;
            if self.player.position.x >= 765.0 {
                self.player.position.x = 765.0
            }
        }

        // enemy animation
        self.animation_time -= delta_time;
        if self.animation_time < 0.0 {
            self.skeleton_cell += 1;
            self.skeleton_cell = self.skeleton_cell % 4;
            self.animation_time = ANIMATION_TIME;
        }

        // enemy march
        for skeleton in &mut self.skeletons {
            if skeleton.position.x < 30.0 {
                self.score = 694200000;
                self.dx = -10.0;
                self.dy = 10.0;
                break;
            } else {
                self.dy = 0.0;
            }
            if skeleton.position.x > 780.0 {
                self.score = 69000;
                self.dx = 10.0;
                self.dy = 10.0;
                break;
            } else {
                self.dy = 0.0;
            }
        }

        for skeleton in &mut self.skeletons {
            if skeleton.position.y <= self.player.position.y + 70.0
                && skeleton.position.x as usize == self.player.position.x as usize
            {
                self.dx = 10.0;
                self.dy = 0.0;
                self.skeletons = Skeleton::init();
                break;
            } else if skeleton.position.y < self.player.position.y {
                self.dx = 10.0;
                self.dy = 0.0;
                self.skeletons = Skeleton::init();
                break;
            }
        }

        let d_time = delta_time * 4.0;
        for skeleton in &mut self.skeletons {
            skeleton.position.y -= self.dy;
            skeleton.position.x -= self.dx * d_time;
            skeleton.position = vec2(skeleton.position.x, skeleton.position.y);
        }

        if !self.player.has_bolt {
            self.player.bolt_flight_pos.y += BOLT_SPEED * delta_time;
        }

        if self.player.bolt_flight_pos.y > 810.0 {
            self.player.has_bolt = true;
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

        let player: vec2<f32>;
        if self.player.has_bolt {
            player = self.player.sprite[2];
            self.player.bolt_flight_pos = self.player.position;
        } else {
            player = self.player.sprite[1];
        }

        self.draw_score(framebuffer, vec2(400.0, 770.0));
        self.draw_tombstones(framebuffer, self.tombstones.position);
        self.draw_skeletons(framebuffer, self.skeletons.clone(), self.skeleton_cell);
        self.draw_player(framebuffer, self.player.position, player);
        self.draw_bolt(
            framebuffer,
            self.player.bolt_flight_pos,
            self.player.sprite[0],
        );
        if self.game_over {
            self.draw_game_over(framebuffer, self.fade_in_out);
        }
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();
    let geng = Geng::new_with(geng::ContextOptions {
        //title: "RIP".to_owned(),
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
