use std::collections::VecDeque;
use bevy::prelude::*;

pub mod temp;


#[derive(Component)]
pub struct Belt {
    item_clusters: VecDeque<BeltItemCluster>,
    unjammed_cluster_index: Option<usize>,
    max_item_len: usize,
    cur_item_len: usize,
    back_dist: usize
}

impl Belt {
    pub fn move_items(&mut self) {
        if let Some(unjammed_cluster_index) = self.unjammed_cluster_index {
            self.item_clusters[unjammed_cluster_index].front_dist -= 1;
            self.back_dist += 1;
            if self.item_clusters[unjammed_cluster_index].front_dist <= 0 {
                if self.item_clusters.len() - 1 > unjammed_cluster_index {
                    self.unjammed_cluster_index = Some(unjammed_cluster_index + 1);
                }
                else {
                    self.unjammed_cluster_index = None;
                }
            }
        }
        //self.back_dist = self.back_dist.clamp(0, self.max_item_len + 1);
    }

    pub fn can_push_item(&self) -> bool {
        return self.item_clusters.len() > 0 && self.item_clusters[0].front_dist <= 0;
    }
    pub fn try_push_item(&mut self) -> bool {
        if !self.can_push_item() || self.item_clusters.len() <= 0 { return false; }

        self.push_item();

        return true;
        
    }
    pub fn push_item(&mut self) {
        self.item_clusters[0].amount -= 1;
        self.cur_item_len -= 1;
        if self.item_clusters[0].amount == 0 {
            self.item_clusters.pop_front();
        }
        if self.item_clusters.len() > 0 {
            self.unjammed_cluster_index = Some(0);
            self.item_clusters[0].front_dist += 1;
        }
        else {
            self.unjammed_cluster_index = None;
        }       
    }

    pub fn can_take_item(&self) -> bool {
        return self.cur_item_len < self.max_item_len && self.back_dist >= 1;
    }
    pub fn take_item(&mut self, item_id: u32) {
        if self.back_dist == 1 && self.item_clusters.len() > 0 && self.item_clusters.back().unwrap().item_id == item_id {
            // back item cluser matches this item
            self.item_clusters.back_mut().unwrap().amount += 1;
        }
        else {
            self.item_clusters.push_back(BeltItemCluster {
                item_id,
                amount: 1,
                front_dist: self.back_dist - 1,
            });
            self.unjammed_cluster_index = Some(0);
        }
        self.back_dist = 0;
        self.cur_item_len += 1;
    }
    pub fn try_take_item(&mut self, item_id: u32) -> bool {
        if self.can_take_item() == false { return false; }
        self.take_item(item_id);
        return true;
    }

}
#[derive(Debug)]
struct BeltItemCluster {
    item_id: u32,
    amount: u32,
    front_dist: usize
}




pub fn progress_belts(
    mut query: Query<&mut Belt>
) {
    for mut belt in &mut query {
        belt.try_push_item();
        belt.move_items();
        belt.try_push_item();
    }
}



