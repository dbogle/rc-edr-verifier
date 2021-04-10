# rc-edr-verifier

# Usage
This tool supports performing various actions on the system. The supported actions it can perform are
* Process creation
* File creation
* File modification
* File deletion
* File reading
* Sending data on the network

## Command line
This tool can be run from the command line. Run the tools --help comand to see verbose output on command line use

## Command file
This tool also accepts a json file of commands that it can execute. There can be 1 or more commands specified in a single json array. The formats and accepted arguments are listed below 

# Command types
## Process creation
```
{
    "type": "process",
    "args": {
        "file_path": "/some/file/path",
        "args": [
            "arg 1",
            "arg 2",
        ]
    }
}
```

## Network traffic
```
{
    "type": "network",
    "args": {
        "host: "127.0.0.1",
        "port: 8080,
        "protocol": "tcp",
        "data": "ZHVycGR1cnBkdXJw",
        "b64_encoded": true
    }
}
```
## File creation/modification
### Create
```
{
    "type": "file",
    "file_path": "/some/file/path.txt",
    "args": {
        "command": "create",
        "data": "ZHVycGR1cnBkdXJw",
        "b64_encoded": true
    }
}
```
### Read
```
{
    "type": "file",
    "file_path": "/some/file/path",
    "args": {
        "command": "read",
        "offset": 0,
        "num_bytes": 100
    }
}
```
### Write
```
{
    "type": "file",
    "args": {
        "file_path": "/some/file/path.jpg",
        "command": write,
        "start": 50,
        "data": "Some data to write to the file",
        "b64_encoded": false
    }
}
```

### Delete
```
{
    "type": "file",
    "file_path": "/some/file/path.json",
    "args": {
        "command": "delete"
    }
}
```