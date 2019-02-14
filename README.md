# JiraGen (Work in progress)

A CLI tool to generate JIRA issues and place them on a board.

## Usage

```bash
jiragen init # Creates a config file + story template csv file
jiragen generate --version v6.0.0 # Generates issues from story template .csv file
```

### Command: `jiragen init`

Initializes the JiraGen config and issues template. You need to:

1. Edit & save the config file (default: `"./jiragen.yaml"`) with your JIRA credentials & board information.
1. Fill the issues template .csv file (default: `"./jiragen-issues.csv"`) with issues that you would like to generate. You should also add any additional field columns for any fields specific to your JIRA configuration.

```bash
jiragen init
#=> creates jiragen.yaml and jiragen-issues.csv in current folder

jiragen init --config ./config/
#=> creates ./config/jiragen.yaml and ./config/jiragen-issues.csv

jiragen init --config ./config/my-custom-jiragen-config.yaml
#=> creates ./config/my-custom-jiragen-config.yaml and ./config/jiragen-issues.csv
```

#### Options for `jiragen init`

**`--config`** (default: `"./jiragen.yaml"`)
A custom path to the config file.

### Command: `jiragen generate`

Generate the issues from the issues template file into the JIRA project.

#### Options for `jiragen generate`

**`--config`** (default: `"./jiragen.yaml"`)
The path to the custom config file, if one was set.

## Configuration

**`jira_url`** (string)
The URL of the Jira instance.

**`jira_user`** (string)
The JIRA user to login as.

**`jira_password`** (string)
The JIRA user’s password. (The tool uses Basic Auth).

**`issues_file_path`** (string, default: `"./jiragen_issues.csv"`)
The file path of the issues CSV template.

## Todos

- automatic documentation generation - use a json or js export containing our default config strings, move README content to a template file.
- github badges?
