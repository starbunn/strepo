# Contributing
(from Strepo tool)

Table Of Contents:
- [Contributing](#contributing)
    - [Granular Commiting](#granular-commiting)
    - [Atomic Commits](#atomic-commits)
    - [Conventional Commits](#conventional-commits)
    - [Branch Naming](#branch-naming)



### Granular Commiting
```
 grain commits on granular branch (every few minutes) ->
     safe-state commits on topic branch (hourly) ->
        safe-state commits on local Master branch (daily) ->
           commits on remote Personal Master (daily) ->
              pull request to Official Master (monthly) ->
                 commits on remote Official Master (monthly)
```

ie:
```
fix/task (topic)

fix/task! (do some work on this, commits don't have to follow conventional commits.)

squash-merge fix/task! into fix/task with a conventional commit, hourly

daily, rebase fix/task into dev <- (main local development branch)

daily, commit into dev <- (main remote development branch)


keep doing that
```

for example, i would do this:

```
git checkout fix/task!
(do some coding)
git add .
git commit -m "commit"
(ready to commit the grain, cause it is a ready commit)
git checkout fix/task
git merge --squash fix/task!
(do the git commit thing)
git checkout fix/task!
(^ to keep working on the changes)

and then when i feel like ive finished with the feature branch:
git rebase fix/task!
git rebase fix/task
git checkout dev
git merge --ff-only fix/task
```

[Link](http://blog.elliottcable.name/posts/granular_committing.xhtml)


### Atomic Commits
```
TLDR;

instead of commiting everything at once,
commit each part seperately.
for example, if your making a vue app,
commit each layout seperately, or each layout change seperately.

if your writing a node.js backend,
write each function, and commit each function seperately

keep granular commits in mind, as it actually helps with doing this because of the grains basically being atomic commits.

```

[Link](https://www.freshconsulting.com/insights/blog/atomic-commits/)


### Conventional Commits

uses angular's commit conventions, are very useful.

[Link](https://www.conventionalcommits.org/en/v1.0.0/)

### Branch Naming

```
TLDR;
dev - where development code is
main - where production code is
qa (or test) - contains code for testing

look at the article for the rest,
also use conventional commit types for this.

for example,

feat/atomic! (for a atomic granular branch)

chore/chore (a chore branch)

build/jenkins4 (a branch with code for builds.)

etc.

```

[Link](https://dev.to/couchcamote/git-branching-name-convention-cch)
