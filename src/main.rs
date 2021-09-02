
use structopt::StructOpt;
#[derive(StructOpt)]
struct Cli {
  component: String,
  #[structopt(short = "p", long = "presenter")]
  presenter: String,
  #[structopt(short = "r", long = "regions-presenter")]
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

  // Creates regional presenters if necessary
  if String::from(&args.presenter).eq(&YES) {
    let mut presenter_file_name_base = &mut component.clone();
    create_file_names(presenter_file_name_base, args.regions_for_presenter, PRESENTER);
  }

  // Creates View Files
  let mut view_file_name_base = &mut component.clone();
  view_file_name_base.push_str(&VIEW_FILE_APPEND);

  create_file_names(view_file_name_base, args.regions_for_view, VIEW_FILE_APPEND);
}

fn create_file_names (base_name: &str, regions: Vec<String>, component_type: &str) {
  let mut new_file_name = base_name.clone().to_string();
  if regions.len() > 0 {
    for mut region in regions {
      if String::from(region.clone()).eq("mx") {
        region = "Mexico".to_string();
      } else {
        region = region.to_uppercase();
      }

      let mut region_file_name = new_file_name.clone();
      region_file_name.push_str(component_type);
      region_file_name.push_str(&region);
      region_file_name.push_str(HTML_EXTENSION);

      let mut region_unit_test_file_name = region_file_name.clone();
      region_unit_test_file_name.push_str(UNIT_TEST_EXT);

      println!("creating {} for {}", component_type, region);
      println!("writing view unit test for {}", region_unit_test_file_name);
    }
  }
  // Creates Global Component
  new_file_name.push_str(component_type);
  new_file_name.push_str(HTML_EXTENSION);

  let mut unit_test_file_name = new_file_name.clone();
  unit_test_file_name.push_str(UNIT_TEST_EXT);

  println!("writing view unit test for {} global", unit_test_file_name);
  println!("creating {} for {} global", new_file_name, component_type);
}