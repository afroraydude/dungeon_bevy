use bevy::prelude::*;

/*
Resources
dungeon generation
random dungeon generation
procedural dungeon generation
dungeon generation algorithm

https://journal.stuffwithstuff.com/2014/12/21/rooms-and-mazes/
https://www.gamedeveloper.com/programming/procedural-dungeon-generation-algorithm
https://www.reddit.com/r/gamedev/comments/1dlwc4/procedural_dungeon_generation_algorithm_explained/
https://gamedev.stackexchange.com/questions/2663/what-are-some-ideal-algorithms-for-rogue-like-2d-dungeon-generation
http://pcg.wikidot.com/pcg-algorithm:dungeon-generation
https://gamedev.stackexchange.com/questions/82059/algorithm-for-procedural-2d-map-with-connected-paths
https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
*/

pub fn gen_dungeon(mut commands: Commands) {
    let min_room_size = UVec2::new(4, 4);
    let max_room_size = UVec2::new(16, 16);

    /*
    We start with a rectangular dungeon filled with wall cells. We are going to split this dungeon recursively until each 
    sub-dungeon has approximately the size of a room. The dungeon splitting uses this operation:

    Choose a random direction : horizontal or vertical splitting
    Choose a random position (x for vertical, y for horizontal)
    Split the dungeon into two sub-dungeons
    When choosing the splitting position, we have to take care not to be too close to the dungeon border.
    We must be able to place a room inside each generated sub-dungeon. We repeat until the lowest sub-dungeons
    have approximately the size of the rooms we want to generate.

    This is called a binary space partitioning algorithm. It is a very common algorithm used in many games to generate levels.
    */
}
