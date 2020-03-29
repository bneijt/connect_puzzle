extern crate cairo;
extern crate rand_chacha;
use cairo::{Context, FontSlant, FontWeight, Format, ImageSurface};
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::f64::consts::PI;
use std::fs::File;
use std::{fs, io, path};

fn image_pairs_from(path: &path::Path) -> Vec<(String, String)> {
    let pieces_directory = fs::read_dir(path).expect("given path must exist");
    let mut pieces: Vec<(String, String)> = Vec::new();
    for entry in pieces_directory {
        let entry_path = entry.unwrap().path();
        let piece_path = entry_path.as_path().to_str().unwrap();
        if piece_path.ends_with("_fst.png") {
            pieces.push((
                piece_path.into(),
                piece_path.replace("_fst.png", "_snd.png").into(),
            ));
        }
        // let piece_path: path::Path = puzzle_dir.as_path();
        // println!("Loading puzzle {:?}", puzzle_path.file_name().unwrap());
    }
    return pieces;
}

fn main() {
    let (a4_width, a4_height) = (595.0, 842.0);
    let n_dots = 16;
    let margin = 60.0;
    let box_margin = 10.0;
    let seed = 11;
    let (box_width, box_height) = (
        (a4_width - 2.0 * margin) / 4.0,
        (a4_height - 2.0 * margin) / 4.0,
    );
    let mut box_top_lefts: Vec<_> = (0..n_dots)
        .map(|idx| (idx % 4, idx / 4))
        .map(|(x, y)| (x as f64, y as f64))
        .map(|(x, y)| (margin + x * box_width, margin + y * box_height))
        .collect();

    let mut rng = rand_chacha::ChaChaRng::seed_from_u64(seed);

    box_top_lefts.shuffle(&mut rng);
    let box_pairs: Vec<_> = box_top_lefts[..8]
        .iter()
        .zip(box_top_lefts[8..].iter())
        .collect();

    println!("{:?}", box_pairs);
    let dot_locations: Vec<_> = box_top_lefts
        .iter()
        .map(|(x, y)| (x + box_width / 2.0, y + box_height))
        .collect();

    let dot_pairs: Vec<_> = box_pairs
        .iter()
        .map(|((x1, y1), (x2, y2))| {
            (
                (x1 + box_width / 2.0, y1 + box_height - (0.5 * box_margin)),
                (x2 + box_width / 2.0, y2 + box_height - (0.5 * box_margin)),
            )
        })
        .collect();

    println!("{:?}", dot_pairs);

    let surface = cairo::PdfSurface::new(a4_width, a4_height, "connections.pdf")
        .expect("Can't create surface");
    let ctx = Context::new(&surface);
    ctx.select_font_face("Consolas", FontSlant::Normal, FontWeight::Normal);
    ctx.move_to(margin / 2.0, margin / 2.0);
    ctx.show_text(format!("connections: seed={}", seed).as_str());
    for ((x1, y1), (x2, y2)) in dot_pairs {
        ctx.save();
        ctx.translate(x1, y1);
        ctx.arc(0.0, 0.0, 5.0, 0.0, 2.0 * PI);
        ctx.fill();
        ctx.restore();

        ctx.save();
        ctx.translate(x2, y2);
        ctx.arc(0.0, 0.0, 5.0, 0.0, 2.0 * PI);
        ctx.fill();
        ctx.restore();

        ctx.move_to(x1, y1);
        ctx.line_to(x2, y2);
        ctx.stroke();
    }

    let puzzles = fs::read_dir("puzzles").expect("require puzzles directory");
    for entry in puzzles {
        let puzzle_dir = entry.unwrap().path();
        let puzzle_path: &path::Path = puzzle_dir.as_path();
        let puzzle_name = puzzle_path.file_name().unwrap().to_str().unwrap();
        println!("Loading puzzle {:?}", puzzle_name);
        let surface =
            cairo::PdfSurface::new(a4_width, a4_height, String::from(puzzle_name) + ".pdf")
                .expect("Can't create surface");

        let ctx = Context::new(&surface);
        ctx.select_font_face("Consolas", FontSlant::Normal, FontWeight::Normal);
        ctx.move_to(margin / 2.0, margin / 2.0);
        ctx.show_text(format!("{}: seed={}", puzzle_name, seed).as_str());

        let image_pairs = image_pairs_from(puzzle_path);
        let images_and_dots = box_pairs.iter().zip(image_pairs);
        for entries in images_and_dots {
            let (((x1, y1), (x2, y2)), _) = entries;
            let (_, (image_a, image_b)) = entries;

            let place_image = |image_path: &String, left_top_x: f64, left_top_y: f64| {
                let mut image_file = File::open(image_path).expect("Could not open file");
                let image_surface =
                    ImageSurface::create_from_png(&mut image_file).expect("Readable png file");
                ctx.save();
                ctx.translate(left_top_x + box_margin, left_top_y + box_margin);
                let scale = ((box_width - 2.0 * box_margin) / image_surface.get_width() as f64)
                    .min((box_height - 2.0 * box_margin) / image_surface.get_height() as f64);
                ctx.scale(scale, scale);
                ctx.set_source_surface(&image_surface, 0.0, 0.0);
                ctx.paint();
                ctx.restore();
            };
            place_image(&image_a, *x1, *y1);
            place_image(&image_b, *x2, *y2);
        }
    }
}
