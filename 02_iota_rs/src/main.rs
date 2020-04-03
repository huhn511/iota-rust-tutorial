use iota_lib_rs::prelude::*;

fn main() {
  let mut iota = iota_client::Client::new("https://nodes.thetangle.org:443");
  println!("{:#?}", iota.get_node_info().unwrap());
}