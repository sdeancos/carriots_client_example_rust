# Carriots Client.

## Install

cargo install --git https://github.com/sdeancos/carriots_client_example_rust.git


## Examples

```shell
client-carriots read --collection=streams --filters="_t=sta&max=10"
client-carriots read --collection=devices --filters="name=defaultDevice"
client-carriots read --collection=devices --id_developer="defaultDevice@FAKE"
```


## Usage

- Use read for get data.
- Use write for create or update data:
    - data_content must be a json string valid.
- Use remove for delete data.


```shell
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
```

You can define your apikey in $HOME/.carriots_apikey

```shell
$ echo -n YOUR_APIKEY > $HOME/.carriots_apikey
$ cat $HOME/.carriots_apikey
YOUR_APIKEY
```