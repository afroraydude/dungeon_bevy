use std::{io::Write, time::Instant};

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

const MIN_LEAF_SIZE: u32 = 16;
const MAX_LEAF_SIZE: u32 = 64;

const MIN_ROOM_SIZE: u32 = MIN_LEAF_SIZE - 2;

#[derive(Debug, Clone)]
struct Room {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Clone)]
struct Leaf {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub left_child: Option<Box<Leaf>>,
    pub right_child: Option<Box<Leaf>>,
    pub room: Option<Room>,
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
}

#[allow(dead_code)]
fn draw_rooms_to_file(file: &mut std::fs::File, leafs: &Vec<Leaf>, width: u32, height: u32) {
    let mut grid = vec![vec!['#'; width as usize]; height as usize];

    for leaf in leafs {
        if leaf.room.is_some() {
            if leaf.room.is_none() {
                continue;
            }
            let room = leaf.room.as_ref().unwrap();
            // draw room with # as walls and . as floor
            for x in room.x..room.x + room.w {
                for y in room.y..room.y + room.h {
                    grid[y as usize][x as usize] = '.';
                }
            }
        }
    }

    // append grid to the bottom of the file
    for row in grid {
        for cell in row {
            write!(file, "{}", cell).unwrap();
        }
        write!(file, "\r").unwrap();
    }
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

        gen_dungeon(width, height);

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

    debug!("Total time: {}ms", total_time);
    debug!("Average time: {}ms", avg_time);
    debug!("Median time: {}ms", median_time);
    debug!("Min time: {}ms", min_time);
    debug!("Max time: {}ms", max_time);
}

pub fn gen_dungeon_stress_test() {
    debug!("Starting stress test");

    debug!("Generating 256x256 dungeon");
    gen_dungeon_stress_test_internal(256, 256);

    debug!("Generating 1024x1024 dungeon");
    gen_dungeon_stress_test_internal(1024, 1024);

    debug!("Generating 4096x4096 dungeon");
    gen_dungeon_stress_test_internal(4096, 4096);


    // exit the program
    std::process::exit(0);
}

pub fn gen_dungeon(width: u32, height: u32) {
    let mut leafs: Vec<Leaf> = Vec::new();

    let root = Leaf::new(0, 0, width, width);

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

    for leaf in leafs.iter_mut() {
        leaf.create_rooms();
    }

    /*write!(file, "Dungeon {}:\r", i).unwrap();
    draw_rooms_to_file(&mut file, &leafs, root.width, root.height);
    write!(file, "\r").unwrap();*/
}
