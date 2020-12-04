use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let input = "..##\n#..#\n#.#.".split('\n');
        let want = vec![
            vec![false, false, true, true],
            vec![true, false, false, true],
            vec![true, false, true, false],
        ];
        let got: Vec<Vec<bool>> = input.map(parse_line).collect();
        assert_eq!(got.len(), want.len());
        for (i, row) in want.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                assert_eq!(got[i][j], want[i][j]);
            }
        }
    }

    #[test]
    fn example() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        let trees = input.split('\n').map(parse_line).collect();
        let tests = vec![
            (1, 1, 2),
            (3, 1, 7),
            (5, 1, 3),
            (7, 1, 4),
            (1, 2, 2),
        ];
        for (r, d, want) in tests {
            let got = slope_hits(&trees, r, d);
            assert_eq!(got, want);
        }
    }
}

pub fn slope_hits(trees: &Vec<Vec<bool>>, r: usize, d: usize) -> usize {
    let mut count: usize = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let row = trees[0].len();
    while y < trees.len() {
        if trees[y][x] {
            count += 1;
        }
        x = (x + r) % row;
        y += d;
    }
    return count;
}

fn parse_line<'a>(line: &'a str) -> Vec<bool> {
    return line.chars().map(|c| c == '#').collect();
}

pub fn run() {
    let filename = "inputs/03.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let field: Vec<Vec<bool>> = contents.lines().map(parse_line).collect();
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mult = slopes.iter().map(|(r, d)| slope_hits(&field, *r, *d)).fold(1, |a, b| a*b);
    println!("{}", mult);
}
