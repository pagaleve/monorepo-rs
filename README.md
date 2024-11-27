# This is a monorepo in Rust

### Bellow are some explanation of how this monorepo works. But first, you need to prepare the project.

### First of all, prepare the project
```make prepare```

#### The command above is responsible to:
- Copy both pre commit and pre push files to .git folder
- Install cargo-edit to take care of crate versions
- Install nextest as our test runner
- Some commands that you might use

### Now you are ready to create some branchs and push to git and let the magic happens
- If the branch is something as *"feat*"*, we bump the minor version of all crates
- If the branch is something as *"fix*"*, we bump the patch version.
- If the branch is something as *"break*"*, we bump the major version.

### Github Pipeline
When the PR is merged to main branch, the pipeline will do:
- Run clippy
- Run tests
- Generate git tags and publish to.. whatever you want
