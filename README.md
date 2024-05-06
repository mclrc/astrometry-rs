# astrometry-rs 
[![Tests](https://github.com/mclrc/astrometry-rs/actions/workflows/tests.yml/badge.svg)](https://github.com/mclrc/astrometry-rs/actions/workflows/tests.yml)

This repository is my (very WIP) attempt at implementing blind astrometric image calibration, which is the process of determining the locations of an arbitrary astronomical images, as well as the identities of the visible stars, based on nothing but the images themselves.

Solving this problem requires a very clever suite of algorithms developed by some very smart people. They are outlined in the [astrometry.net paper](https://arxiv.org/abs/0910.2233), which this project is based on.

## The basic components
### Source extraction
We need to be able to extract pixel-space coordinates of stars from an image, and we need to do so *very accurately*. In fact, we ideally require sub-pixel accuracy.
### Geometric hashing
Because all we will have to go on is the image, and the most meaningful information the image contains is the relative arrangement of the visible stars, we need to be able to make use of this information.

This is done using a geometric hashing algorithm, which encodes the relative positions of four stars ("quads" of stars) at a time as four numbers. These four numbers are independent of the rotation and scaling of the image, the order the stars are looked at, etc. - They solely encode the relative positions of the four stars.
### Index building
To form hypotheses of possible solutions, we will build an index containing the geometric hashes of a lot of quads built from a lot of known stars, and I mean *a lot.* The astronomical catalog I am using is the USNO B1, which contains in excess of a billion stars.
We want to be able to recognize images of arbitrary scales and positions, so we have to be clever about how we construct quads based on the catalog.
Searching the index for the geometric hashes of quads built from stars in the query image is how we can form guesses about where that image is and which stars it contains.
### Bayesian decision making
In order to determine how good such a guess is, we can employ bayesian decision making. If the hypothesis is accurate, we can use it to predict the presence and location of other stars in the image where we know stars should be based on the index.
If the hypothesis is good (meaning orders of magnitude better than chance) at predicting the locations of other stars in the query image, we accept it as true. 
### The solver
Putting it all together.

## But why?
I have implemented a small part of these things, all the code is in this repository. I find that the higher level nature as well as the more expressive type system of Rust make it a bit more easily understandable than the original C implementation. It should be easier to parallelize as well.
I will also try to use more common data formats - an SQLite database, while it may be slightly less performant, and Rust structs serialized and deserialized using Serde, will be much more familiar and ergonomic to most developers than the .fits files extensively used in the reference implementation.
So if this ends up coming together, the code will hopefully be of some use to those looking to understand how this works, which is the goal for me personally as well. It's already been lots of fun, we'll see where it goes. :)
