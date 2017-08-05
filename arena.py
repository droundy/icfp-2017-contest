#!/usr/bin/env python2

import sys, os, subprocess

import rwjson
import puntertools as pt

punter_executables = ['target/debug/punter', 'target/debug/punter']

if len(sys.argv) > 1:
    punter_executables = sys.argv[1:]

BUFFER_SIZE = 1024*1024

def offline_call(args, inp):
    print 'calling', args, 'with', inp
    x = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    data = rwjson.readMessage(x.stdout)
    print 'got offline handshake response', data
    x.stdin.write(rwjson.writeMessage({'you': data['me']}))
    x.stdin.write(rwjson.writeMessage(inp))
    print 'have written back to offline'
    return rwjson.readMessage(x.stdout)

states = [0]*len(punter_executables)
with open('maps/sample.json') as f:
    themap = rwjson.readJson(f)
serverstate = pt.map_to_nice(themap)
print 'serverstate is', serverstate

for punterid in range(len(punter_executables)):
    setup = {}
    setup['map'] = themap
    setup['punters'] = len(punter_executables)
    setup['punter'] = punterid

    states[punterid] = offline_call(['sh', '-c', punter_executables[punterid]], setup)

print 'states are', states
