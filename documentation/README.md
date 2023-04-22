The fantastic people at [**Old CW's Germ Free Clan**](https://www.oldcws.org/index.php), and the usual gang of fools at [**Pan Transcendental Industries**](https://en.wikipedia.org/wiki/Hawklords) ("Reality you can rely on.") proudly present:

# KoLBot_2022

KoLBot_2022 is code to automate actions for an account in the game [The Kingdom of Loathing](https://www.kingdomofloathing.com).

## Introduction

The name comes from the year coding started. This allows others to use the "KolBot" namespace.

This project replaces the code for [CWBot](https://github.com/rlbond86/cwbot), which was based on python v2.7 & pypi, with rust.

This uses data driven commands to enable actions and activities. This is very configurable. 

It is assumed anyone running this code is :
1. technically proficient in IT to understand how to use [git](https://git-scm.com/), 
1. understands unix and it's many variants, including OS X and linux.
1. understands the kill(1) command and signals in a unix type OS.

It is developed to run in a unix environment with less than 1Gb of memory so it is very frugal in it's resource usage. 

Using *top -l 1* 
v0.1.2: 4 theads, 7.992Mb. After over an hour 0.00% CPU time.

## Features

1. Fortune: the unix fortune(1) command, with responses under 200 characters.
1. Rule: return local rules for the clan basement dungeons.

## Configuration

The file data/initialize.ini is where all configuration is set. Full details are in documentation/configuration.txt.

This uses the [INI specification](https://en.wikipedia.org/wiki/INI_file).

## Installation

'''
git checkout https://github.com/djve60/KoLBot_2022

Obviously to be updated, with "Follow the instructions in <file>.
'''

## Running

If running interactively try to use SIGQUIT (normally 3) if shutting down locally. The is CTRL-C for most operating systems.

Sending a command to shut the code down is not implemented at this time but should be in the third commit.

## Directories

### data

Static data. This includes the configuration directives file, read at start-up.

### logging

All logging is shown in the logs directory. The current log ends in "rCURRENT.log". When restarted this is given a timestamp and compressed.

### status

The status directory is for data that needs to carry over restarts. This will mainly be of use when "delayed" consultations are enabled.
