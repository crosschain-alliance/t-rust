#!/bin/env python3

import os
import sys


grandparent_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), os.pardir, os.pardir))
sys.path.append(grandparent_dir)

import docker_mgmt


__name__ = "JOLT"
target = 'jolt'

def compile(project_path, mode_value, verbose):
    """
    Compile project

    Args:
        project_path (str): path of the project
        mode_value (str): debug or release mode (default debug)
        verbose (bool): enable verbose mode
    """
    docker_mgmt.run_container('compile', target, project_path, mode_value, verbose)

def run(project_path, mode_value, verbose):
    """
    Run compiled project

    Args:
        project_path (str): path of the project
        mode_value (str): debug or release mode (default debug)
        verbose (bool): enable verbose mode
    """
    docker_mgmt.run_container('run', target, project_path, mode_value, verbose)

def benchmark(project_path, mode_value, verbose):
    """
    Benchmark compiled project getting execution time

    Args:
        project_path (str): path of the project
        mode_value (str): debug or release mode (default debug)
        verbose (bool): enable verbose mode
    """
    docker_mgmt.run_container('benchmark', target, project_path, mode_value, verbose)

def calculate_codehash(project_path, mode_value, verbose):
    """
    Calculate compiled project bin's codehash

    Args:
        project_path (str): path of the project
        mode_value (str): debug or release mode (default debug)
        verbose (bool): enable verbose mode
    """
    docker_mgmt.run_container('codehash', target, project_path, mode_value, verbose)