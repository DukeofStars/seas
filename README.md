# Seas
The Seas project aims to unify package management and installation (specifically for windows).

###### Note
The Seas project is still under heavy development and is not yet ready for use.

## Jellyfish
Jellyfish is a barebones, global package format that is extendable. It serves as a kind of base, where everything else branches off.
It also contains some boilerplate code for setting up a jellyfish repository with rocket.

## Seas
Seas is a package server which hosts a jellyfish repository.

## Squid
Squid is a package manager frontend, designed for the jellyfish package format.

## Wharf
Wharf branches off almost entirely, and is it's own "fork" of the jellyfish format. Packages are loaded onto "ships" which are files containing instructions for wharf.
This allows for some more advanced installations.

## Contribution
It should be pretty simple. The project does not rely on any external non-rust libraries and should just work with ```cargo build```.