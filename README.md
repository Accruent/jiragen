# JiraGen

A library and CLI tool to generate JIRA issues. This makes it easy to create issue "templates" for repeatable processes (such as releasing a new software version) and track that in JIRA. Uses [version 2 of JIRA’s REST API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/).

[CLI Docs](jiragen-cli/)
[Library Docs](jiragen-lib/)

## The Issues Template file

JiraGen reads a .csv file that represents the issues (and their field values) to be created in JIRA.

The first row of the generated .csv file (from the `init` command) contains the field ids, and the second row contains the field names. Actual issue content should be entered starting on the third row. Note that the second row is ignored (it is used for human readability).

JiraGen will read from this file in order to generate the corresponding issues in JIRA (using the `push` command).

## .csv syntax

Because JIRA’s API requires that the issues’ fields be shaped to specific schemas (See [JIRA’s API](https://developer.atlassian.com/cloud/jira/platform/rest/v2/#api-api-2-issue-bulk-post) for an example), we translate that schema to the .csv file: `[]` and `.` describe an array or object property, respectively. Remember that the second row of the .csv file is ignored.

Some examples of how data is converted from the .csv file to JSON:

```bash
# first row = csv id field/key, second row = readable field name (ignored), third row = value of that id

summary
Summary
A Test Summary
# { "summary": "A Test Summary" }

labels[]
Labels
a-label
# { "labels": ["a-label"] }

issuetype.id
Issue Type
12345
# { "issuetype": {"id": "12345"} }

components[].id
Components
A Component
# { "components": [ {"id": "A Component"} ] }

watcher.watchers[].accountId
Watchers
abcc281-qk3j8d8fj
# { "watcher": { "watchers": [{"accountId": "abcc281-qk3j8d8fj"}] } }

timetracking.originalEstimate,timetracking.remainingEstimate
Time Tracking,Time Tracking
10,5
# { "timetracking": { "originalEstimate": "10", "remainingEstimate": "5" } }

fixVersions[].id,fixVersions[].id
Fix Versions,Fix Versions
10000,10001
# { "fixVersions": [ {"id": "10000"}, {"id": "10001"} ] }
```
