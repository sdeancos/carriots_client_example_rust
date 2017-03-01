extern crate docopt;
#[macro_use] extern crate hyper;

use docopt::Docopt;
use std::io::Read;
use std::fs::File;
use std::env;
use std::path::Path;
use hyper::{Client};
use hyper::header::Headers;
header! { (CarriotsApikey, "Carriots.apikey") => [String] }


const USAGE: &'static str = "
Carriots Client.
Usage:
  client-carriots read [--apikey=<apikey>] --collection=<collection> [--id_developer=<id_developer>]
  client-carriots write [--apikey=<apikey>] --collection=<collection> --data_content=<data_content> [--id_developer=<id_developer>]
  client-carriots remove [--apikey=<apikey>] --collection=<collection> --id_developer=<id_developer>
  client-carriots (-h | --help)
Options:
  -h --help
  --apikey=<apikey>
  --collection<collection>
  --id_developer=<id_developer>
  --data_content=<data_content>
";

const HOST: &'static str = "https://api.carriots.com";

fn read(client: Client, url_with_cli: String, headers: Headers) -> String {
    let mut response = match client.get(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn remove(client: Client, url_with_cli: String, headers: Headers) -> String {
    let mut response = match client.delete(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn write_post(client: Client, url_with_cli: String, headers: Headers, data_content: String) -> String {
    let mut response = match client.post(&url_with_cli).headers(headers).body(&data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn write_put(client: Client, url_with_cli: String, headers: Headers, data_content: String) -> String {
    let mut response = match client.put(&url_with_cli).headers(headers).body(&data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn read_carriots_apikey_file() -> String {
    let mut home = String::new();

    match env::home_dir() {
        Some(path) => home = format!("{}/", path.display()),
        None => home = format!("{}", ""),
    }

    let file_path = format!("{}{}", home, ".carriots_apikey");

    let path = Path::new(&file_path);

    if path.exists() {
        let mut file = File::open(&path).unwrap();
        let mut string_content = String::new();

        file.read_to_string(&mut string_content);

        return string_content;
    };

    return format!("{}", "");
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let client = Client::new();

    let mut url_with_cli: String = format!("{}/{}/", HOST, args.get_str("--collection"));

    if !args.get_str("--id_developer").is_empty() {
        url_with_cli = format!("{}{}/", url_with_cli, args.get_str("--id_developer"));
    };

    let mut headers = Headers::new();

    let apikey_from_file = read_carriots_apikey_file();

    if args.get_str("--apikey").is_empty() && !apikey_from_file.is_empty() {
        headers.set(CarriotsApikey(apikey_from_file.to_owned()));
    } else {
        headers.set(CarriotsApikey(args.get_str("--apikey").to_owned()));
    }

    let buf;
    if args.get_bool("read") {
        buf = read(client, url_with_cli, headers);
    } else if args.get_bool("write") {
        let data_content: String = format!("{}", args.get_str("--data_content"));
        if args.get_str("--id_developer").is_empty()  {
            buf = write_post(client, url_with_cli, headers, data_content);
        } else {
            buf = write_put(client, url_with_cli, headers, data_content);
        }
    } else if args.get_bool("remove") {
        buf = remove(client, url_with_cli, headers);
    } else {
        buf = format!("{}","command not found")
    }

    println!("{}", buf);
}
