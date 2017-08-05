#!/usr/bin/env python2

import sys, os, subprocess

import rwjson
import puntertools as pt

punter_executables = ['target/debug/punter', 'target/debug/punter']

if len(sys.argv) > 2:
    punter_executables = sys.argv[1:]

num_punters = len(punter_executables)

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

all_moves = []
for punterid in range(num_punters):
    setup = {}
    setup['map'] = themap
    setup['punters'] = num_punters
    setup['punter'] = punterid

    ready = offline_call(['sh', '-c', punter_executables[punterid]], setup)
    states[punterid] = ready['state']
    all_moves.append({'pass': {'punter': punterid}})

print '\nstates are', states

for movenum in range(len(serverstate['siteids'])):
    punterid = movenum % num_punters
    # the following passes just the most recent num_punters moves
    gameplay = {
        'move': {'moves': all_moves[-num_punters:]},
        'state': states[punterid],
    }
    result = offline_call(['sh', '-c', punter_executables[punterid]], gameplay)
    if result is None:
        all_moves.append({'pass': {'punter': punterid}})
        print 'bad result from punter'
    else:
        print '\nresult is\n', result
        states[punterid] = result['state']
        del result['state']
        pt.update_nice(serverstate, [result])
        all_moves.append(result)
        print 'score for', punterid, 'is', pt.score(serverstate, punterid)
