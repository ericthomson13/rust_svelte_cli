
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
  component: String,
  #[structopt(short = "p", long = "presenter")]
  presenter: String,
  #[structopt(short = "r", long = "regions")]
  regions_for_presenter: Vec<String>,
  #[structopt(short = "v", long = "regions-view")]
  regions_for_view: Vec<String>,
}

static YES: &str = "Y";
static PRESENTER: &str = "Presenter";
static UNIT_TEST_EXT: &str = ".test.js";
static DIRECTORY_BASE: &str = "src/components/v2/";
static HTML_EXTENSION: &str = ".html";
static E2E_TEST_EXTENSION: &str = ".e2e.js";
static VIEW_FILE_APPEND: &str = "View";

use std::fs;
mod write_file;

fn main() {
  let args = Cli::from_args();
  let component = args.component;
  let mut directory_to_write = DIRECTORY_BASE.clone().to_owned();
  directory_to_write.push_str(&component);
  println!("creating new component {}, in {}", component, directory_to_write);
  fs::create_dir_all(directory_to_write).expect("directory failed to write");

  if String::from(&args.presenter).eq(&YES) && args.regions_for_presenter.len() > 0 {
    println!("creating presenter for {}", component);
    // Creates any region specific presenters
    for mut region in args.regions_for_presenter {
      // Converts Edge Case for "MX" to "Mexico" otherwise Uppercases Region
      // TODO: add fn to handle this for both Presenter and View
      if String::from(region.clone()).eq("mx") {
        region = "Mexico".to_string();
      } else {
        region = region.to_uppercase();
      }
      let mut presenter_file_name = component.clone();
      presenter_file_name += &PRESENTER;
      presenter_file_name += &region;
      let mut presenter_unit_test_name = presenter_file_name.clone();
      presenter_file_name += &HTML_EXTENSION;

      println!("writing {}", presenter_file_name);
      let mock_file_string = Vec::<String>::new();
      // self::write_file::write_new_file(&presenter_file_name, &directory_to_write, &mock_file_string);
      // TODO: have write_file create correct file stub with correct names
      // TODO: have write_file create correct file stub for unit tests
    }

    // Creates global Presenter
    let mut global_presenter_file_name = component.clone();
    global_presenter_file_name += &PRESENTER;
    global_presenter_file_name += &HTML_EXTENSION;
    println!("writing {}", global_presenter_file_name);
    // TODO: have write_file create correct file stub with correct names
    // TODO: have write_file create correct file stub for unit tests
  } else if String::from(&args.presenter).eq(&YES) {
    let mut presenter_file_name = component.clone();
    presenter_file_name += &PRESENTER;
    presenter_file_name += &HTML_EXTENSION;
    println!("writing {}", presenter_file_name);
  }
  let mut view_file_name = &mut component.clone();
  view_file_name.push_str(&VIEW_FILE_APPEND);
  let mut unit_test_view_file_name = &mut view_file_name.clone();
  unit_test_view_file_name.push_str(HTML_EXTENSION);
  unit_test_view_file_name.push_str(UNIT_TEST_EXT);
  if args.regions_for_view.len() > 0 {
    for region in args.regions_for_view {
      view_file_name.push_str(&region);
      view_file_name.push_str(&HTML_EXTENSION);
      println!("writing view for {}", region);
      // TODO: have write_file create correct file stub with correct names
      // TODO: have write_file create unit test stub
      // unit_test_view_file_name += &region;
      // unit_test_view_file_name += &UNIT_TEST_EXT;
      println!("writing view unit test for {}", unit_test_view_file_name)
    }
  } else {
    // Creates a file for the View
    view_file_name.push_str(&HTML_EXTENSION);
    println!("writing {}", view_file_name);
    create_file_names(view_file_name.to_string(), args.regions_for_view, VIEW_FILE_APPEND);
    // TODO: have write_file create correct file stub with correct names
    // unit_test_view_file_name += &UNIT_TEST_EXT;
    // TODO: have write_file create unit test stub
    println!("writing view unit test for {}", unit_test_view_file_name);
  }
}

fn create_file_names (base_name: String, regions: Vec<String>, presenter_or_view: &str) {
  let mut component_file_name = base_name.clone();
  component_file_name.push_str(presenter_or_view.clone());
}