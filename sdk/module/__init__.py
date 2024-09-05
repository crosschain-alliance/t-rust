#!/bin/env python3

import os
import importlib


backends_enabled = []
def load_backends():
    """
    Load and enable all backend modules from 'backends' dir
    """
    for backend in os.listdir(os.path.dirname(__file__) + '/backends'):
        try:
            importlib.import_module('sdk.module.backends.' + backend)
            backends_enabled.append(backend)
        except ImportError as e:
            print(f'[!] error while importing {backend}: {e}')

load_backends()
