use neerajsi::read_stdin_input;

struct Shape {
    // bitmaps indicating which squares of the 3x3 grid are filled
    filled_squares: [u8; 3],
}

struct Area {
    width: usize,
    height: usize,

    shape_counts: Vec<usize>,
}

struct Puzzle {
    shapes: Vec<Shape>,

    areas: Vec<Area>,
}

fn parse_shape<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Shape {
    let mut filled_squares = [0u8; 3];
    
    for row in 0..3 {
        let line = lines.next().expect("Expected 3 rows for shape");
        for (col, ch) in line.chars().take(3).enumerate() {
            if ch == '#' {
                filled_squares[row] |= 1 << col;
            }
        }
    }
    
    Shape { filled_squares }
}

fn parse_area(line: &str) -> Area {
    let (area, counts) = line.split_once(':').unwrap();
    
    // Parse "4x4:" or "12x5:"
    let (width, height) = area.split_once('x').unwrap();
    let width = width.parse().expect("Failed to parse width");
    let height = height.parse().expect("Failed to parse height");
    
    // Parse shape counts
    let shape_counts: Vec<usize> = counts
        .split_ascii_whitespace()
        .map(|s| s.parse().expect("Failed to parse shape count"))
        .collect();
    
    Area {
        width,
        height,
        shape_counts,
    }
}

fn parse_puzzle(input: &str) -> Puzzle {
    let mut lines = input.lines().map(|line| line.trim()).peekable();
    
    let mut shapes = Vec::new();
    let mut areas = Vec::new();
    
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        
        // Check if this is a shape definition (starts with a number and colon)
        if line.ends_with(':') && line.chars().next().unwrap().is_ascii_digit() {
            // Parse the shape
            let shape = parse_shape(&mut lines);
            shapes.push(shape);
        } else if line.contains('x') && line.contains(':') {
            // This is an area definition
            let area = parse_area(line);
            areas.push(area);
        }
    }
    
    Puzzle { shapes, areas }
}

fn main() {
    let input = String::from_utf8(read_stdin_input()).unwrap();
    
    let puzzle = parse_puzzle(&input);

    struct ShapeInfo {
        area: usize
    };

    let mut shape_infos = Vec::new();
    for shape in &puzzle.shapes {
        let area = shape.filled_squares.iter().map(|&row| row.count_ones() as usize).sum();
        shape_infos.push(ShapeInfo { area });
    }
    
    let mut trivially_possible_count = 0;
    let mut uncertain_count = 0;
    for (i, a) in puzzle.areas.iter().enumerate() {
        println!("Area {}: {}x{}, Shape counts: {:?}", i, a.width, a.height, a.shape_counts);

        let area = a.width * a.height;
        let shape_min_area = a.shape_counts.iter().enumerate().map(|(idx, &count)| count * shape_infos[idx].area).sum::<usize>();
        let total_shape_count = a.shape_counts.iter().sum::<usize>();
        let shape_max_area = total_shape_count * 9;

        // find area with width and height rounded down to nearest multiple of 3
        let rounded_width = (a.width / 3) * 3;
        let rounded_height = (a.height / 3) * 3;
        let bounding_area = rounded_width * rounded_height;

        println!("  Total area: {}, Shape min area: {}, Shape max area: {}, Bounding area: {}", area, shape_min_area, shape_max_area, bounding_area);

        if shape_min_area > area {
            println!("  Impossible: shape min area exceeds area");
        } else if shape_max_area <= bounding_area {
            println!("  Possible: trivially fits within bounding area");
            trivially_possible_count += 1;
        } else {
            println!("  Uncertain: requires detailed packing analysis");
            uncertain_count += 1;
        }
    }

    println!("Trivially possible areas: {}", trivially_possible_count);
    println!("Uncertain areas: {}", uncertain_count);

}
