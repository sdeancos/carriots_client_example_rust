# Carriots Client.

## Install

cargo install --git https://github.com/sdeancos/carriots_client_example_rust.git


## Usage

- Use read for get data.
- Use write for create or update data.
- Use remove for delete data.


```shell
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
```

You can define your apikey in $HOME/.carriots_apikey

```shell
$ echo -n YOUR_APIKEY > $HOME/.carriots_apikey
$ cat $HOME/.carriots_apikey
YOUR_APIKEY
```