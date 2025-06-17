# Go client library for the OpenCore


## Installation

```bash
go get github.com/openiap/client-rustapi/go
```

## Build

```
export LD_LIBRARY_PATH=$(pwd)/target/lib:$LD_LIBRARY_PATH
(cd go && go build -o cli ./cmd/cli)
(cd go && ./cli)
```

```
CGO_ENABLED=1 go build ./cmd/openiap
export CGO_ENABLED=1
export LD_LIBRARY_PATH=./lib:$LD_LIBRARY_PATH
```
on nixos, also add
```
mkdir -p ~/go/bin
ln -sf "$(command -v dlv)" ~/go/bin/dlv
```