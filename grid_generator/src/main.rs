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
    follows_rules: bool,
}

#[derive(Debug, Clone)]
struct Column {
    x: i32,
    vec: Vec<Square>,
    follows_rules: bool,
}

#[derive(Debug, Clone)]
struct Grid{
    size: i32,
    width: f32,
    rows: Vec<Row>,
    columns: Vec<Column>,
    follows_rules: bool,
}

impl Grid {

    fn new(size: i32) -> Grid {
        Grid {
            size,
            width : 200.0 / size as f32,
            rows: Vec::<Row>::new(),
            columns: Vec::<Column>::new(),
            follows_rules: false,
        }
    }

    fn generate(&mut self) {
        let mut x_pos: i32 = 0; 
        let mut y_pos: i32 = 0;
        let start_x = Mm(5.0);
        let start_y = Mm(292.0 - self.width);
        let mut y_mm = start_y;
        let mut rng = thread_rng();
        
        while y_pos < (self.size / 2) {
            x_pos = 0;
            // defines the vector that will represent a specific horizontal row
            let mut squares = Row {
                y: y_pos,
                vec: Vec::<Square>::new(),
                follows_rules: false,
            };
            while x_pos < self.size {
                // decides if square will be blacked out
                let num = rng.gen_range(0..6);
                let mut mode: PaintMode = PaintMode::Stroke;
                if num == 0 {
                    mode = PaintMode::Fill;
                }
            
                let x_mm = start_x + Mm(x_pos as f32) * self.width;
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
            self.rows.push(squares);
            y_mm -= Mm(self.width);
            y_pos += 1;
        }

        let mut x_pos = 0;
        let y_pos = (self.size - 1) / 2;
        let mut middle_row = Row {
            y: y_pos,
            vec: Vec::<Square>::new(),
            follows_rules: false,
        };

        while x_pos < ((self.size / 2) + 1) {
            let num = rng.gen_range(0..6);
            let mut mode: PaintMode = PaintMode::Stroke;
            if num == 0 {
                mode = PaintMode::Fill;
            }
            // defines a new square object
            let square = Square {
                fill: mode,                
                x_mm: Mm(5.0) + Mm(self.width * (x_pos) as f32),
                y_mm: Mm(292.0 - self.width) - Mm(self.width * y_pos as f32),
                x_pos,
                y_pos,
            };
            middle_row.vec.push(square);
            x_pos += 1;
        }

        self.rows.push(middle_row);


    }

    fn mirror(&mut self) {
        
        let mut middle_row: Row = self.rows.pop().unwrap();
        let mut middle_row_rev = middle_row.vec.clone();
        middle_row_rev.remove(middle_row_rev.len() - 1);

        let mut x_pos: i32 = (self.size + 1) / 2;

        while x_pos < self.size  {
                let reference_square = &middle_row_rev[(self.size - x_pos - 1) as usize];
                let square = Square {
                    fill: reference_square.fill,
                    x_mm: Mm(205.0 - ((self.size - x_pos) as f32 * self.width)),
                    y_mm: reference_square.y_mm,
                    x_pos: self.size - reference_square.x_pos - 1,
                    y_pos: reference_square.y_pos,
                };

                middle_row.vec.push(square);
                x_pos += 1;
        }

        self.rows.push(middle_row);

        let mut y_pos = self.size - 1;
        let mut y_mm = Mm(292.0 - (self.size as f32 * self.width));
        //creates a reverse of 'rows'
        //the vector is reversed as well as each vector held within the broader vector
        let mut rows_rev = Vec::new();
        for row in &self.rows {
            let mut row_rev = row.clone();
            row_rev.vec.reverse();
            rows_rev.push(row_rev);
        }
        rows_rev.remove(rows_rev.len() - 1);
        // essentially the same loop as above, doing the same thing for the grid that makes up the lower half
        while y_pos >= ((self.size / 2) + 1) {
            let mut x_pos = 0;
            let mut squares = Row {
                y: y_pos,
                vec: Vec::<Square>::new(),
                follows_rules: false,
            };
            while x_pos < self.size {
                let reference_square = &rows_rev[(self.size - y_pos - 1) as usize].vec[(x_pos) as usize];
                let square = Square {
                    fill: reference_square.fill,
                    x_mm: Mm(205.0 - ((self.size - x_pos) as f32 * self.width)),
                    y_mm,
                    x_pos,
                    y_pos,
                };

                squares.vec.push(square);
                x_pos += 1;
            }
            self.rows.push(squares);
            y_mm += Mm(self.width);
            y_pos -= 1;
        }
    }

    fn draw_grid(&self, pdf: PdfDocumentReference, layer: PdfLayerReference, font: IndirectFontRef) {
        let mut ct = 1;
        //for squares in self.rows.clone() {
        for squares in self.columns.clone() {
            //println!("{:#?}", squares);
            for square in squares.vec {
                //println!("{} ({}, {})", ct, square.x_pos, square.y_pos);
                let text = format!("{}, {}", square.x_pos.to_string(), square.y_pos.to_string());
                //layer.use_text(text, 40.0, square.x_mm + Mm(self.width / 2.0) - Mm(15.0), square.y_mm + Mm(self.width / 2.0) - Mm(15.0), &font);
                let square = Rect::new(square.x_mm, square.y_mm, square.x_mm + Mm(self.width), square.y_mm + Mm(self.width)).with_mode(square.fill);
                layer.add_rect(square);
                ct += 1;
            }
        }
        
        //exports the pdf
        pdf.save(&mut BufWriter::new(File::create("crossword.pdf").unwrap())).unwrap();
    }

    fn columns(&mut self) {
        let mut columns: Vec<Column> = Vec::new();
        for column_index in 0..self.size {
            let mut column_vec: Vec<Square> = Vec::new();
            for row in &self.rows {
                if let Some(square) = row.vec.get(column_index as usize) {
                    column_vec.push(*square);
                }
            }
            let column = Column {
                x: column_index,
                vec: column_vec,
                follows_rules: false,
            };
            columns.push(column);
        }
        self.columns = columns;
    }

    fn decide_fill(&mut self) {
        let max_black_ct = (self.size * self.size) / 6;
        let mut black_ct: i32 = 0;


        for row in self.rows.clone() {
            for square in row.vec {
                todo!()
            }
        }



        // while !self.follows_rules {
        //     for row in self.rows.clone() {
        //         let mut words: Vec<i32> = Vec::new();
        //         while !row.follows_rules {
                    
        //         }
        //     }

        //     for column in self.columns.clone() {
        //         while !column.follows_rules {

        //         }
        //     }
        // }

    }

}

fn main() {
    // creates a pdf with a plain border 
    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open(String::from("../ARIAL.TTF")).unwrap()).unwrap();
    let outline = Rect::new(Mm(5.0), Mm(5.0), Mm(205.0), Mm(292.0)).with_mode(PaintMode::Stroke);
    layer.add_rect(outline);
    // define square_ct -> all other variables are dependent on this variable
    let size: i32 = 11;
    // because crossword follows symmetry, square_ct must be odd so that there is a 'middle'
    if size % 2 == 0 || size < 5 {
        println!("# of squares must be an odd whole number larger than 5");
        exit(0);
    }

    let mut grid= Grid::new(size);

    grid.generate();
    grid.mirror();
    grid.columns();
    grid.draw_grid(doc, layer, font);
}