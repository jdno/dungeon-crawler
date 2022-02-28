use bracket_lib::prelude::*;
use legion::World;

use crate::components::{ChasesPlayer, Enemy, Health, Name, Player, Render};

type Monster = (i32, String, FontCharType);

fn goblin() -> Monster {
    (1, "Goblin".to_string(), to_cp437('g'))
}

fn orc() -> Monster {
    (2, "Orc".to_string(), to_cp437('o'))
}

pub fn spawn_monster(ecs: &mut World, rng: &mut RandomNumberGenerator, position: Point) {
    let (hp, name, glyph) = match rng.roll_dice(1, 10) {
        1..=8 => goblin(),
        _ => orc(),
    };

    ecs.push((
        Enemy,
        ChasesPlayer,
        Health {
            current: hp,
            max: hp,
        },
        Name(name),
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph,
        },
    ));
}

pub fn spawn_player(ecs: &mut World, position: Point) {
    ecs.push((
        Player,
        Health {
            current: 20,
            max: 20,
        },
        position,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
