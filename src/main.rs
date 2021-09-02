
use structopt::StructOpt;
/// Search for a patter in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
  /// The new component Name
  component: String,
  /// The path to the file to read
  #[structopt(short = "p", long = "presenter")]
  presenter: String,
  /// Regions to activate components for
  #[structopt(short = "r", long = "regions")]
  regions_for_presenter: Vec<String>,
  #[structopt(short = "v", long = "regions-view")]
  regions_for_view: Vec<String>,
}

static YES: &str = "Y";
static PRESENTER: &str = "Presenter";

fn main() {
  let args = Cli::from_args();
  let component = args.component;
  let html_extention = ".html";
  let unit_test_extension = ".test.html";
  println!("creating new component {}", component);
  if String::from(&args.presenter).eq(&YES) && args.regions_for_presenter.len() > 0 {
    println!("creating presenter for {}", component);
    for mut region in args.regions_for_presenter {
      /// Converts Edge Case for "MX" to "Mexico" otherwise Uppercases Region
      // TODO: add fn to handle this for both Presenter and View
      if String::from(region.clone()).eq("mx") {
        region = "Mexico".to_string();
      } else {
        region = region.to_uppercase();
      }
      let mut presenter_file_name = component.clone();
      presenter_file_name += &PRESENTER;
      presenter_file_name += &region;
      presenter_file_name += &html_extention;
      println!("writing {}", presenter_file_name);
      // TODO: have write_file create correct file stub with correct names
      // TODO: have write_file create correct file stub for unit tests
    }
  } else if String::from(&args.presenter).eq(&YES) {
    let mut presenter_file_name = component.clone();
    presenter_file_name += &PRESENTER;
    presenter_file_name += &html_extention;
    println!("writing {}", presenter_file_name);
  }
  let mut view_file_name = component.clone();
  view_file_name  += "View";
  if args.regions_for_view.len() > 0 {
    for region in args.regions_for_view {
      view_file_name += &region;
      view_file_name += &html_extention;
      println!("writing view for {}", region);
      // TODO: have write_file create correct file stub with correct names
      // TODO: have write_file create unit test stub
    }
  } else {
    // Creates a file for the View
    view_file_name  += &html_extention;
    println!("writing {}", view_file_name);
    // TODO: have write_file create correct file stub with correct names
    // TODO: have write_file create unit test stub
  }
}
