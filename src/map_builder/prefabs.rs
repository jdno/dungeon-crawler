use bracket_lib::prelude::*;

use crate::map::{coordinate_to_index, TileType};
use crate::{MapBuilder, RandomNumberGenerator, MAP_HEIGHT, MAP_WIDTH};

const FORTRESS: (&str, i32, i32) = (
    r#"
        ------------
        ---######---
        ---#----#---
        ---#-M--#---
        -###----###-
        --M------M--
        -###----###-
        ---#----#---
        ---#----#---
        ---######---
        ------------
    "#,
    12,
    11,
);

pub fn apply_prefab(builder: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &[builder.map.point2d_to_index(builder.player_start)],
        &builder.map,
        1024.0,
    );

    for _ in 0..10 {
        let dimensions = Rect::with_size(
            rng.range(0, MAP_WIDTH - FORTRESS.1),
            rng.range(0, MAP_HEIGHT - FORTRESS.2),
            FORTRESS.1,
            FORTRESS.2,
        );

        if dimensions.point_set().iter().all(|point| {
            let index = builder.map.point2d_to_index(*point);
            let distance = dijkstra_map.map[index];

            distance < 2000.0 && distance > 20.0 && builder.amulet_position != *point
        }) {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            builder
                .monster_spawns
                .retain(|point| !dimensions.point_set().contains(point));
            break;
        }
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = FORTRESS
            .0
            .chars()
            .filter(|c| *c != '\r' || *c != '\n')
            .collect();
        let mut i = 0;

        for ty in placement.y..placement.y + FORTRESS.2 {
            for tx in placement.x..placement.x + FORTRESS.1 {
                let index = coordinate_to_index(tx, ty);

                match string_vec[i] {
                    'M' => {
                        builder.map.tiles[index] = TileType::Floor;
                        builder.monster_spawns.push(Point::new(tx, ty));
                    }
                    '-' => builder.map.tiles[index] = TileType::Floor,
                    '#' => builder.map.tiles[index] = TileType::Wall,
                    _ => println!("unexpected character in prefab"),
                }

                i += 1;
            }
        }
    }
}
