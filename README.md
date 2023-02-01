# Project 1 - Data Visualizer
My project 1 is a Rust command line tool, which is a data visualizer to help users draw plots. It's based on the crate called Plotters.

## How to use
Currently this tool supports configuration of setting the filename to save the plot, the caption of the plot, and x/y min/max values.
For example, run this `cargo run -- plot --filename demo.png --caption "demo plot" --x-min 0 --x-max 100 --y-min 0 --y-max 80` to try it out!
I am still working on to parse x and y values to the command line, so currently the x and y values are hardcoded. It's basically drawing a line from (0,0) to (100,100). I am thinking of using a file to store the x and y values and import the file. 

## Future Work
* Finish parsing x and y values to the command line
* Deploy on AWS
* Add more fields for users to customize

## References
* https://plotters-rs.github.io/book/basic/basic_data_plotting.html
* https://docs.rs/plotters/latest/plotters/#quick-start
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
