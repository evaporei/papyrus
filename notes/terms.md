# Git

## Introduction

### repository

Is a collection of commits. Each commit being what the `working tree` looked like at the time.

It also defined a `HEAD` which identifies the `branch` **or** `commit` is currently being looked at.

Lastly, it contains a set of `branches` and `tags`, to identify certain `commits` by name.

### the index

The `staging area`. It is a place where you register changes on the project, which sits in between the `working tree` and the `repository`. So it is a middle person acting as a step before you actually commit something.

### working tree

Any directory on your filesystem, associated with a `repository`. Which basically means to have the `.git` folder.

### commit

A snapshot of your `working tree` at some point in time. When you create a new commit, the last one (`HEAD`) becomes its parent, and itself has the `HEAD` point to it.

### branch

It is just a name for a `commit`. Also called a reference. And links to the parentage of that commit, that's how you get the history.

### tag

Also a name for a `commit`. Similar to a branch, except it always names the same `commit`, and can have its own description text.

### master

Main branch of any repository. It's just a default, it could have another name/branch.

### HEAD

Used to define what is currently checked out:

- Checking out a `branch`, `HEAD` refers to it. Also indicates that the `branch` name should be updated after the next commit operation.
- Checking out a specific `commit`, makes `HEAD` refer to only that `commit`. Referred as *detached HEAD*. Also occurs with checking out `tag`.
