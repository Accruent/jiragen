# JiraGen

A library and CLI tool to generate JIRA issues. This makes it easy to create issue "templates" for repeatable processes (such as releasing a new software version) and track that in JIRA. Uses [JIRA’s REST API v2](https://developer.atlassian.com/cloud/jira/platform/rest/v2/).

[CLI Docs](jiragen-cli/)

[Library Docs](https://docs.rs/jiragen)

## The Issues Template file

JiraGen reads a .csv file that represents the issues (and their field values) to be created in JIRA (using the `push` command).

```csv
summary,description,project.key
Summary,Description,Project
A Summary,A Description,CON
```

- The first row of the generated .csv file (from the `init` command) contains the field ids.
- The second row contains the field names. This row is ignored (it is used for human readability).
- Actual issue content should be entered starting on the third row.

## .csv syntax

Because JIRA’s API requires that the issues’ fields be shaped to specific schemas (See [JIRA’s API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/#api-api-2-issue-bulk-post) for an example), we translate that schema to the .csv file: `[]` and `.` describe an array or object property, respectively. Remember that the second row of the .csv file is ignored.

Some examples of how data is converted from the .csv file to JSON:

```bash
# first row = csv id field/key, second row = readable field name (ignored), third row = value of that id

summary
Summary # Ignored
A Test Summary
# { "summary": "A Test Summary" }

labels[]
Labels # Ignored
a-label
# { "labels": ["a-label"] }

issuetype.id
Issue Type # Ignored
12345
# { "issuetype": {"id": "12345"} }

components[].id
Components # Ignored
A Component
# { "components": [ {"id": "A Component"} ] }

watcher.watchers[].accountId
Watchers # Ignored
abcc281-qk3j8d8fj
# { "watcher": { "watchers": [{"accountId": "abcc281-qk3j8d8fj"}] } }

timetracking.originalEstimate,timetracking.remainingEstimate
Time Tracking,Time Tracking # Ignored
10,5
# { "timetracking": { "originalEstimate": "10", "remainingEstimate": "5" } }

fixVersions[].id,fixVersions[].id
Fix Versions,Fix Versions # Ignored
10000,10001
# { "fixVersions": [ {"id": "10000"}, {"id": "10001"} ] }
```
