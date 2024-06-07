#!/usr/bin/env python3

from argparse import ArgumentParser
from glob import glob
from pathlib import Path
from tomllib import loads as tomllib_loads
from subprocess import run as subprocess_run
from io import StringIO
from re import compile as re_compile
from re import sub as re_sub
import logging

REGEX_CONTROL_CHARS = re_compile(r'\[.*?;.*?m')

def process_test(exec_path, s_conf, s_in, s_out):
  global REGEX_CONTROL_CHARS
  conf = tomllib_loads(s_conf)
  if 'command' not in conf: 
    raise Exception('no command found in configuration test') 
  command = conf['command'].strip().replace("$moustache", exec_path)
  returncode = int(conf['returncode'])
  completed_process = subprocess_run(
    command, 
    shell=True,
    input=s_in.strip(),
    capture_output=True,
    text=True
  )
  logging.debug(completed_process.stdout.strip())
  if completed_process.returncode != returncode:
    raise Exception(f'invalid return code (desired: {returncode}; found: {completed_process.returncode})') 
  stdout = re_sub(REGEX_CONTROL_CHARS, '', completed_process.stdout.strip())
  if 'compare_stdout' in conf:
    if conf['compare_stdout'] is False: 
      return 
  if stdout != s_out.strip():
    raise Exception('invalid stdout from process') 

def get_parts(path):
  parts = []
  part = []
  with open(path, 'r') as f: 
    separator = f.readline().strip()
    for line in f: 
      if line.strip() == separator: 
        parts.append("".join(part))
        part = []
      else:
        part.append(line)
    if len(part) > 0:
      parts.append("".join(part))
  return parts

def process(args):
  OK = 0
  KO = 0
  tests_paths = Path(args.tests_path).glob(args.tests_pattern) 
  for i, test_path in enumerate(tests_paths):
    logging.debug(f'Test n°{i} - Process of "{test_path}"... ')
    parts = get_parts(test_path)
    s_conf, s_in, s_out, *s_others = parts
    if len(s_others)>0:
      raise Exception('invalid test file format (too much parts !)')
    try: 
      process_test(args.exec_path, s_conf, s_in, s_out)
      logging.debug(f'Test n°{i} - Test passed...')
      OK += 1
    except Exception as err: 
      logging.error(f'Test n°{i} - Test failed: {err}') 
      KO += 1
  return KO, (OK+KO)

if __name__=='__main__':
  parser = ArgumentParser(
    prog='moustache-test',
    description='Intregration and functional tests for Moustache',
    epilog='by Julien Garderon <julien.garderon@gmail.com>'
  )
  parser.add_argument(
      '-v', '--verbose',
      help="More (and more) verbose",
      action='count', 
      default=0
  )
  parser.add_argument(
    '--tests-path',
    help='the path to tests files (must be a valid directory)', 
    required=True
  )
  parser.add_argument(
    '--tests-pattern',
    help='the pattern of tests files', 
    default='*.test'
  )
  parser.add_argument(
    '--exec-path',
    help='the path to executable', 
    required=True
  )
  args = parser.parse_args()
  logging.basicConfig(
    encoding='utf-8', 
    level=[
      logging.CRITICAL,
      logging.ERROR,
      logging.WARNING,
      logging.INFO,
      logging.DEBUG,
    ][args.verbose%5]
  )
  ko, total = process(args)
  (logging.info if ko == 0 else logging.warning)(f'{total} test(s) passed (with {ko} failed)') 


