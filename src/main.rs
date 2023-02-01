use plotters::prelude::*;
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Kate Feng", about = "A data visualization tool")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Kate Feng")]
    Plot {
        #[clap(short, long)]
        filename: String,
        #[clap(short, long)]
        caption: String,
        #[clap(long)]
        x_min: i32,
        #[clap(long)]
        x_max: i32,
        #[clap(long)]
        y_min: i32,
        #[clap(long)]
        y_max: i32,
    },
}


fn main() {
    let x = vec![0,100];
    let y = vec![0,100];

    let args = Cli::parse();
    match args.command {
        Some(Commands::Plot {
            filename,
            caption,
            x_min,
            x_max,
            y_min,
            y_max,
        }) => {
            drawline(x, y, &filename, &caption, x_min, x_max, y_min, y_max);
         
        }
        None => println!("No subcommand was used"),
    }

}

fn combine_vectors(a: Vec<i32>, b: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (a_element, b_element) in a.iter().zip(b.iter()) {
        result.push((*a_element, *b_element));
    }
    result
}

fn drawline(x: Vec<i32>, y: Vec<i32>, file_name: &str, caption: &str, x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
  let root_area = BitMapBackend::new(file_name, (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption(caption, ("sans-serif", 40))
    .build_cartesian_2d(x_min..x_max, y_min..y_max)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  ctx.draw_series(
    LineSeries::new(combine_vectors(x,y), &RED)
  ).unwrap();
}