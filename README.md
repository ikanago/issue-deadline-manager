# Issue Deadline Manager

[![CI](https://github.com/ikanago/issue-deadline-manager/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ikanago/issue-deadline-manager/actions/workflows/ci.yml)
[![Build Docker image](https://github.com/ikanago/issue-deadline-manager/actions/workflows/build_image.yml/badge.svg?branch=main)](https://github.com/ikanago/issue-deadline-manager/actions/workflows/build_image.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

This is a GitHub Action to label an issue by its deadline specified with a slash command and notify when there is updates on labels.
It is recommended to use in a repository for a daily life TODO list, but it might useful for software development.

## Set up
### Use in a existing repository
Create `.github/workflows/todo.yml`(any file name goes well, of course) and copy and paste following.
```yml
name: Manage TODO

on:
  schedule:
    - cron: '0/10 * * * *'
  issues:
    types: [opened, edited]

jobs:
  update-todo:
    runs-on: ubuntu-20.04
    steps:
      - uses: ikanago/issue-deadline-manager@v1
```
Then commit it and push to `master`.

### Create from template
You can create a new repository with the setting completed.
Go to [issue-todo-template](https://github.com/ikanago/issue-todo-template) and create a new repository by using it as a template.

## How to use
After setting up, create an issue as a TODO.
And write its deadline with a slash command like this:
```text
/deadline 2021/12/31
```
After a while, github-actions bot adds an label according to the deadline and comments.
![Working example](/assets/example.png)

The command format is `/deadline ${DATE} ${TIME}`.
`DATE` format is `yyyy-MM-dd` or `MM-dd`.
`TIME` is optional. The format is `HH:mm`.

