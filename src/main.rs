use clap::Parser;
use plotters::prelude::*;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Kate Feng",
    about = "A data visualization tool"
)]
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
        #[clap(short, long)]
        xlabel: String,
        #[clap(short, long)]
        ylabel: String,
    },
}

fn main() {
    let xy = datavisualizer::read_x_y();
    let minmax = datavisualizer::read_min_max();
    let args = Cli::parse();
    match args.command {
        Some(Commands::Plot {
            filename,
            caption,
            xlabel,
            ylabel,
        }) => {
            drawline(xy, &filename, &caption, minmax, &xlabel, &ylabel);
        }
        None => println!("No subcommand was used"),
    }
}

fn drawline(
    xy: Vec<(i32, i32)>,
    file_name: &str,
    caption: &str,
    minmax: Vec<i32>,
    xlabel: &str,
    ylabel: &str,
) {
    let root_area = BitMapBackend::new(file_name, (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(minmax[0]..minmax[1], minmax[2]..minmax[3])
        .unwrap();

    ctx.configure_mesh()
        .x_desc(xlabel)
        .y_desc(ylabel)
        .axis_desc_style(("sans-serif", 20))
        .draw()
        .unwrap();

    ctx.draw_series(LineSeries::new(xy, RED)).unwrap();
}
