# Carriots Client.

## Install

cargo install --git https://github.com/sdeancos/carriots_client_example_rust.git


## Examples

```shell
carriots-client --set_apikey=YOUR_APIKEY_HERE

carriots-client read --collection=streams --filters="_t=sta&max=10"
carriots-client read --collection=devices --filters="name=defaultDevice"
carriots-client read --collection=devices --id_developer="defaultDevice@FAKE"

carriots-client remove --collection=group --id_developer="MyGroup@Testing.Testing"

carriots-client write --collection=group --data_content="$(cat data.json)"
carriots-client write --collection=group --id_developer="MyAwesomeGroup@Testing.Testing" --data="$(cat data.json)"
```

## More Examples

```shell
# Send memory info to Carriots platform.
data=$(cat /proc/meminfo | sed 's/://g' | awk '{ print "{\"" $1 "\"" ":\"" $2 "\"}," }');
carriots-client write --collection=streams --data_content="{\"device\":\"YOUR_DEVICE\",\"protocol\":\"v2\",\"at\":\"now\",\"data\":[$(echo ${data::-1})]}"
```


## Usage

- Use 'read' for get data.
- Use 'write' for create or update data:
    - data_content must be a json string valid.
- Use 'remove' for delete data.


```shell
Carriots Client.
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
```

You can define your apikey in $HOME/.carriots_apikey (or with client)

```shell
$ carriots-client --set_apikey=YOUR_APIKEY
$ cat $HOME/.carriots_apikey
YOUR_APIKEY
or
$ echo -n YOUR_APIKEY > $HOME/.carriots_apikey
$ cat $HOME/.carriots_apikey
YOUR_APIKEY
```
