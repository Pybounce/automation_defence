
use bevy::{prelude::*, transform::commands};

use super::*;


pub fn feed_belts(
    mut query: Query<&mut Belt>
) {
    for mut belt in &mut query {
        belt.try_take_item(0);
    }
}

pub fn insert_belts(
    mut commands: Commands
) {
    commands.spawn(Belt {
        item_clusters: VecDeque::default(),
        unjammed_cluster_index: None,
        max_item_len: 5,
        cur_item_len: 0,
        back_dist: 6,   // since there are no items on it?
    });
    
}

pub fn manually_progress(
    mut query: Query<&mut Belt>,
    input: Res<ButtonInput<KeyCode>>,

) {
    if input.just_pressed(KeyCode::Space) {
        for mut belt in &mut query {
            belt.try_push_item();
            belt.move_items();
            belt.try_push_item();
        }
    }
}

pub fn manually_feed(
    mut query: Query<&mut Belt>,
    input: Res<ButtonInput<KeyCode>>,

) {
    if input.just_pressed(KeyCode::KeyF) {
        for mut belt in &mut query {
            belt.try_take_item(0);
        }
    }
}

pub fn print_belts(
    query: Query<&Belt>,
    input: Res<ButtonInput<KeyCode>>,

) {
    if input.just_pressed(KeyCode::KeyP) {
        for belt in &query {
            println!("------------------------------");
            println!("back_dist:        {}", belt.back_dist);
            println!("unjammed_index:   {:?}", belt.unjammed_cluster_index);
            println!("max_item_len:     {}", belt.max_item_len);
            println!("cur_item_len:     {}", belt.cur_item_len);
            for cluster in belt.item_clusters.iter() {
                println!("cluster:          {:?}", cluster);
            }
        }


    }
}


/*


#[derive(Component)]
pub struct Belt {
    item_clusters: VecDeque<BeltItemCluster>,
    unjammed_cluster_index: Option<usize>,
    max_item_len: usize,
    back_dist: usize
}
    
    struct BeltItemCluster {
    item_id: u32,
    amount: u32,
    front_dist: usize
}

    
    */