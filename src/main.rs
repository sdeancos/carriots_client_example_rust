extern crate docopt;
#[macro_use] extern crate hyper;

use docopt::Docopt;
use std::io::Read;
use std::fs::File;
use std::env;
use std::path::Path;
use hyper::{Client};
use hyper::client::Response;
use hyper::header::Headers;
header! { (CarriotsApikey, "Carriots.apikey") => [String] }
header! { (ContentType, "Content-type") => [String] }
header! { (Accept, "Accept") => [String] }

const USAGE: &'static str = "
Carriots Client.
Usage:
  client-carriots read [--apikey=<apikey>] --collection=<collection> [--id_developer=<id_developer>] [--filters=<filters>]
  client-carriots write [--apikey=<apikey>] --collection=<collection> --data_content=<data_content> [--id_developer=<id_developer>]
  client-carriots remove [--apikey=<apikey>] --collection=<collection> --id_developer=<id_developer>
  client-carriots (-h | --help)
Options:
  -h --help
  -a --apikey=<apikey>
  -c --collection=<collection>
  -i --id_developer=<id_developer>
  -d --data_content=<data_content>
  -f --filters=<filters>
";

const HOST: &'static str = "https://api.carriots.com";
const CONTENT_TYPE: &'static str = "application/json";

fn create_response(mut response: Response) -> String {
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn read(client: Client, url_with_cli: String, headers: Headers) -> String {
    let response = match client.get(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn remove(client: Client, url_with_cli: String, headers: Headers) -> String {
    let response = match client.delete(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn write_post(client: Client, url_with_cli: String, headers: Headers, data_content: String) -> String {
    let response = match client.post(&url_with_cli).headers(headers).body(&data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn write_put(client: Client, url_with_cli: String, headers: Headers, data_content: String) -> String {
    let response = match client.put(&url_with_cli).headers(headers).body(&data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn read_carriots_apikey_file() -> String {
    let home = match env::home_dir() {
        Some(path) => format!("{}/", path.display()),
        None => format!("{}", ""),
    };

    let file_path = format!("{}{}", home, ".carriots_apikey");

    let path = Path::new(&file_path);

    if path.exists() {
        let mut file = File::open(&path).unwrap();
        let mut string_content = String::new();

        let _result = file.read_to_string(&mut string_content);

        return string_content;
    };

    return format!("{}", "");
}

fn main() {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());

    let client = Client::new();

    let mut headers = Headers::new();
    headers.set(ContentType(String::from(CONTENT_TYPE)));
    headers.set(Accept(String::from(CONTENT_TYPE)));

    let apikey_from_file = read_carriots_apikey_file();
    if args.get_str("--apikey").is_empty() && !apikey_from_file.is_empty() {
        headers.set(CarriotsApikey(apikey_from_file.to_owned()));
    } else {
        headers.set(CarriotsApikey(args.get_str("--apikey").to_owned()));
    }

    let mut url_with_cli: String = format!("{}/{}/", HOST, args.get_str("--collection"));

    if !args.get_str("--id_developer").is_empty() {
        url_with_cli = format!("{}{}/", url_with_cli, args.get_str("--id_developer"));
    }

    if !args.get_str("--filters").is_empty() {
        url_with_cli = format!("{}?{}", url_with_cli, args.get_str("--filters"));;
    }

    let response_buffer;
    if args.get_bool("read") {
        response_buffer = read(client, url_with_cli, headers);
    } else if args.get_bool("write") {
        let data_content: String = format!("{}", args.get_str("--data_content"));
        if args.get_str("--id_developer").is_empty()  {
            response_buffer = write_post(client, url_with_cli, headers, data_content);
        } else {
            response_buffer = write_put(client, url_with_cli, headers, data_content);
        }
    } else if args.get_bool("remove") {
        response_buffer = remove(client, url_with_cli, headers);
    } else {
        response_buffer = format!("{}","command not found");
    }

    println!("{}", response_buffer);
}
