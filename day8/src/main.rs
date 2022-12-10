#![feature(is_sorted)]
use std::env;
use std::fs;
use std::collections::HashSet;
use std::ops::Range;

#[derive(Debug)]
struct TreeMap {
    tree_heights: Vec<usize>,
    width: usize,
    height: usize
}

impl TreeMap {
    fn from_text_input(text: &str) -> Self {
        let tree_heights : Vec<usize> = text.chars().filter(|&y| y.is_ascii_digit()).map(|y| y.to_digit(10).unwrap() as usize).collect();
        let height = text.lines().count();
        let width = tree_heights.len() / height;
        Self { tree_heights, width, height }
    }

    fn pos_to_index(&self, x: usize, y: usize) -> usize {
         y * self.width + x
    }

    fn filter_visible_trees(&self)  {

        let mut tree_visibility_map  = vec![false; self.tree_heights.len()]; 
        
        let get_trees_from_edge_ranged_width = |w: Range<usize>, h: usize| {
            let mut tree_height = Vec::new();
            if w.start >= w.end{
                return tree_height;
            }
            for x in w {
                if let Some(tree) = self.tree_heights.get(self.pos_to_index(x, h)) {
                    tree_height.push(*tree);
                }
            }
            tree_height
        };

        let get_trees_from_edge_ranged_height = |w: usize, h: Range<usize>| {
            let mut tree_height = Vec::new();
            if h.start >= h.end{
                return tree_height;
            }
            for y in h {
                if let Some(tree) = self.tree_heights.get(self.pos_to_index(w, y)) {
                    tree_height.push(*tree);
                }
            }
            tree_height
        };

        let mut visible_tree_count = 0;
        // bottom to top
        for x in 0..self.width {
            for y in 0..self.height {
                
                let trees_beneath = get_trees_from_edge_ranged_height(x, (0..y));
                let trees_above = get_trees_from_edge_ranged_height(x, (y+1..self.height));

                let trees_to_left = get_trees_from_edge_ranged_width((0..x), y);
                let trees_to_right = get_trees_from_edge_ranged_width((x+1..self.width), y);
            
                
                if trees_above.is_empty() || trees_beneath.is_empty() || trees_to_left.is_empty() || trees_to_right.is_empty() {
                    visible_tree_count += 1;
                    continue;
                }

                let current_tree_height = self.tree_heights[self.pos_to_index(x, y)];
                if trees_above.iter().all(|y| *y < current_tree_height) {
                    visible_tree_count += 1;
                    continue;
                }
                if trees_beneath.iter().all(|y| *y < current_tree_height) {
                    visible_tree_count += 1;
                    continue;
                }
                if trees_to_left.iter().all(|y| *y < current_tree_height) {
                    visible_tree_count += 1;
                    continue;
                }
                if trees_to_right.iter().all(|y| *y < current_tree_height) {
                    visible_tree_count += 1;
                    continue;
                }
            }
        }
      
        println!("Trees visible from outside: {}", visible_tree_count);

    }
}

fn riddle_part_one(file_path: &String)  {
    let text = fs::read_to_string(file_path).expect("Error reading file");

    let tree_map = TreeMap::from_text_input(&text);
    tree_map.filter_visible_trees();
}

fn riddle_part_two(file_path: &String) {
    let text = fs::read_to_string(file_path).expect("Error reading file");

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Provide the input text file!");
    }
    let riddle_num: u32 = args
        .get(1)
        .unwrap()
        .parse()
        .expect("Error parsing riddle num");

    match riddle_num {
        1 => {
            riddle_part_one(args.get(2).unwrap());
        }
        2 => {
            riddle_part_two(args.get(2).unwrap());
        }
        _ => {
            panic!("Unknown riddle part number");
        }
    };
}
