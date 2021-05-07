use std::cmp::min;
use crate::drivers::Driver;
use crate::game_core::GameCore;
use crate::game_core::board::Board;
use crate::game_core::utils::point::Point;


pub struct StickyDriver<'a> {
    core: GameCore<'a>,

    gravity_frames_per_cell_per_level: &'static [usize],
    frames_since_drop: usize,

    level: usize,
    score: usize,

    lock_delay: f32,
}

impl<'a> StickyDriver<'a> {
    pub fn new(core: GameCore<'a>, gravity_table: &'static [usize], lock_delay: f32) -> Self {
        Self {
            core,

            gravity_frames_per_cell_per_level: gravity_table,
            frames_since_drop: 0,

            level: 0,
            score: 0,

            lock_delay,
        }
    }

    pub fn calculate_sticky_falls(&self, mut visit: Vec<Point>) -> Vec<(Point, Point)> {
        let lowest_point = visit.iter().map(|p| p.y()).min().unwrap();
        let board = self.core.get_board();
        let mut shapes = ShapeTracker::new(lowest_point, board);

        let mut i = 0;
        while i < visit.len() {
            let point = visit[i];
            i += 1;

            if shapes.get_shape_index(point) != None {
                continue
            }

            // explore the shape
            let shape_index = shapes.new_shape();
            let shape_value = board.get_cell(point).unwrap();
            let mut shape_fall = board.get_height() as i32;
            let mut depended_shapes = Vec::new();

            let mut shape_cells = vec![point];
            let mut j = 0;
            // now, recursively scan adjacent cells
            while j < shape_cells.len() {
                let point = shape_cells[j];
                j += 1;
                if shapes.get_shape_index(point) != None {
                    continue;
                }

                shapes.set_shape_index(point, shape_index);
                // down is a special adjacent
                let down = point - Point(0, 1);
                if board.is_on_board(down) {
                    if let Some(value) = board.get_cell(down) {
                        // if the cell is off the point of simulated gravity, it will support this shape
                        if down.y() < lowest_point {
                            shape_fall = 0;
                        } else if value == shape_value {
                            shape_cells.push(down)
                        } else {
                            depended_shapes.push(ShapeDependency::adjacent(down));
                            visit.push(down)
                        }
                    } else {
                        // check if the collision is a piece or off the board
                        let translation = board.point_first_collision(down) - Point::unit_y(1);
                        let first_collision = down + translation;
                        if first_collision.y() < lowest_point {
                            shape_fall = min(shape_fall, -translation.y());
                        } else {
                            depended_shapes.push(ShapeDependency::fall(first_collision, -translation.y()));
                        }
                    }
                } else {
                    shape_fall = 0;
                }

                let other_adjacents = [Point(0, 1), Point(1, 0), Point(-1, 0)]
                    .iter()
                    .map(|o| *o + point)
                    .filter(|p| board.is_on_board(*p) && (p.y() as usize) < board.num_active_rows());
                for adjacent in other_adjacents {
                    if let Some(value) = board.get_cell(adjacent) {
                        if value == shape_value {
                            shape_cells.push(adjacent)
                        } else {
                            visit.push(adjacent)
                        }
                    }
                }
            }

            // update the shape
            *shapes.get_shape_mut(shape_index) = (shape_fall, depended_shapes);
        }



        println!("printing");

        shapes.print();
        shapes.apply_dependencies();
        shapes.print();

        Vec::new()
    }
}

impl<'a> Driver<'a> for StickyDriver<'a> {
    fn get_game_core(&self) -> &GameCore<'a> {
        &self.core
    }

    fn get_game_core_mut(&mut self) -> &mut GameCore<'a> {
        &mut self.core
    }

    fn next_frame(&mut self) {
        self.frames_since_drop += 1;
        if self.frames_since_drop >= self.gravity_frames_per_cell_per_level[self.level] {
            self.core.fall();
            self.frames_since_drop = 0;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum ShapeDependency {
    Adjacent {
        point: Point, 
        index: usize
    },
    Fall {
        point: Point, 
        index: usize, 
        fall: i32
    }
}

impl ShapeDependency {
    fn adjacent(point: Point) -> Self {
        Self::Adjacent {
            point,
            index: 0
        }
    }

    fn fall(point: Point, fall: i32) -> Self {
        Self::Fall {
            point,
            fall,
            index: 0
        }
    }

    fn set_index(&mut self, new_index: usize) {
        match self {
            Self::Adjacent {index, ..} => *index = new_index,
            Self::Fall {index, ..} => *index = new_index
        }
    }

    fn get_point(&self) -> Point {
        match self {
            Self::Adjacent {point, ..} => *point,
            Self::Fall {point, ..} => *point
        }
    }

    fn get_index(&self) -> usize {
        match self {
            Self::Adjacent {index, ..} => *index,
            Self::Fall {index, ..} => *index
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ResolveStatus {
    NotResolved,
    Resolving,
    Resolved
}

struct ShapeTracker {
    // a board that maps each point to an index in `shapes`
    shape_indexes: Vec<Vec<Option<usize>>>,
    // all points should be translated down by base_y
    base_y: i32,
    // for each shape, records the fall distance or a vector of other shapes upon which this one rests
    shapes: Vec<(i32, Vec<ShapeDependency>)>,
}

impl ShapeTracker {
    fn new(base_y: i32, board: &Board) -> Self {
        Self {
            base_y,
            shape_indexes: vec![
                vec![None; board.get_width()]; 
                board.num_active_rows() - base_y as usize
            ],
            shapes: Vec::new()
        }
    }

    fn get_shape_index(&self, point: Point) -> Option<usize> {
        let y = (point.y() - self.base_y) as usize;
        self.shape_indexes[y][point.x() as usize]
    }

    fn set_shape_index(&mut self, point: Point, index: usize) {
        let y = (point.y() - self.base_y) as usize;
        self.shape_indexes[y][point.x() as usize] = Some(index)
    }

    fn new_shape(&mut self) -> usize {
        self.shapes.push((0, Vec::new()));
        self.shapes.len() - 1
    }

    fn get_shape_mut(&mut self, index: usize) -> &mut (i32, Vec<ShapeDependency>) {
        &mut self.shapes[index]
    }

    fn apply_dependency(&mut self, shape_index: usize, mut resolve_status: Vec<ResolveStatus>) -> Vec<ResolveStatus> {
        if resolve_status[shape_index] == ResolveStatus::Resolved {
            // no fall dependencies, the fall is calculated correctly
            return resolve_status
        }
        resolve_status[shape_index] = ResolveStatus::Resolving;

        let mut fall = self.shapes[shape_index].0;
        let mut i = 0;
        let mut num_circular_dependencies = 0;
        while i < self.shapes[shape_index].1.len() {
            let dependency = self.shapes[shape_index].1[i];
            let dep_shape = dependency.get_index();
            let status = resolve_status[dep_shape];

            if status == ResolveStatus::NotResolved {
                resolve_status = self.apply_dependency(dep_shape, resolve_status);
            }
            
            if status == ResolveStatus::Resolving || self.shapes[dep_shape].1.iter().find(|sd| sd.get_index() == shape_index).is_some() {
                self.shapes[shape_index].1[num_circular_dependencies] = self.shapes[shape_index].1[i];
                num_circular_dependencies += 1;
            } else {
                let mut dep_fall = self.shapes[dep_shape].0; 
                if let ShapeDependency::Fall{fall, ..} = dependency {
                    dep_fall += fall
                }
                fall = min(fall, dep_fall);
            }
            i += 1;
        }
        self.shapes[shape_index].1.truncate(num_circular_dependencies);
        self.shapes[shape_index].0 = fall;

        resolve_status[shape_index] = ResolveStatus::Resolved;
        resolve_status
    }

    fn apply_dependencies(&mut self) {
        // first, resolve all of the dependencies
        let mut shapes = std::mem::replace(&mut self.shapes, Vec::new());
        for shape in shapes.iter_mut() {
            for dependency in shape.1.iter_mut() {
                let index = self.get_shape_index(dependency.get_point()).unwrap();
                dependency.set_index(index);
            }
        }

        self.shapes = shapes;

        let mut resolve_status = vec![ResolveStatus::NotResolved; self.shapes.len()];
        // now recursively apply all fall dependencies
        for i in 0..self.shapes.len() {
            resolve_status = self.apply_dependency(i, resolve_status);
        }

        self.print();

        // now all the remaining dependencies are circular dependencies.  resolve those
        // shapes should be in order of discovery, so ascending
        for i in 0..self.shapes.len() {
            // skip over non-circular dependency shapes
            if self.shapes[i].1.len() == 0 {
                continue
            }

            // iterate over all of the shapes in the circular dependency and see which falls the furthest
            // 
            let cd_shapes = &mut self.shapes[i].1.iter().map(|d| d.get_index()).collect::<Vec<_>>();
            cd_shapes.push(i);
            // for each shape, pretend it is the one that hits the ground first, and then calculate 
            for _ in cd_shapes.iter() {
                let (first_impact_index, first_impact) = cd_shapes
                    .iter()
                    .cloned()
                    .map(|i| (i, self.shapes[i].0))
                    .min_by_key(|(_, d)| *d)
                    .unwrap();
                for shape in cd_shapes.iter().cloned() {
                    if self.shapes[shape].0 == first_impact {
                        self.shapes[shape].1.truncate(0);
                    } else {
                        let some_dep_idx = self.shapes[shape].1
                            .iter()
                            .enumerate()
                            .find(|(_, d)| d.get_index() == first_impact_index);
                        if let Some((dep_idx, _)) = some_dep_idx {
                            let fall = match self.shapes[shape].1[dep_idx] {
                                ShapeDependency::Fall{fall, ..} => first_impact + fall,
                                ShapeDependency::Adjacent{..} => first_impact
                            };
                            self.shapes[shape].1.remove(dep_idx);
                            self.shapes[shape].0 = min(self.shapes[shape].0, fall);
                        }
                    }
                }
            }
        }
    }

    fn print(&self) {
        println!("");
        for row in self.shape_indexes.iter().rev() {
            for cell in row.iter() {
                if let Some(index) = *cell {
                    print!("{: >2} ", index)
                } else {
                    print!("   ")
                }
            }
            println!("");
        }

        for shape in self.shapes.iter() {
            println!("{:?}", shape);
        }
    }
}