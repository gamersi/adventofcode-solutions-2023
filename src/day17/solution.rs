use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

const INPUT: &str = include_str!("input.txt");

type Position = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    position: Position,
    remaining_up: u8,
    remaining_right: u8,
    remaining_down: u8,
    remaining_left: u8,
    direction: Direction,
    has_moved: Option<u8>,
}

pub fn part1() {
    let blocks: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = (0, 0);
    let target = (blocks[0].len() - 1, blocks.len() - 1);
    let min_loss: usize = find_most_efficient_path(&blocks, start, target);
    println!("Part 1: {}", min_loss);
}

pub fn part2() {
    let blocks: Vec<Vec<usize>> = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let start = (0, 0);
    let target = (blocks[0].len() - 1, blocks.len() - 1);
    let min_loss: usize = find_most_efficient_path_ultra(&blocks, start, target);
    println!("Part 2: {}", min_loss);
}

fn get_adjacent_states(state: State, blocks: &Vec<Vec<usize>>) -> Vec<State> {
    let mut neighbours: Vec<State> = Vec::new();

    if state.remaining_up >= 1 && state.position.0 >= 1 && state.direction != Direction::Down {
        neighbours.push(State {
            position: (state.position.0 - 1, state.position.1),
            remaining_up: state.remaining_up - 1,
            remaining_right: 3,
            remaining_down: 3,
            remaining_left: 3,
            direction: Direction::Up,
            has_moved: None,
        });
    }

    if state.remaining_right >= 1
        && state.position.1 < blocks[0].len() - 1
        && state.direction != Direction::Left
    {
        neighbours.push(State {
            position: (state.position.0, state.position.1 + 1),
            remaining_up: 3,
            remaining_right: state.remaining_right - 1,
            remaining_down: 3,
            remaining_left: 3,
            direction: Direction::Right,
            has_moved: None,
        });
    }

    if state.remaining_down >= 1
        && state.position.0 < blocks.len() - 1
        && state.direction != Direction::Up
    {
        neighbours.push(State {
            position: (state.position.0 + 1, state.position.1),
            remaining_up: 3,
            remaining_right: 3,
            remaining_down: state.remaining_down - 1,
            remaining_left: 3,
            direction: Direction::Down,
            has_moved: None,
        });
    }

    if state.remaining_left >= 1 && state.position.1 >= 1 && state.direction != Direction::Right {
        neighbours.push(State {
            position: (state.position.0, state.position.1 - 1),
            remaining_up: 3,
            remaining_right: 3,
            remaining_down: 3,
            remaining_left: state.remaining_left - 1,
            direction: Direction::Left,
            has_moved: None,
        });
    }

    neighbours
}

fn find_most_efficient_path(blocks: &Vec<Vec<usize>>, start: Position, target: Position) -> usize {
    let mut loss: HashMap<State, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let default_state = State {
        position: start,
        remaining_up: 3,
        remaining_right: 3,
        remaining_down: 3,
        remaining_left: 3,
        direction: Direction::Right,
        has_moved: None,
    };

    loss.insert(default_state, 0);
    heap.push(Reverse((0, default_state)));

    while let Some(Reverse((current_loss, current_state))) = heap.pop() {
        if current_state.position == target {
            return current_loss;
        }

        if current_loss > *loss.get(&current_state).unwrap_or(&usize::MAX) {
            continue;
        }

        let adjacent_states = get_adjacent_states(current_state, blocks);

        for neighbour in adjacent_states {
            let new_pos = neighbour.position;
            let new_loss = current_loss + blocks[new_pos.0][new_pos.1];
            if new_loss < *loss.get(&neighbour).unwrap_or(&usize::MAX) {
                loss.insert(neighbour, new_loss);
                heap.push(Reverse((new_loss, neighbour)));
            }
        }
    }

    usize::MAX
}

// part 2

fn get_adjacent_states_ultra(state: State, blocks: &Vec<Vec<usize>>) -> Vec<State> {
    let mut neighbours: Vec<State> = Vec::new();

    if state.has_moved < Some(4) {
        match state.direction {
            Direction::Up => {
                if state.position.0 >= 1 {
                    neighbours.push(State {
                        position: (state.position.0 - 1, state.position.1),
                        remaining_up: state.remaining_up - 1,
                        remaining_right: 10,
                        remaining_down: 10,
                        remaining_left: 10,
                        direction: Direction::Up,
                        has_moved: Some(state.has_moved.unwrap() + 1),
                    });
                }
                return neighbours;
            }
            Direction::Right => {
                if state.position.1 < blocks[0].len() - 1 {
                    neighbours.push(State {
                        position: (state.position.0, state.position.1 + 1),
                        remaining_up: 10,
                        remaining_right: state.remaining_right - 1,
                        remaining_down: 10,
                        remaining_left: 10,
                        direction: Direction::Right,
                        has_moved: Some(state.has_moved.unwrap() + 1),
                    });
                }
                return neighbours;
            }
            Direction::Down => {
                if state.position.0 < blocks.len() - 1 {
                    neighbours.push(State {
                        position: (state.position.0 + 1, state.position.1),
                        remaining_up: 10,
                        remaining_right: 10,
                        remaining_down: state.remaining_down - 1,
                        remaining_left: 10,
                        direction: Direction::Down,
                        has_moved: Some(state.has_moved.unwrap() + 1),
                    });
                }
                return neighbours;
            }
            Direction::Left => {
                if state.position.1 >= 1 {
                    neighbours.push(State {
                        position: (state.position.0, state.position.1 - 1),
                        remaining_up: 10,
                        remaining_right: 10,
                        remaining_down: 10,
                        remaining_left: state.remaining_left - 1,
                        direction: Direction::Left,
                        has_moved: Some(state.has_moved.unwrap() + 1),
                    });
                }
                return neighbours;
            }
        }
    }

    if state.remaining_up >= 1 && state.position.0 >= 1 && state.direction != Direction::Down {
        neighbours.push(State {
            position: (state.position.0 - 1, state.position.1),
            remaining_up: state.remaining_up - 1,
            remaining_right: 10,
            remaining_down: 10,
            remaining_left: 10,
            direction: Direction::Up,
            has_moved: if state.direction == Direction::Up {
                Some(state.has_moved.unwrap() + 1)
            } else {
                Some(1)
            },
        });
    }

    if state.remaining_right >= 1
        && state.position.1 < blocks[0].len() - 1
        && state.direction != Direction::Left
    {
        neighbours.push(State {
            position: (state.position.0, state.position.1 + 1),
            remaining_up: 10,
            remaining_right: state.remaining_right - 1,
            remaining_down: 10,
            remaining_left: 10,
            direction: Direction::Right,
            has_moved: if state.direction == Direction::Right {
                Some(state.has_moved.unwrap() + 1)
            } else {
                Some(1)
            },
        });
    }

    if state.remaining_down >= 1
        && state.position.0 < blocks.len() - 1
        && state.direction != Direction::Up
    {
        neighbours.push(State {
            position: (state.position.0 + 1, state.position.1),
            remaining_up: 10,
            remaining_right: 10,
            remaining_down: state.remaining_down - 1,
            remaining_left: 10,
            direction: Direction::Down,
            has_moved: if state.direction == Direction::Down {
                Some(state.has_moved.unwrap() + 1)
            } else {
                Some(1)
            },
        });
    }

    if state.remaining_left >= 1 && state.position.1 >= 1 && state.direction != Direction::Right {
        neighbours.push(State {
            position: (state.position.0, state.position.1 - 1),
            remaining_up: 10,
            remaining_right: 10,
            remaining_down: 10,
            remaining_left: state.remaining_left - 1,
            direction: Direction::Left,
            has_moved: if state.direction == Direction::Left {
                Some(state.has_moved.unwrap() + 1)
            } else {
                Some(1)
            },
        });
    }

    neighbours
}

fn find_most_efficient_path_ultra(
    blocks: &Vec<Vec<usize>>,
    start: Position,
    target: Position,
) -> usize {
    let mut loss: HashMap<State, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let default_state_right = State {
        position: start,
        remaining_up: 10,
        remaining_right: 10,
        remaining_down: 10,
        remaining_left: 10,
        direction: Direction::Right,
        has_moved: Some(0),
    };
    let default_state_down = State {
        position: start,
        remaining_up: 10,
        remaining_right: 10,
        remaining_down: 10,
        remaining_left: 10,
        direction: Direction::Down,
        has_moved: Some(0),
    };

    loss.insert(default_state_right, 0);
    loss.insert(default_state_down, 0);
    heap.push(Reverse((0, default_state_right)));
    heap.push(Reverse((0, default_state_down)));

    while let Some(Reverse((current_loss, current_state))) = heap.pop() {
        if current_state.position == target && current_state.has_moved >= Some(4) {
            return current_loss;
        }

        if current_loss > *loss.get(&current_state).unwrap_or(&usize::MAX) {
            continue;
        }

        let adjacent_states = get_adjacent_states_ultra(current_state, blocks);

        for neighbour in adjacent_states {
            let new_pos = neighbour.position;
            let new_loss = current_loss + blocks[new_pos.0][new_pos.1];
            if new_loss < *loss.get(&neighbour).unwrap_or(&usize::MAX) {
                loss.insert(neighbour, new_loss);
                heap.push(Reverse((new_loss, neighbour)));
            }
        }
    }

    usize::MAX
}
