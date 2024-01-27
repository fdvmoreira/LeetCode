// --- Day 10: Pipe Maze ---
// You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.
//
// You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.
//
// The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.
//
// Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).
//
// The pipes are arranged in a two-dimensional grid of tiles:
//
// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
// Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.
//
// For example, here is a square loop of pipe:
//
// .....
// .F-7.
// .|.|.
// .L-J.
// .....
// If the animal had entered this loop in the northwest corner, the sketch would instead look like this:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.
//
// Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:
//
// -L|F7
// 7S-7|
// L|7||
// -L-J|
// L|-JF
// In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).
//
// Here is a sketch that contains a slightly more complex main loop:
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:
//
// 7-F7-
// .FJ|7
// SJLL7
// |F--J
// LJ.LJ
// If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.
//
// In the first example with the square loop:
//
// .....
// .S-7.
// .|.|.
// .L-J.
// .....
// You can count the distance each tile in the loop is from the starting point like this:
//
// .....
// .012.
// .1.3.
// .234.
// .....
// In this example, the farthest point from the start is 4 steps away.
//
// Here's the more complex loop again:
//
// ..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...
// Here are the distances for each tile on that loop:
//
// ..45.
// .236.
// 01.78
// 14567
// 23...
// Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?
//---
// Find the loop
// Assign a incremental number to each tile that forms the loop
// Return the half of the total pipe length
//

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Location {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipeType {
    Horizontal,
    Vertical,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
}

fn get_pipe(tile: char) -> Option<PipeType> {
    match tile {
        '-' => Some(PipeType::Horizontal),
        '|' => Some(PipeType::Vertical),
        'J' => Some(PipeType::SouthWest),
        'L' => Some(PipeType::SouthEast),
        '7' => Some(PipeType::NorthWest),
        'F' => Some(PipeType::NorthEast),
        _ => Some(PipeType::Ground),
    }
}

fn get_neighbouring_pipes<Location, PipeType>(
    matrix: &Vec<Vec<char>>,
    current_tile: (u32, u32),
) -> Option<[(Location, PipeType); 4]> {
    let mut neighbours: Vec<(Location, PipeType)> = Vec::new();

    let left = if current_tile.1 > 0 {
        Some(matrix[current_tile.0 as usize][current_tile.1 as usize - 1])
    } else {
        None
    };

    let top = if current_tile.0 > 0 {
        Some(matrix[current_tile.0 as usize - 1][current_tile.1 as usize])
    } else {
        None
    };

    let right = if (current_tile.1 as usize) < matrix[0].len() - 2 {
        Some(matrix[current_tile.0 as usize][current_tile.1 as usize + 1])
    } else {
        None
    };

    let bottom = if (current_tile.0 as usize) < matrix.len() - 2 {
        Some(matrix[current_tile.0 as usize + 1][current_tile.1 as usize])
    } else {
        None
    };

    // neighbours.push(());

    neighbours.push((Location::WEST, get_pipe(left.unwrap()).unwrap()));
    neighbours.push((Location::NORTH, get_pipe(top.unwrap()).unwrap()));
    neighbours.push((Location::EAST, get_pipe(right.unwrap()).unwrap()));
    neighbours.push((Location::SOUTH, get_pipe(bottom.unwrap()).unwrap()));
    // TODO: add the Pipes to the array by checking the pipe starting in the West and ending in the
    // South
    // Get the pipe type from enum and couple them with the direction value
    //
    Some(neighbours.try_into().unwrap())
}

fn get_next_tile(
    matrix: &Vec<Vec<char>>,
    prev_tile: (u32, u32),
    current_tile: (u32, u32),
) -> Option<(u32, u32)> {
    todo!()
    // find the pipes connected to current pipe
    // exclude the previous one and the next one should be the one left
    // let n_pipes = get_neighbouring_pipes(matrix, current_tile).unwrap();
}

fn get_start_index(grid: &Vec<Vec<char>>) -> Option<(u32, u32)> {
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                return Some((row as u32, col as u32));
            }
        }
    }
    None
}

pub fn furthest_point_from_start(data: &[&str]) -> Option<u32> {
    if data.is_empty() {
        return None;
    }

    let grid: Vec<Vec<char>> = data
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = get_start_index(&grid).unwrap();

    // find next pipe until the loop is completed
    // save distance of each pipe from starting point
    let mut current_tile = start.clone();
    let mut prev_tile: Option<(u32, u32)> = None;
    let mut next_tile = get_next_tile(&grid, prev_tile?, current_tile).unwrap();
    let mut pipe_counter: u32 = 0;

    while next_tile != start {
        pipe_counter += 1;
        prev_tile = Some(current_tile);
        current_tile = next_tile;
        next_tile = get_next_tile(&grid, prev_tile?, current_tile).unwrap();
    }

    let mid_pipe = pipe_counter.saturating_div(2);

    Some(mid_pipe)
}

#[cfg(test)]
mod tests {

    use super::*;
    use googletest::{assert_that, matchers::eq};
    use rstest::rstest;

    #[rstest]
    #[test]
    #[case(vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."],4)]
    #[case(vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."],8)]
    fn total_steps_returns_the_number_of_steps(#[case] input: Vec<&str>, #[case] expected: u32) {
        let actual = furthest_point_from_start(&input).unwrap();
        assert_that!(actual, eq(expected));
    }

    #[rstest]
    #[test]
    #[case(
            vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.']
                ], (1, 1))]
    #[case(
            vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.']
                ], (2, 0))]
    fn given_a_non_empty_matrix_get_start_index_should_return(
        #[case] input: Vec<Vec<char>>,
        #[case] expected: (u32, u32),
    ) {
        let actual = get_start_index(&input).unwrap();
        assert_that!(actual, eq(expected));
    }

    #[rstest]
    #[test]
    #[case(
            vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.']
                ], Some((1,2)), (1,3), (2, 3))]
    #[case(
            vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.']
                ], Some((2,0)), (2,1),(1, 1))]
    fn given_current_pos_get_next_tile_returns_the_next_tile_coords(
        #[case] matrix: Vec<Vec<char>>,
        #[case] prev_tile: Option<(u32, u32)>,
        #[case] current_tile: (u32, u32),
        #[case] expected: (u32, u32),
    ) {
        let actual = get_next_tile(&matrix, prev_tile.unwrap(), current_tile).unwrap();
        assert_that!(actual, eq(expected));
    }

    #[rstest]
    #[test]
    #[case(
            vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', 'S', '-', '7', '.'],
            vec!['.', '|', '.', '|', '.'],
            vec!['.', 'L', '-', 'J', '.'],
            vec!['.', '.', '.', '.', '.']
                ], (2,3), [(Location::WEST, PipeType::Ground),(Location::NORTH,PipeType::NorthWest),(Location::EAST,PipeType::Ground),(Location::SOUTH, PipeType::SouthWest)])]
    #[case(
            vec![
            vec!['.', '.', 'F', '7', '.'],
            vec!['.', 'F', 'J', '|', '.'],
            vec!['S', 'J', '.', 'L', '7'],
            vec!['|', 'F', '-', '-', 'J'],
            vec!['L', 'J', '.', '.', '.']
                ], (3,1), [(Location::WEST, PipeType::Vertical),(Location::NORTH,PipeType::SouthWest),(Location::EAST,PipeType::Horizontal),(Location::SOUTH, PipeType::SouthWest)])]

    fn get_neighbouring_pipes_returns_the_pipes_sorrounding_the_current_pipe(
        #[case] matrix: Vec<Vec<char>>,
        #[case] current_tile: (u32, u32),
        #[case] expected: [(Location, PipeType); 4],
    ) {
        let actual = get_neighbouring_pipes(&matrix, current_tile).unwrap();
        assert_that!(actual, eq(expected));
    }

    #[rstest]
    #[test]
    #[case('-', PipeType::Horizontal)]
    #[case('|', PipeType::Vertical)]
    #[case('F', PipeType::NorthEast)]
    #[case('7', PipeType::NorthWest)]
    #[case('L', PipeType::SouthEast)]
    #[case('J', PipeType::SouthWest)]
    #[case('.', PipeType::Ground)]
    fn get_pipe_return_the_pipe_name_given_a_char_representation_of_a_pipe(
        #[case] input: char,
        #[case] expected: PipeType,
    ) {
        let actual = get_pipe(input).unwrap();
        assert_that!(actual, eq(expected));
    }
}
