#!/bin/env python3

import argparse
import cbor2
import os
import sys

from . import module as tRUST


project_path = os.path.abspath(os.path.curdir)

ENABLED_TYPES = [
    'uint32',
    'string',
    'bytearray'
]

def parse_user_args(args):
    """
    Parse list of user args into a list of dict

    Args:
        args (list): list of arg entries, each containing type and value

    Returns:
        list: list of dict with parsed args data
    """
    parsed_list = []
    for entry in args:
        parsed_dict = {}
        try:
            _type = entry[0].split(':')[1]
            value = entry[1]
            key = entry[0].split(':')[0]
            if _type not in ENABLED_TYPES:
                print(f'[!] unsupported type: {_type}\n[!] supported types are: {", ".join(ENABLED_TYPES)}')
                sys.exit(1)
            parsed_dict['name'] = key
            parsed_dict['value'] = value
            parsed_dict['kind'] = _type
        except ValueError as e:
            print(f'[!] error parsing "{entry}": {e}')
            sys.exit(1)
        parsed_list.append(parsed_dict)
    return parsed_list

parser = argparse.ArgumentParser()
parser.add_argument('command', nargs='?', choices=['compile', 'run', 'benchmark', 'codehash'],
                    help="Command to execute", default='none')
parser.add_argument('backend', nargs='?', choices=['local'], help="Backend to use", default='none')
parser.add_argument('-k', nargs='+', action='append',
                    help='Optional runtime arguments, format `key:type value`')
parser.add_argument('-m', '--mode', choices=['release', 'debug'],
                    default='debug', help='Run mode (release or debug)')
parser.add_argument('-v', '--verbose', action='store_true',
                    help='Enable verbose mode')
args = parser.parse_args()
key_value_pairs = parse_user_args(args.k) if args.k else []
mode_value = args.mode
command = args.command
backend = args.backend
verbose = args.verbose
with open('/tmp/trust.rargs', 'wb') as cbor_file:
    cbor2.dump(key_value_pairs, cbor_file)

def main():
    if command == 'compile':
        getattr(tRUST.backends, backend).compile(project_path, mode_value, verbose)
    elif command == 'run':
        getattr(tRUST.backends, backend).run(project_path, mode_value, verbose)
    elif command == 'benchmark':
        getattr(tRUST.backends, backend).benchmark(project_path, mode_value, verbose)
    elif command == 'codehash':
        getattr(tRUST.backends, backend).calculate_codehash(project_path, mode_value, verbose)
    elif command == 'list_targets':
        print('Backends:', tRUST.backends_enabled)
    else:
        print(f'[!] unknown command: {command}')
        sys.exit(1)
