# `jiragen-cli`

The command line tool to send bulk issue creation requests to JIRA from a .csv file.

## Installation

Download the binary (located in the releases section of the GitHub repo) and run it on the command line.

## Usage

```bash
A CLI tool to generate JIRA issues and place them on a board.

USAGE:
    jiragen <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    init    Creates the JiraGen config file. [aliases: i]
    push    Pushes issues to JIRA. [aliases: p]
```

1. Run `jiragen init`. This creates a config file (default: `./jiragen.json`) and issues template file (default: `./jiragen_issues.csv`).
1. Edit the config file with your JIRA credentials and save the file.
1. Edit the issues template .csv file with the issues you would like to generate. Feel free to remove any field columns that are not necessary for issue creation. See [section about how to enter column data](../#csv-syntax).
1. Run `jiragen push`. This reads the data in the file and creates the corresponding issues in JIRA.

## Commands

### Command: `jiragen init`

Creates the JiraGen config file.

```bash
jiragen init
#=> creates jiragen.json

jiragen init --config ./config/my-custom-jiragen-config.json
#=> creates "./config/my-custom-jiragen-config.json"

jiragen init --config ./config/my-custom-jiragen-config.json  --issues ./config/my-issues-template.csv
#=> creates "./config/my-custom-jiragen-config.json" and "./config/my-issues-template.csv"
```

### Command: `jiragen push`

Takes the content from the issues template file and creates the issues in the JIRA project.

```bash
jiragen push
#=> reads jiragen-issues.csv in the current folder and pushes issues to JIRA

jiragen push --config ./config/my-custom-jiragen-config.json --issues ./config/my-issues-template.csv
#=> reads the files located at "./config/my-custom-jiragen-config.json" and "./config/my-issues-template.csv" and pushes issues to JIRA
```

### Command Options

**`--config`** (default: `"./jiragen.json"`)
A custom path where the config file is created.

**`--issues`** (default: `"./jiragen-issues.csv"`)
A custom path where the issues template CSV file is created.

## Configuration

Configuration is stored in a `.json` file (default `./jiragen.json`) and has the following properties:

**`jira_url`** (string)
The URL of the Jira instance.

**`jira_user`** (string)
The JIRA user to login as.

**`jira_password`** (string)
The JIRA user’s password. (The tool uses Basic Auth).
