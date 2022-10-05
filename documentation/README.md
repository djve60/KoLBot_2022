# KoLBot_2022

KoLBot_2022 is code to automate actions for an account in the game [The Kingdom of Loathing](https://www.kingdomofloathing.com).

## Introduction

The name comes from the year coding started. This allows others to use the "KolBot" namespace.

This project replaces the code for [CWBot](https://github.com/rlbond86/cwbot), which was based on python 2.7 & pypi, with rust.

This uses data driven commands to enable actions and activities. This is very configurable. 

It is assumed anyone running this code is :
1. technically proficient in IT to understand how to use [git](https://git-scm.com/), 
1. understands unix and it's many variants, including OS X and linux.
1. understands the kill(1) command and signals in a unix type OS.

It is developed to run in a unix environment with less than 1Gb of memory so it is very frugal in it's resource usage. 

Oct 2022: 15.8Mb resident set size + 5.6Mb peak memory.

## Features

None as of this commit.

## Configuration

The file data/initialize.ini is where all configuration is set. Full details are in documentation/configuration.txt.

[Need to describe the ini format].

## Installation

'''
git checkout https://[no repository as I write this]

Obviously to be updated.
'''

## Running

If running interactively try to use SIGQUIT (normally 3) if shutting down locally.

Sending a command to shut the code down is not implemented at this time but should be in the second commit.

### Logging

All logging is shown in the logs directory. The current log ends in "rCURRENT.log". When restarted this is given a timestamp and compressed.

### Status

This directory is for data that needs to carry over restarts. This will mainly be of use when "delayed" consultations are enabled.
