use std::{io::Write, time::Instant};

use crate::components::LoadingText;
use crate::resources::assets::MyAssets;
use crate::systems::print_pc_data_to_debug;
use bevy::prelude::*;
use bevy::utils::tracing::field::debug;

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

const MIN_LEAF_SIZE: u32 = 16;
const MAX_LEAF_SIZE: u32 = 64;

const MIN_ROOM_SIZE: u32 = MIN_LEAF_SIZE - 2;

#[derive(Debug, Clone)]
pub struct Room {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Clone)]
pub struct Leaf {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub left_child: Option<Box<Leaf>>,
    pub right_child: Option<Box<Leaf>>,
    pub room: Option<Room>,
    pub halls: Vec<Room>,
}

pub struct Dungeon {
    pub base_map: Vec<Vec<char>>,
    width: u32,
    height: u32,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            base_map: Vec::new(),
            width: 128,
            height: 128,
        }
    }
}

impl Leaf {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None,
            halls: Vec::new(),
        }
    }

    pub fn split(&mut self) -> bool {
        if !self.left_child.is_none() || !self.right_child.is_none() {
            return false;
        }

        /*
        Determine direction of split
        if the width is > 25% larger than height, we split vertically
        if the height is > 25% larger than the width, we split horizontally
        otherwise we split randomly
        */
        let split_horizontal: bool;
        if self.width > self.height && (self.width / self.height) as f32 >= 1.25 {
            // split vertically
            split_horizontal = false;
        } else if self.height > self.width && (self.height / self.width) as f32 >= 1.25 {
            // split horizontally
            split_horizontal = true;
        } else {
            // split randomly
            split_horizontal = rand::random::<bool>();
        }

        let max = if split_horizontal {
            self.height - MIN_LEAF_SIZE
        } else {
            self.width - MIN_LEAF_SIZE
        };

        if max <= MIN_LEAF_SIZE {
            return false;
        }

        let split = rand::random::<u32>() % (max - MIN_LEAF_SIZE) + MIN_LEAF_SIZE;

        if split_horizontal {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split)));
            self.right_child = Some(Box::new(Leaf::new(
                self.x,
                self.y + split,
                self.width,
                self.height - split,
            )));
        } else {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split, self.height)));
            self.right_child = Some(Box::new(Leaf::new(
                self.x + split,
                self.y,
                self.width - split,
                self.height,
            )));
        }

        true
    }

    pub fn create_rooms(&mut self) {
        // there is already a room here
        if self.room.is_some() {
            return;
        }

        if !self.left_child.is_none() || !self.right_child.is_none() {
            if !self.left_child.is_none() {
                self.left_child.as_mut().unwrap().create_rooms();
            }
            if !self.right_child.is_none() {
                self.right_child.as_mut().unwrap().create_rooms();
            }
            if !self.left_child.is_none() && !self.right_child.is_none() {
                self.create_halls();

                // make sure the halls don't exceed the bounds of the leaf
                for hall in self.halls.iter_mut() {
                    if hall.x < self.x {
                        hall.w -= self.x - hall.x;
                        hall.x = self.x;
                    }
                    if hall.y < self.y {
                        hall.h -= self.y - hall.y;
                        hall.y = self.y;
                    }
                    if hall.x + hall.w > self.x + self.width {
                        hall.w = self.x + self.width - hall.x;
                    }
                    if hall.y + hall.h > self.y + self.height {
                        hall.h = self.y + self.height - hall.y;
                    }
                }
            }
        } else {
            let room_width = rand::random::<u32>() % (self.width - MIN_ROOM_SIZE) + MIN_ROOM_SIZE;
            let room_height = rand::random::<u32>() % (self.height - MIN_ROOM_SIZE) + MIN_ROOM_SIZE;

            let room_x = rand::random::<u32>() % (self.width - room_width);
            let room_y = rand::random::<u32>() % (self.height - room_height);

            self.room = Some(Room {
                x: self.x + room_x,
                y: self.y + room_y,
                w: room_width,
                h: room_height,
            });
        }
    }

    #[allow(dead_code)]
    pub fn get_room(&self) -> Option<Room> {
        if !self.room.is_none() {
            return Some(self.room.clone().unwrap());
        }

        let l_room: Option<Room>;
        let r_room: Option<Room>;

        if !self.left_child.is_none() {
            l_room = self.left_child.as_ref().unwrap().get_room();
        } else {
            l_room = None;
        }

        if !self.right_child.is_none() {
            r_room = self.right_child.as_ref().unwrap().get_room();
        } else {
            r_room = None;
        }

        if l_room.is_none() && r_room.is_none() {
            return None;
        } else if l_room.is_none() {
            return Some(r_room.unwrap());
        } else if r_room.is_none() {
            return Some(l_room.unwrap());
        } else {
            if rand::random::<bool>() {
                return Some(l_room.unwrap());
            } else {
                return Some(r_room.unwrap());
            }
        }
    }

    pub fn create_halls(&mut self) {
        // TODO: Change back to old algorithm

        // connect rooms by adding halls which are rooms with a width of 1
        let l_room = self.left_child.as_ref().unwrap().get_room().unwrap();
        let r_room = self.right_child.as_ref().unwrap().get_room().unwrap();

        let l_center_x = l_room.x + l_room.w / 2;
        let l_center_y = l_room.y + l_room.h / 2;
        let r_center_x = r_room.x + r_room.w / 2;
        let r_center_y = r_room.y + r_room.h / 2;

        if rand::random::<bool>() {
            // first move horizontally, then vertically
            self.halls.push(Room {
                x: l_center_x,
                y: l_center_y,
                w: (r_center_x as i32 - l_center_x as i32).abs() as u32,
                h: 1,
            });
            self.halls.push(Room {
                x: r_center_x,
                y: l_center_y,
                w: 1,
                h: (r_center_y as i32 - l_center_y as i32).abs() as u32,
            });
        } else {
            // first move vertically, then horizontally
            self.halls.push(Room {
                x: l_center_x,
                y: l_center_y,
                w: 1,
                h: (r_center_y as i32 - l_center_y as i32).abs() as u32,
            });
            self.halls.push(Room {
                x: l_center_x,
                y: r_center_y,
                w: (r_center_x as i32 - l_center_x as i32).abs() as u32,
                h: 1,
            });
        }
    }
}

fn draw_map(leafs: &Vec<Leaf>, width: u32, height: u32) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['#'; width as usize]; height as usize];

    let mut i = 0;
    for leaf in leafs {
        if leaf.room.is_some() {
            if leaf.room.is_none() {
                continue;
            }
            let room = leaf.room.as_ref().unwrap();
            // draw room with # as walls and . as floor
            for x in room.x..(room.x + room.w) {
                for y in room.y..room.y + room.h {
                    grid[y as usize][x as usize] = '.';
                }
            }
        }

        // draw halls with # as walls and . as floor
        for hall in &leaf.halls {
            for x in hall.x..hall.x + hall.w {
                for y in hall.y..hall.y + hall.h {
                    grid[y as usize][x as usize] = '.';
                }
            }
        }

        i += 1;
    }

    grid
}

fn format_map(dungeon: &mut Dungeon) -> Vec<Vec<char>> {
    let mut map = dungeon.base_map.clone();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            // change char of walls to depend on surrounding tiles (left, right, up, down)
            if map[y][x] == '#' {
                let mut wall_char = '#';
                // if not a corner block
                if x > 0 && x < map[y].len() - 1 && y > 0 && y < map.len() - 1 {
                    // if there is a wall to the left
                    if map[y][x - 1] != '.' {
                        // if there is also a wall to the right
                        if map[y][x + 1] != '.' {
                            // if there is also a wall above
                            if map[y - 1][x] != '.' {
                                // if there is also a wall below
                                if map[y + 1][x] != '.' {
                                    wall_char = '#';
                                } else {
                                    wall_char = '┴';
                                }
                            } else {
                                // if there is also a wall below
                                if map[y + 1][x] != '.' {
                                    wall_char = '┬';
                                } else {
                                    wall_char = '│';
                                }
                            }
                        } else {
                            // if there is also a wall above
                            if map[y - 1][x] != '.' {
                                // if there is also a wall below
                                if map[y + 1][x] != '.' {
                                    wall_char = '┤';
                                } else {
                                    wall_char = '┘';
                                }
                            } else {
                                // if there is also a wall below
                                if map[y + 1][x] != '.' {
                                    wall_char = '┐';
                                } else {
                                    wall_char = '└';
                                }
                            }
                        }
                        // else if there is a wall to the right
                    } else if map[y][x + 1] != '.' {
                        // if there is also a wall above
                        if map[y - 1][x] != '.' {
                            // if there is also a wall below
                            if map[y + 1][x] != '.' {
                                wall_char = '├';
                            } else {
                                wall_char = '┌';
                            }
                        } else {
                            // if there is also a wall below
                            if map[y + 1][x] != '.' {
                                wall_char = '└';
                            } else {
                                wall_char = '─';
                            }
                        }
                        // else if there is a wall above
                    } else if map[y - 1][x] != '.' {
                        // if there is also a wall below
                        if map[y + 1][x] != '.' {
                            wall_char = '│';
                        } else {
                            wall_char = '┴';
                        }
                        // else if there is a wall below
                    } else if map[y + 1][x] != '.' {
                        wall_char = '┬';
                    }
                } else {
                    // if top left
                    if x == 0 && y == 0 {
                        // if there is a wall to the right
                        if map[y][x + 1] != '.' {
                            // if there is also a wall below
                            if map[y + 1][x] != '.' {
                                wall_char = '┐';
                            } else {
                                wall_char = '└';
                            }
                            // else if there is a wall below
                        } else if map[y + 1][x] != '.' {
                            wall_char = '┘';
                        } else {
                            wall_char = '└';
                        }
                        // else if top right
                    }
                    if x == map[y].len() - 1 && y == 0 {
                        // if there is a wall to the left
                        if map[y][x - 1] != '.' {
                            // if there is also a wall below
                            if map[y + 1][x] != '.' {
                                wall_char = '┌';
                            } else {
                                wall_char = '┘';
                            }
                            // else if there is a wall below
                        } else if map[y + 1][x] != '.' {
                            wall_char = '└';
                        } else {
                            wall_char = '┘';
                        }
                        // else if bottom left
                    }
                    if x == 0 && y == map.len() - 1 {
                        // if there is a wall to the right
                        if map[y][x + 1] != '.' {
                            // if there is also a wall above
                            if map[y - 1][x] != '.' {
                                wall_char = '┘';
                            } else {
                                wall_char = '┌';
                            }
                            // else if there is a wall above
                        } else if map[y - 1][x] != '.' {
                            wall_char = '┐';
                        } else {
                            wall_char = '┌';
                        }
                        // else if bottom right
                    }
                    if x == map[y].len() - 1 && y == map.len() - 1 {
                        // if there is a wall to the left
                        if map[y][x - 1] != '.' {
                            // if there is also a wall above
                            if map[y - 1][x] != '.' {
                                wall_char = '└';
                            } else {
                                wall_char = '┐';
                            }
                            // else if there is a wall above
                        } else if map[y - 1][x] != '.' {
                            wall_char = '┌';
                        } else {
                            wall_char = '┐';
                        }
                    }
                }
                map[y][x] = wall_char;
            }
        }
    }

    map
}

fn gen_dungeon_stress_test_internal(width: u32, height: u32) {
    //let min_room_size = UVec2::new(4, 4);
    //let max_room_size = UVec2::new(16, 16);
    /*let mut file = std::fs::File::create(format!("generation.txt")).unwrap();*/

    let mut i = 0;
    let max = 10;

    let mut run_times: Vec<u128> = Vec::new();

    while i < max {
        let start_time = Instant::now();

        gen_dungeon_internal(width, height);

        let run_time = start_time.elapsed().as_millis();
        run_times.push(run_time.clone());

        i += 1;
    }

    run_times.sort();
    let total_time: u128 = run_times.iter().sum();
    let avg_time = total_time / max as u128;
    let median_time = run_times[run_times.len() / 2];
    let min_time = run_times[0];
    let max_time = run_times[run_times.len() - 1];

    info!(
        "Random dungeon generation stress test results for {}x{}:",
        width, height
    );
    debug!("Total time: {}ms", total_time);
    debug!("Average time: {}ms", avg_time);
    debug!("Median time: {}ms", median_time);
    debug!("Min time: {}ms", min_time);
    debug!("Max time: {}ms", max_time);
}

pub fn gen_dungeon_stress_test(mut commands: Commands, mut assets: ResMut<MyAssets>) {
    print_pc_data_to_debug();
    debug!("Starting stress test");

    // using 2^x values for width and height
    // start with 128x128
    gen_dungeon_stress_test_internal(128, 128);
    gen_dungeon_stress_test_internal(256, 256);
    gen_dungeon_stress_test_internal(512, 512);
    gen_dungeon_stress_test_internal(1024, 1024);
    gen_dungeon_stress_test_internal(2048, 2048);
    gen_dungeon_stress_test_internal(4096, 4096);

    // exit the program
    std::process::exit(0);
}

fn print_leaf_data(leafs: &Vec<Leaf>) {
    let mut file = std::fs::File::create(format!("leaf_data.txt")).unwrap();
    let mut i = 0;
    for leaf in leafs {
        write!(file, "Leaf {}\r", i).unwrap();
        write!(file, "{:?}\r", leaf).unwrap();
        i += 1;
    }
}

fn gen_dungeon_internal(width: u32, height: u32) -> Vec<Vec<char>> {
    let mut leafs: Vec<Leaf> = Vec::new();

    let root = Leaf::new(0, 0, width, height);

    leafs.push(root.clone());

    let mut did_split = true;

    while did_split {
        did_split = false;

        let mut new_leafs: Vec<Leaf> = Vec::new();

        for leaf in leafs.iter_mut() {
            if leaf.left_child.is_none() && leaf.right_child.is_none() {
                if leaf.width > MAX_LEAF_SIZE
                    || leaf.height > MAX_LEAF_SIZE
                    || rand::random::<i32>() % 100 > 25
                {
                    if leaf.split() {
                        did_split = true;
                        new_leafs.push(*leaf.left_child.as_ref().unwrap().clone());
                        new_leafs.push(*leaf.right_child.as_ref().unwrap().clone());
                    }
                }
            }
        }

        // add new leafs to leafs
        leafs.append(&mut new_leafs);
    }

    // create rooms
    for leaf in leafs.iter_mut() {
        leaf.create_rooms();
    }

    draw_map(&leafs, root.width, root.height)
}

//pub fn gen_dungeon(width: u32, height: u32) {
pub fn gen_dungeon_system(
    mut commands: Commands,
    mut dungeon: ResMut<Dungeon>,
    assets: Res<MyAssets>,
) {
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a single section.
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Loading...".to_string(),
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 100.0,
                    color: Color::WHITE,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(LoadingText);

    dungeon.base_map = gen_dungeon_internal(dungeon.width, dungeon.height);
    format_map(dungeon.as_mut()).clone_into(&mut dungeon.base_map);

    // deallocate leafs
    //leafs.clear();

    let mut file = std::fs::File::create(format!("generation.txt")).unwrap();

    // draw to file
    for y in 0..dungeon.height {
        for x in 0..dungeon.width {
            write!(file, "{}", dungeon.base_map[y as usize][x as usize]).unwrap();
        }
        write!(file, "\r").unwrap();
    }

    // exit the program
    std::process::exit(0);
}
