# Vampire

Upload secrets/parameters from various sources to various destinations.

## Overview

This tool is in very early stage, I'm planning to support more sources/destinations in the future. Use at your own risk.
Make sure to export your AWS profile before executing the tool.

### Supported Integrations

#### Sources

- .env files

#### Destinations

- AWS SSM Parameter Store

## Getting Started

### Installation

```bash
git clone git@github.com:refl3ction/vampire.git
cd vampire
make install # Requires rust toolchain to be installed for now
```

### Usage

#### Basic

```bash
Usage: vampire [OPTIONS] --file <FILE> --region <REGION>

Options:
  -f, --file <FILE>
  -p, --prefix <PREFIX>  [default: ]
  -r, --region <REGION>
  -o, --overwrite
  -h, --help             Print help information
  -V, --version          Print version information
```

#### .env to SSM

```bash
export AWS_PROFILE=my-profile

cat .env
FOO=bar
BAR=baz
BAZ=foobar

vampire --file .env --prefix /dev/app/ --region us-east-1 --overwrite

# output
File: .env
Prefix: /dev/app/
Region: us-east-1
Overwrite: true
The following variables will be created:
/dev/app/FOO=***
/dev/app/BAR=***
/dev/app/BAZ=***
Would you like to proceed? (n/y)
y
[ok] parameter updated | Name: /dev/app/FOO | version: 5
[ok] parameter updated | Name: /dev/app/BAR | version: 5
[ok] parameter created | Name: /dev/app/BAZ | version: 1
```

### Roadmap

- [ ] Add tests
- [ ] Build binary using Github Actions
- [ ] Add more sources
  - [ ] k8s secrets (yaml manifest)
  - [ ] Yaml
- [ ] Add more destinations
  - [ ] AWS Secrets Manager
