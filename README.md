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
    type: process,
    task_args: {
        file_path: "/some/file/path",
        args: [
            "arg 1",
            "arg 2",
            ...
        ]
    }
}
```

## Network traffic
```
{
    type: network,
    task_args: {
        host: "127.0.0.1",
        port: 8080
        protocol: [tcp | udp]
        data: "ZHVycGR1cnBkdXJw"
        base64_encoded: true
    }
}
```
## File creation/modification
### Create
```
{
    type: file,
    file_path: "/some/file/path.txt",
    task_args: {
        command: create,
        data: "ZHVycGR1cnBkdXJw"
        base64_encoded: true
    }
}
```
### Read
```
{
    type: file,
    task_args: {
        file_path: "/some/file/path",
        command: read
        start: 0 // The offset to start reading at
        bytes_to_read: 100 // The maximum number of bytes to read or read until EOF
    }
}
```
### Write
```
{
    type: file,
    task_args: {
        file_path: "/some/file/path.jpg",
        command: write,
        start: 50 // The offset to start reading at,
        data: "Some data to write to the file",
        base64_encoded: false
    }
}
```

### Delete
```
{
    type: file,
    file_path: "/some/file/path.json",
    task_args: {
        command: delete,
    }
}
```