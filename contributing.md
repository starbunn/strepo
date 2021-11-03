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

fix/task! (doing some work on this, rebase this every few minutes)

rebase task! into task

keep doing that
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

keep granular commits in mind.

```

[Link](https://www.freshconsulting.com/insights/blog/atomic-commits/)


### Conventional Commits

commitlint should help with this one, read this for documentation (using angular's stuff):

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
