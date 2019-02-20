# JiraGen (Work in progress)

A CLI tool to generate JIRA issues and place them on a board. This makes it easy to create issue "templates" for repeatable processes (such as releasing a new software version) and track that in JIRA. Uses [version 2 of JIRA’s REST API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/).

## Installation

Download the binary (located in the releases section of the GitHub repo) and run it on the command line.

## Usage

```bash
jiragen init # Creates a config file + issues template csv file
jiragen generate # Generates the issues template .csv file
jiragen push # Reads the issues template .csv file to create corresponding issues in JIRA
```

1. Run `jiragen init`. This creates a config file (default: `"./jiragen.yaml"`).
1. Edit the config file with your JIRA credentials and save the file.
1. Run `jiragen generate`. This creates the issues template .csv file (default: `"./jiragen-issues.csv"`) with prefilled column ids & names from your JIRA configuration.
1. Edit the issues template .csv file with the issues you would like to generate. Feel free to remove any field columns that are not necessary for issue creation.
1. Run `jiragen push`. This reads the data in the file and creates the corresponding issues in JIRA.

## Commands

### Command: `jiragen init`

Creates the JiraGen config file.

```bash
jiragen init
#=> creates jiragen.yaml

jiragen init --config ./config/my-custom-jiragen-config.yaml
#=> creates "./config/my-custom-jiragen-config.yaml"
```

#### Options for `jiragen init`

**`--config`** (default: `"./jiragen.yaml"`)
A custom path where the config file is created.

**`--issues`** (default: `"./jiragen-issues.csv"`)
A custom path where the issues template CSV file is created.

<!-- ### Command: `jiragen generate`

Creates the issues template .csv file.

```bash
jiragen generate
# => creates jiragen-issues.csv in current folder

jiragen generate --config ./config/my-custom-config.yaml --issues ./config/my-issues-template.csv
# => reads the config file located at "./config/my-custom-config.yaml" and creates "./config/my-issues-template.csv"
```

#### Options for `jiragen generate`

**`--config`** (default: `"./jiragen.yaml"`)
The path to the custom config file, if one was set.

**`--issues`** (default: `"./jiragen-issues.csv"`)
A custom path where the issues template CSV file is created. -->

### Command: `jiragen push`

Takes the content from the issues template file and creates the issues in the JIRA project.

```bash
jiragen push
#=> reads jiragen-issues.csv in the current folder and pushes issues to JIRA

jiragen push --config ./config/my-custom-jiragen-config.yaml --issues ./config/my-issues-template.csv
#=> reads the files located at "./config/my-custom-jiragen-config.yaml" and "./config/my-issues-template.csv" and pushes issues to JIRA
```

#### Options for `jiragen push`

**`--config`** (default: `"./jiragen.yaml"`)
The path to the custom config file, if one was set.

**`--issues`** (default: `"./jiragen-issues.csv"`)
A custom path to the issues template CSV file, if one was set.

## Configuration

**`jira_url`** (string)
The URL of the Jira instance.

**`jira_user`** (string)
The JIRA user to login as.

**`jira_password`** (string)
The JIRA user’s password. (The tool uses Basic Auth).

**`issues_schema`** (object)
A JSON object describing the shape of the issue data that is sent to JIRA (See [JIRA’s API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/#api-api-2-issue-bulk-post) for an example of the type of data that is sent to JIRA’s API).

### The Issues Template file

The first line of the generated .csv file (from the `generate` command) contains the field ids, and the second line contains the field names. Actual issue content should be entered starting on line 3.

JiraGen will read from this file in order to generate the corresponding issues in JIRA (using the `push` command).

### .csv syntax

Only the first row of the .csv file (the field ids) is used when parsing the .csv file to create the corresponding issues in JIRA; The second row (the field names) is ignored.

JIRA’s API requires that the data sent is shaped in specific ways. A user field must be sent as an object with a "name" key, for example. See [JIRA’s API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/#api-api-2-issue-bulk-post) for a more complete example.

In order to follow this formatting within our CSV file, the first row (ids) uses `[]` and `.` to describe an array or object property, respectively:

```bash
# first row = csv id field/key, second row = value of that id

summary
A Test Summary
# { "summary": "A Test Summary" }

labels[]
a-label
# { "labels": ["a-label"] }

issuetype.id
12345
# { "issuetype": {"id": "12345"} }

components[].id
A Component
# { "components": [ {"id": "A Component"} ] }

watcher.watchers[].accountId
abcc281-qk3j8d8fj
# { "watcher": { "watchers": [{"accountId": "abcc281-qk3j8d8fj"}] } }

timetracking.originalEstimate,timetracking.remainingEstimate
10,5
# { "timetracking": { "originalEstimate": "10", "remainingEstimate": "5" } }

fixVersions[].id,fixVersions[].id
10000,10001
# { "fixVersions": [ {"id": "10000"}, {"id": "10001"} ] }

# Unimplemented: multiple properties of an object in an array (I don't think there's a need for this for bulk JIRA issue creation)
```

## Todos

- github badges?
- use jiragen_id’s to enable subtasking
- move individual functions to structs
- better error handling of unwraps
