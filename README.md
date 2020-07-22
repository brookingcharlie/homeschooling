# Homeschooling

Rust implementation for Shokunin Jul 2020,
using recursion to search for an even division of tasks.

Prerequisites: Docker

To build and run the app using the example input file:

```
$ ./go run < input.txt
Yes

Child 1: Task A (5 points) + Task B (4 points) + Task C (1 point) = 10 points
Child 2: Task D (2 points) + Task F (8 points) = 10 points
Child 3: Task E (7 points) + Task G (3 points) = 10 points
```

For other commands, run `./go` to see usage.

```
$ ./go
Usage:
./go run < input.txt: build and run application
./go test: run unit tests
./go perf: run perf test partitioning large input (tip: run twice since it builds first time)
./go clean: remove build file
```

This was my first time using Rust. I found the learning curve steep: the compiler
enforces memory safety by requiring that you're explicit about owernship and references.
This is definitely a language that requires a decent investment in understanding
and effort but I can see how it'd pay off for a performance-critical system.

## Background

In one busy Australian household both parents have been working from home during
lockdown while also home schooling their three children (triplets). Each day the
children's teacher assigns a list of tasks for them to complete between them.
Each task is assigned some points based on difficulty. The parents are keen that
each day each child completes the same number of points so as not to fall behind.
And also to stop the constant bickering about who does more/less work :)

For example on Monday the teacher assigned:

* Task A: 5 points
* Task B: 4 points
* Task C: 1 point
* Task D: 2 points
* Task E: 7 points
* Task F: 8 points
* Task G: 3 points

These tasks can be split amongst the triplets like so:

* Child 1: Task D (2 points) + Task F (8 points) = 10 points
* Child 2: Task A (5 points) + Task B (4 points) + Task C (1 point) = 10 points
* Child 3: Task E (7 points) + Task G (3 points) = 10 points

The children cannot work on the same task. That would require the negotiation
skills of a UN diplomat.

The family needs a way to quickly assess if the tasks can be divided amongst all
the children so they each get the same number of points. If the tasks fail to be
split between the three children they will send them back to the teacher and ask
for a new set of tasks.

## Challenge

The challenge is to write some code that will take any given list of tasks with
associated points and output No if the tasks can't be split with points divided
evenly between the three children; and output Yes plus the tasks with points for
each child if they can.

## Criteria for Awesomeness

* Solving the problem as outlined in "Challenge"
* A command line interface for inputting tasks/points and outputting the result.
* Clean Code
* Evidence of TDD
* A go script and README
