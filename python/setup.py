#!/usr/bin/env python
from setuptools import setup

# This ensures the library name includes Python version, ABI, and platform tags
setup(has_ext_modules=lambda: True)
