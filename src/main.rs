extern crate docopt;
extern crate reqwest;
#[macro_use] extern crate hyper;

use docopt::Docopt;
use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::path::Path;

use reqwest::Client;
use reqwest::Response;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE, ACCEPT};

const NAME: &'static str = "Carriots Client";
const VER: &'static str = "0.4.0";
const USAGE: &'static str = "

Usage:
  client-carriots --get_apikey
  client-carriots --set_apikey=<apikey>
  client-carriots read [--apikey=<apikey>] --collection=<collection> [--id_developer=<id_developer>] [--filters=<filters>]
  client-carriots write [--apikey=<apikey>] --collection=<collection> --data_content=<data_content> [--id_developer=<id_developer>]
  client-carriots remove [--apikey=<apikey>] --collection=<collection> --id_developer=<id_developer>
  client-carriots (-h | --help)
Options:
  --help
  --get_apikey
  --set_apikey=<apikey>
  --apikey=<apikey>
  --collection=<collection>
  --id_developer=<id_developer>
  --data_content=<data_content>
  --filters=<filters>
";

const HOST: &'static str = "https://api.carriots.com";
const DEFAULT_USER_AGENT: &'static str = "Carriots-Client-Rust";
const DEFAULT_CONTENT_TYPE: &'static str = "application/json";

fn construct_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static(DEFAULT_USER_AGENT));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static(DEFAULT_CONTENT_TYPE));
    headers.insert(ACCEPT, HeaderValue::from_static(DEFAULT_CONTENT_TYPE));
    headers
}

fn create_response(mut response: Response) -> String {
    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("Response error."),
    };

    return buf;
}

fn read(client: Client, url_with_cli: String, headers: HeaderMap) -> String {
    let response = match client.get(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn remove(client: Client, url_with_cli: String, headers: HeaderMap) -> String {
    let response = match client.delete(&url_with_cli).headers(headers).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn write_post(client: Client, url_with_cli: String, headers: HeaderMap, data_content: String) -> String {
    let response = match client.post(&url_with_cli).headers(headers).body(data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn write_put(client: Client, url_with_cli: String, headers: HeaderMap, data_content: String) -> String {
    let response = match client.put(&url_with_cli).headers(headers).body(data_content).send() {
        Ok(response) => response,
        Err(_) => panic!("Client Error"),
    };

    return create_response(response);
}

fn write_carriots_apikey_file(apikey: String) {
    let home = match env::home_dir() {
        Some(path) => format!("{}/", path.display()),
        None => format!("{}", ""),
    };

    let file_path = format!("{}{}", home, ".carriots_apikey");
    let path = Path::new(&file_path);

    let mut file = File::create(&path).unwrap();

    let _result = file.write_all(apikey.as_bytes());
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
    let usage = format!("{} {}{}", NAME, VER, USAGE);
    let args = Docopt::new(usage)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());
    //println!("{:?}", args);


    let client_instance = Client::new();
    let response_buffer;

    if !args.get_str("--set_apikey").is_empty() {
        write_carriots_apikey_file(args.get_str("--set_apikey").to_owned());
        return ()
    }

    if args.get_bool("--get_apikey") {
        response_buffer = read_carriots_apikey_file();
        println!("{}", response_buffer);
        return ()
    }

    let mut headers = construct_default_headers();

    let apikey_from_file = read_carriots_apikey_file();
    if args.get_str("--apikey").is_empty() && !apikey_from_file.is_empty() {
        headers.insert("Apikey", HeaderValue::from_str(&apikey_from_file).unwrap());

    } else {
        headers.insert("Apikey", HeaderValue::from_str(args.get_str("--apikey")).unwrap());
    }

    let mut url_with_cli: String = format!("{}/{}/", HOST, args.get_str("--collection"));

    if !args.get_str("--id_developer").is_empty() {
        url_with_cli = format!("{}{}/", url_with_cli, args.get_str("--id_developer"));
    }

    if !args.get_str("--filters").is_empty() {
        url_with_cli = format!("{}?{}", url_with_cli, args.get_str("--filters"));;
    }

    if args.get_bool("read") {
        response_buffer = read(client_instance, url_with_cli, headers);
    } else if args.get_bool("write") {
        let data_content: String = format!("{}", args.get_str("--data_content"));
        if args.get_str("--id_developer").is_empty()  {
            response_buffer = write_post(client_instance, url_with_cli, headers, data_content);
        } else {
            response_buffer = write_put(client_instance, url_with_cli, headers, data_content);
        }
    } else if args.get_bool("remove") {
        response_buffer = remove(client_instance, url_with_cli, headers);
    } else {
        response_buffer = format!("{}","command not found");
    }

    println!("{}", response_buffer);
}
