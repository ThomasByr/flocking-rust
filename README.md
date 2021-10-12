# Flocking simulation in rust

1. [Introduction](#introduction)
2. [Rules](#rules)
3. [Features](#features)
4. [Dependencies](#dependencies)
5. [Changelog](#changelog)

## Introduction

In 1986 Craig Reynolds made a computer model of coordinated animal motion such as bird flocks and fish schools. It was based on three dimensional computational geometry of the sort normally used in computer animation or computer aided design. He called the generic simulated flocking creatures boids. The basic flocking model consists of three simple steering behaviors which describe how an individual boid maneuvers based on the positions and velocities its nearby flockmates (see [Rules](#rules)).

## Rules

| Illustration                                                                        | Rule                                                                        |
| ----------------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| <img src="images/separation.gif" alt="separation diagram" height="145" width="217"> | **Separation**: steer to avoid crowding local flockmates                    |
| <img src="images/alignment.gif" alt="alignment diagram" height="145" width="217">   | **Alignment**: steer towards the average heading of local flockmates        |
| <img src="images/cohesion.gif" alt="cohesion diagram" height="145" width="217">     | **Cohesion**: steer to move toward the average position of local flockmates |

Please visit this [link](http://www.red3d.com/cwr/boids/) for more. All credits to Craig Reynolds.

## Features

None ! Exept for the fact that the simulation seems to speed up on mouse events (for eg. moving the mouse in the window), could be a bug... nah. Also you can tweak some constants in the [boid.rs](src/boid.rs) file and in the [main.rs](src/bin/main.rs) file. Do not expect more than 400 updates per second, and for some strange reason the frame rate can not be kept around 60 even with ``window.set_max_fps(60);`` for more than one frame of animation.

## Dependencies

All dependencies can be found in [cargo.toml](Cargo.toml) :

*   piston_window
*   find_folder
*   fps_counter
*   rand

## Changelog

1.  initial commit
2.  boids on the screen (their position was a little off)
3.  quadtree acceleration which bring a ``$n*log{n}$`` time complexity
