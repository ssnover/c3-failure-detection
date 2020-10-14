# c3-failure-detection
Oxidized version of programming assignment from Cloud Computing Concepts, Part 1

This is intended to be an alternate set of starter code for the programming assignment from the University 
of Illinois' Coursera course: Cloud Computing Concepts, Part 1. This course can be found here: 
https://www.coursera.org/learn/cloud-computing/home/welcome

If you're like me, or many of the other frustrated students posting to the forums, you may have found the provided 
programming assignment to be obnoxiously difficult for various reasons including: the horrible memory management 
with raw pointers, obtuse documentation, terrible provided APIs, and general just C with .cpp extension shenanigans 
that abounded. The assignment also required Linux, pretty arbitrarily (pretty easy to remove that requirement 
without affecting the assignment). I wanted to focus on the actual provided problem: implement a failure detector
for a distributed system. Perhaps you do too, and if so you've come to the right place.

# How to use this repo
You should fork this repo for working on your own failure detector implementation, it has been provided as-is on
the `master` branch in an incomplete state. Your implementation should live as implementation of the `Node` struct
found in `src/node.rs`. You can feel free to add methods and member variables to this struct and provided that you
don't change the signature of `Node::new` or `Node::run` you have free rein of that file.

You can test your implementation with the suite of tests defined in `src/lib.rs`. There are just four tests (one 
more than provided in the original assignment, just a simple case with no failures as a sanity check). You can
validate your implementation with this test suite by running `cargo test` and by checking the contents of the file
created by the logger `debug.log`. Note that this log file is overwritten by each individual test case so if there's
one test case you're interested in you can run it individually with `cargo test TESTNAME` with no quotes on the test
name.

# Contributions
If you've got some improvements that should be made to the APIs in the provided code, feel free to submit a PR.
Please do not submit PRs which implement the Node struct, that defeats the purpose of this repo.
