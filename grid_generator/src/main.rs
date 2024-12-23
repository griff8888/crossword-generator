use printpdf::*;
use printpdf::path::PaintMode;
use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::BufWriter;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
struct Square {
    fill: printpdf::path::PaintMode,
    x_mm: Mm,
    y_mm: Mm,
    x_pos: i32,
    y_pos: i32,
}

#[derive(Debug, Clone)]
struct Row {
    y: i32,
    vec: Vec<Square>,
}

#[derive(Debug, Clone)]
struct Column {
    x: i32,
    vec: Vec<Square>,
}

#[derive(Debug, Clone,)]
struct Grid{
    size: i32,
    rows: Vec<Row>,
    columns: Vec<Column>,
}

fn generate_top(mut grid: Grid, size: i32, width: f32) -> Grid {
    let mut x_pos: i32 = 0; 
    let mut y_pos: i32 = 0;
    let start_x = Mm(5.0);
    let start_y = Mm(292.0 - width);
    let mut y_mm = start_y;
    let mut rng = thread_rng();
    
    while y_pos < (size / 2) {
        x_pos = 0;
        // defines the vector that will represent a specific horizontal row
        let mut squares = Row {
            y: y_pos,
            vec: Vec::<Square>::new(),
        };
        while x_pos < size {
            // decides if square will be blacked out
            let num = rng.gen_range(0..5);
            let mut mode: PaintMode = PaintMode::Stroke;
            if num == 0 {
                mode = PaintMode::Fill;
            }
           
            let x_mm = start_x + Mm(x_pos as f32) * width;
           // defines a new square object
            let square = Square {
                fill: mode,
                x_mm,
                y_mm,
                x_pos,
                y_pos,

            };
           
            squares.vec.push(square);
            x_pos += 1;
        }
        grid.rows.push(squares);
        y_mm -= Mm(width);
        y_pos += 1;
    }

    Grid {
        size: grid.size,
        rows: grid.rows,
        columns: Vec::<Column>::new(),
    }
}





fn main() {
    // creates a pdf with a plain border 
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open(String::from("../ARIAL.TTF")).unwrap()).unwrap();
    //let font = doc.add_external_font(File::open(String::from("../../ARIAL.TTF")).unwrap());
    let outline = Rect::new(Mm(5.0), Mm(5.0), Mm(205.0), Mm(292.0)).with_mode(PaintMode::Stroke);
    layer.add_rect(outline);
    // define square_ct -> all other variables are dependent on this variable
    let square_ct = 5;
    // because crossword follows symmetry, square_ct must be odd so that there is a 'middle'
    if square_ct % 2 == 0 || square_ct < 5 {
        println!("# of squares must be an odd whole number larger than 5");
        exit(0);
    }
    // defining variables that will be repeatedly used
    // Mm(x,y) refers to coordinates on the pdf in milimeters 
    // the bottom left corner is Mm(0,0)
    let square_size = 200.0 / square_ct as f32;
    let start_x = Mm(5.0);
    let start_y = Mm(292.0 - square_size);
    let mut y_mm = start_y.clone();
    let mut x_mm = start_x.clone();
    let mut x_pos = 0;
    let mut y_pos = 0;
    // this vector will hold the top half of the grid
    // for a 15x15 grid, this vector will hold the top 15x8 grid
    // dummy variable needed to use rng
    let mut rng = thread_rng();

    let mut grid = Grid {
        size: square_ct,
        rows: Vec::<Row>::new(),
        columns: Vec::<Column>::new(),
    };
    // outer while loop iterates over the outer vector that holds the entire row
    // while y_pos < ((square_ct / 2)) {
    //     x_pos = 0;
    //     // defines the vector that will represent a specific horizontal row
    //     let mut squares = Row {
    //         y: y_pos,
    //         vec: Vec::<Square>::new(),
    //     };
    //     while x_pos < square_ct {
    //         // decides if square will be blacked out
    //         let num = rng.gen_range(0..5);
    //         let mut mode: PaintMode = PaintMode::Stroke;
    //         if num == 0 {
    //             mode = PaintMode::Fill;
    //         }
           
    //         x_mm = start_x + Mm(x_pos as f32) * square_size;
    //        // defines a new square object
    //         let square = Square {
    //             fill: mode,
    //             x_mm,
    //             y_mm,
    //             x_pos,
    //             y_pos,

    //         };
           
    //         squares.vec.push(square);
    //         x_pos += 1;
    //     }
    //     grid.rows.push(squares);
    //     y_mm -= Mm(square_size);
    //     y_pos += 1;
    // }
    
    grid = generate_top(grid, square_ct, square_size);

    // println!("{:#?}", grid);

    x_pos = 0;
    y_pos = (square_ct - 1) / 2;
    let mut middle_row = Row {
        y: y_pos,
        vec: Vec::<Square>::new(),
    };

    while x_pos < ((square_ct / 2) + 1) {
        let num = rng.gen_range(0..5);
        let mut mode: PaintMode = PaintMode::Stroke;
        if num == 0 {
            mode = PaintMode::Fill;
        }
        //x_mm = start_x + Mm(x_pos as f32) * square_size;
        x_mm = Mm(5.0) + Mm(square_size * (x_pos - 1) as f32);
        // defines a new square object
        let square = Square {
            fill: mode,                
            x_mm,
            y_mm,
            x_pos,
            y_pos,
        };
        middle_row.vec.push(square);
        x_pos += 1;
    }


    let mut middle_row_rev = middle_row.vec.clone();
    middle_row_rev.remove(middle_row_rev.len() - 1);

    while x_pos < square_ct  {
            let reference_square = &middle_row_rev[(square_ct - x_pos - 1) as usize];
            let square = Square {
                fill: reference_square.fill,
                x_mm: Mm(205.0 - ((square_ct - x_pos) as f32 * square_size)),
                y_mm,
                x_pos: square_ct - reference_square.x_pos - 1,
                y_pos: reference_square.y_pos,
            };

            middle_row.vec.push(square);
            x_pos += 1;
    }

    println!("{:#?}", middle_row.vec);

    // defines another vector of rows that will combine the 'rows' vector and 'rows_symmetrical' vector
    grid.rows.push(middle_row);
    y_pos = square_ct - 1;
    y_mm = Mm(292.0 - (square_ct as f32 * square_size));
    //creates a reverse of 'rows'
    //the vector is reversed as well as each vector held within the broader vector
    let mut rows_rev = Vec::new();
    for row in &grid.rows {
        let mut row_rev = row.clone();
        row_rev.vec.reverse();
        rows_rev.push(row_rev);
    }
    rows_rev.remove(rows_rev.len() - 1);
    // essentially the same loop as above, doing the same thing for the grid that makes up the lower half
    while y_pos >= ((square_ct / 2) + 1) {
        x_pos = 0;
        let mut squares = Row {
            y: y_pos,
            vec: Vec::<Square>::new(),

        };
        while x_pos < square_ct {
            let reference_square = &rows_rev[(square_ct - y_pos - 1) as usize].vec[(x_pos) as usize];
            let square = Square {
                fill: reference_square.fill,
                x_mm: Mm(205.0 - ((square_ct - x_pos) as f32 * square_size)),
                y_mm,
                x_pos,
                y_pos,
            };

            squares.vec.push(square);
            x_pos += 1;
        }
        grid.rows.push(squares);
        y_mm += Mm(square_size);
        y_pos -= 1;
    }
    let mut ct = 1;
    for squares in grid.rows {
        //println!("{:#?}", squares);
        for square in squares.vec {
            //println!("{} ({}, {})", ct, square.x_pos, square.y_pos);
            let text = format!("{}, {}", square.x_pos.to_string(), square.y_pos.to_string());
            //layer.use_text(text, 40.0, square.x_mm + Mm(square_size / 2.0) - Mm(15.0), square.y_mm + Mm(square_size / 2.0) - Mm(15.0), &font);
            let square = Rect::new(square.x_mm, square.y_mm, square.x_mm + Mm(square_size), square.y_mm + Mm(square_size)).with_mode(square.fill);
            layer.add_rect(square);
            ct += 1;
        }
    }
    
    //exports the pdf
    doc.save(&mut BufWriter::new(File::create("crossword.pdf").unwrap())).unwrap();
}