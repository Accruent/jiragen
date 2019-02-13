# JiraGen (Work in progress)

A CLI tool to generate JIRA issues and place them on a board

## Usage

```bash
jiragen init # Creates a config file + story template csv file
jiragen generate --version v6.0.0 # Generates issues from story template .csv file
```

### Command: `jiragen init`

Initializes the JiraGen config and issues template. You need to:

1. Edit & save the config file (default: `jiragen.json`) with your JIRA credentials & board information.
1. Fill the issues template .csv file (default: `jiragen-issues.csv`) with issues that you would like to generate. You should also add any additional field columns for any fields specific to your JIRA configuration.

#### Options for `jiragen init`

**`--config`** (default: `jiragen.json`)
A custom path to the config file.

### Command: `jiragen generate`

Uses JIRA’s REST API to generate the issues detailed in the issues template file, into your JIRA project.

## Configuration

**`jiraUser`**
The JIRA user to login as.

**`jiraPassword`**
The JIRA user’s password. (The tool uses Basic Auth).

**`issuesPath`** (default: `jiragen-issues.csv`)
The file path of the issues CSV template.

**`versionField`** (default: `fixVersion`)
The field used in your JIRA for version tracking.
