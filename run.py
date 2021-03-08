#! /usr/bin/env python3

import subprocess
import pprint
import sys
import json

if len(sys.argv) < 2:
    print('Usage: ./run.py [dev/prod]')
else:
    run_type = sys.argv[1]

    print(f'Run type: {run_type}')

    env_json_file = open('env.json', 'r')
    env_json = json.loads(env_json_file.read())
    env_json_file.close()

    index_html_template_file = open('frontend/index.template.html', 'r')

    index_html_str = index_html_template_file.read()
    index_html_template_file.close()

    index_html = ''
    build_cmd = ''
    build_location = ''

    if run_type == 'dev':
        index_html = index_html_str.format('\'http://localhost:8080\'')
        build_cmd = 'cargo build'
        build_location = 'target/debug/twiot-gateway'
    elif run_type == 'prod':
        index_html = index_html_str.format('`http://${window.location.host}`')
        build_cmd = 'cross build --target armv7-unknown-linux-musleabihf'
        build_location = 'target/armv7-unknown-linux-musleabihf/debug/twiot-gateway'

    frontend_index_html = open('frontend/index.html', 'w+')
    frontend_index_html.write(index_html)
    frontend_index_html.close()

    steps = [
        ('.', 'rm -rf dist'),
        ('.', 'mkdir -p dist/frontend'),
    ]

    if run_type == 'prod':
        steps.extend([
            ('.', build_cmd),
            ('.', f'cp {build_location} .env dist'),
            ('./frontend', './build.sh'),
            ('.', 'cp -r frontend/dist/* dist/frontend'),
            ('./dist', 'zip -r release.zip *')
        ])
    else:
        steps.extend([('.', 'cargo run')])

    for (path, step) in steps:
        print(path, step)
        subprocess.run(step, cwd=path, shell=True)
