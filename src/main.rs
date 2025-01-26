mod resources;
mod player;

use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use macroquad_platformer::*;

#[macroquad::main("Dave")]
async fn main() {
    let resources = resources::Resources::load().await;

    let mut static_colliders = vec![];
    for (_x, _y, tile) in resources.tiled_map.tiles("Tile Layer 1", None) {
        static_colliders.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let mut world = World::new();
    world.add_static_tiled_layer(static_colliders, 32., 32., 19, 1);

    let mut player = player::Player::new(world.add_actor(vec2(60.0, 250.0), 32, 32));

    let mut animated_player = AnimatedSprite::new(
        32,
        32,
        &[
            Animation {
                name: "walk".to_string(),
                row: 0,
                frames: 2,
                fps: 4,
            },
            Animation {
                name: "idle".to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            },
            Animation {
                name: "jump".to_string(),
                row: 0,
                frames: 1,
                fps: 1,
            },
        ],
        true,
    );

    let camera = Camera2D::from_display_rect(Rect::new(0.0, 320.0, 608.0, -320.0));

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        resources.tiled_map.draw_tiles("Tile Layer 1", Rect::new(0.0, 0.0, 608.0, 320.0), None);

        let pos = world.actor_pos(player.collider);

        let on_ground = world.collide_check(player.collider, pos + vec2(0., 1.));

        // Draw player
        let state: &str;
        let flip: f32;

        if player.speed.x != 0.0 {
            state = if !on_ground {
                animated_player.set_animation(2); // jump
                "dave_jump"
            } else {
                animated_player.set_animation(0); // walk
                "dave_walk"
            };
        
            if player.speed.x < 0.0 {
                player.facing_left = true;
                flip = -32.0;
            } else {
                player.facing_left = false;
                flip = 32.0;
            }
        } else {
            state = "dave_idle";
            animated_player.set_animation(1); // idle
            flip = if player.facing_left { -32.0 } else { 32.0 };
        }

        resources.tiled_map.spr_ex(
            state,
            animated_player.frame().source_rect,
            Rect::new(
                pos.x + if flip < 0.0 { 32.0 } else { 0.0 },
                pos.y,
                flip,
                32.0,
            ),
        );

        animated_player.update();

        // player movement control
        if !on_ground {
            player.speed.y += 500. * get_frame_time();
        }

        if is_key_down(KeyCode::Right) {
            player.speed.x = 100.0;
        } else if is_key_down(KeyCode::Left) {
            player.speed.x = -100.0;
        } else {
            player.speed.x = 0.;
        }

        if is_key_pressed(KeyCode::Space) && on_ground {
            player.speed.y = -260.;
        }

        world.move_h(player.collider, player.speed.x * get_frame_time());
        world.move_v(player.collider, player.speed.y * get_frame_time());

        next_frame().await
    }
}
