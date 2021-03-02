#! /usr/bin/env python3

import subprocess
import pprint
import sys

if len(sys.argv) < 2:
    print('Usage: ./run.py [dev/prod]')
else:
    steps = [
        ('.', 'rm -rf dist'),
        ('.', 'mkdir -p dist/frontend'),
        ('.', 'cross build --target armv7-unknown-linux-musleabihf'),
        ('.', 'cp target/armv7-unknown-linux-musleabihf/debug/twiot-gateway dist'),
        ('./frontend', './build.sh'),
        ('.', 'cp -r frontend/dist/* dist/frontend'),
        ('./dist', 'zip -r release.zip *')
    ]

    for (path, step) in steps:
        print(path, step)
        subprocess.run(step, cwd=path, shell=True)
