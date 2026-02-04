use bevy::prelude::*;
use packets::client::Pickup;

use crate::{
    ecs::components::{Direction, LocalPlayer, Position, ItemSprite, EntityId},
    events::PlayerAction,
    network::PacketOutbox,
};

/// Handle PickupItemBelow action: search for items under then in front, send pickup for the first one found.
pub fn handle_pickup_item_below(
    mut player_actions: MessageReader<PlayerAction>,
    player_query: Query<(&Position, &Direction), With<LocalPlayer>>,
    item_query: Query<(&Position, &ItemSprite, &EntityId), With<ItemSprite>>,
    outbox: Res<PacketOutbox>,
) {
    for action in player_actions.read() {
        if matches!(action, PlayerAction::PickupItemBelow) {
            // Get player position and direction
            if let Ok((player_pos, direction)) = player_query.single() {
                let under_x = player_pos.x as u16;
                let under_y = player_pos.y as u16;

                // Compute front tile
                let (dx, dy) = match direction {
                    Direction::Up => (0.0, -1.0),
                    Direction::Down => (0.0, 1.0),
                    Direction::Left => (-1.0, 0.0),
                    Direction::Right => (1.0, 0.0),
                };
                let front_x = (player_pos.x + dx) as u16;
                let front_y = (player_pos.y + dy) as u16;

                // Search positions in priority order: under, then front. Stop after the first pickup.
                for search_pos in [(under_x, under_y), (front_x, front_y)] {
                    for (ipos, _isprite, _eid) in item_query.iter() {
                        if ipos.x as u16 == search_pos.0 && ipos.y as u16 == search_pos.1 {
                            tracing::info!(
                                "Item pickup triggered at ({}, {})",
                                search_pos.0,
                                search_pos.1
                            );
                            outbox.send(&Pickup {
                                destination_slot: 0,
                                source_point: search_pos,
                            });
                            return; // handled one item; stop
                        }
                    }
                }

                tracing::trace!(
                    "Item pickup: no items found under ({}, {}) or in front ({}, {})",
                    under_x,
                    under_y,
                    front_x,
                    front_y
                );
            }
        }
    }
}
