extern crate docopt;
#[macro_use] extern crate hyper;

use docopt::Docopt;
use std::io::Read;
use hyper::{Client};
use hyper::header::Headers;
header! { (CarriotsApikey, "Carriots.apikey") => [String] }


const USAGE: &'static str = "
Carriots Client.
Usage:
  carriots-client --apikey=<apikey> --collection=<collection> [--id_developer=<id_developer>]
  carriots-client (-h | --help)
Options:
  -h --help                         Show this screen.
  --apikey=<apikey>                 APIKEY
  --collection<collection>          COLLECTION.
  --id_developer=<id_developer>     ID DEVELOPER.
";

const HOST: &'static str = "https://api.carriots.com";

fn main() {
    let args = Docopt::new(USAGE).and_then(|dopt| dopt.parse()).unwrap_or_else(|e| e.exit());

    let client = Client::new();

    let mut url_with_cli: String = format!("{}/{}/", HOST, args.get_str("--collection"));

    if !args.get_str("--id_developer").is_empty() {
        url_with_cli = format!("{}{}/", url_with_cli, args.get_str("--id_developer"));
    };

    let mut headers = Headers::new();
    headers.set(CarriotsApikey(args.get_str("--apikey").to_owned()));

    let mut response = match client.get(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Whoops."),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };

    println!("{}", buf);
}
