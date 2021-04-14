# rc-edr-verifier

## RC
This tool supports performing various actions on the system. The supported actions it can perform are
* Process creation
* File creation
* File modification
* File deletion
* File reading
* Sending data on the network

It has been tested on mac and linux but not windows although it should work on it. For each activity that it runs it generates a json object that is written to tasks.log.

## Building
In order to build the tool it is assumed you already have rust installed on your machine. If not please go to their site and install rustup
To build the project simply run `cargo build` for debug mode or `cargo build --release` for a release build

### Running from command line
This tool can be run from the command line. Run the tools --help comand to see verbose output on command line use. The general usage is 
`rc [FLAGS] [OPTIONS] [SUBCOMMAND]` where subcommands are the specific command you want to run. See examples below for how to run the tool
```
rc network 127.0.0.1 8080 tcp "My data to send"
rc createfile ./somefile.txt "Data in the file"
rc process /bin/ls -- -la /tmp
rc readfile ./somefile.txt -o 20 100
```

### Command file
This tool also accepts a json file of commands that it can execute. There can be 1 or more commands specified in a single json array. The formats and accepted arguments are listed below. There is an file at examples/cmds.json that shows what that might look like

## Command types
### Process creation
```
{
    "type": "process",
    "args": {
        "filepath": "/some/file/path",
        "args": ["arg 1", "arg 2", ...]
    }
}
```

### Network traffic
```
{
    "type": "network",
    "args": {
        "host": "127.0.0.1",
        "port": 8080,
        "protocol": "tcp",
        "data": "This is some test data"
    }
}
```
### File creation/modification
#### Create
```
{
    "type": "file",
    "args": {
        "filepath": "/some/file/path.txt",
        "command": "create",
        "data": "AAAAAAAAAAAAAAAAA"
    }
}
```
#### Read
```
{
    "type": "file",
    "args": {
        "filepath": "/some/file/path",
        "command": "read",
        "offset": 0,
        "num_bytes": 100
    }
}
```
#### Write
```
{
    "type": "file",
    "args": {
        "filepath": "/some/file/path.jpg",
        "command": write,
        "start": 50,
        "data": "Some data to write to the file"
    }
}
```

#### Delete
```
{
    "type": "file",
    "args": {
        "filepath": "/some/file/path.json",
        "command": "delete"
    }
}
```