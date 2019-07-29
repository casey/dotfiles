#!/usr/bin/env python3

import sys

def e(*args, **kwargs):
  print(*args, **kwargs, file=sys.stderr)

def die(*args, **kwargs):
  e(*args, **kwargs)
  sys.exit(-1)

binary = sys.argv[0]

commands = {}

def cmd(f):
  commands[f.__name__] = f

@cmd
def py(*args):
  print(eval(' '.join(args)))

@cmd
def get(url):
  import code, re

  try:
    import requests
  except:
    die('{}: get: requests not installed'.format(binary))
 
  try:
    r = requests.get(url)
  except requests.exceptions.MissingSchema:
    r = requests.get('http://' + url)
  code.interact(local=locals())
  sys.exit(0)

if len(sys.argv) == 1:
  variables = {}
  exports = {}

  def var(name, binding):
    variables[name] = binding

  def export(name, binding):
    exports[name] = binding

  var('hello', 2)
  export('bar', 7)

  for name in variables:
    print('{}={}'.format(name, variables[name]))

  for name in exports:
    print('export {}={}'.format(name, exports[name]))

  for command in commands:
    print('alias {}=\'{} {}\''.format(command, sys.argv[0], command))
  sys.exit(0)

command = sys.argv[1]

if command not in commands:
  e('{}: unknown command: {}'.format(sys.argv[0], command))
  sys.exit(1)

commands[command](*sys.argv[2:])
