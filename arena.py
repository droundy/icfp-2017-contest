#!/usr/bin/env python2

import sys, os, subprocess, argparse

import rwjson
import puntertools as pt

def offline_call(args, inp):
    #print 'calling', args, 'with', inp
    x = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    data = rwjson.readMessage(x.stdout)
    #print 'got offline handshake response', data
    x.stdin.write(rwjson.writeMessage({'you': data['me']}))
    x.stdin.write(rwjson.writeMessage(inp))
    #print 'have written back to offline'
    return rwjson.readMessage(x.stdout)

def arena(mapfile, punter_executables):
    num_punters = len(punter_executables)
    states = [0]*len(punter_executables)
    with open(mapfile) as f:
        themap = rwjson.readJson(f)
    serverstate = pt.map_to_nice(themap)
    #print 'serverstate is', serverstate

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
            # print '\nresult is\n', result
            states[punterid] = result['state']
            del result['state']
            pt.update_nice(serverstate, [result])
            all_moves.append(result)
        scores = {}
        for pid in range(num_punters):
            score = pt.score(serverstate, pid)
            print 'score for', pid, 'is', score
            if punter_executables[pid] not in scores:
                scores[punter_executables[pid]] = []
            scores[punter_executables[pid]].append(score)
    return scores

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Compete some punters')
    parser.add_argument('--map', metavar='MAP', nargs='?',
                        default='maps/sample.json',
                        help='the map to use')
    parser.add_argument('programs', metavar='PUNTER', nargs=argparse.REMAINDER,
                        help='the programs to compete')

    args = parser.parse_args()
    while len(args.programs) < 2:
        args.programs.append('target/debug/punter')

    if '.json' not in args.map:
        args.map = 'maps/{}.json'.format(args.map)
    print 'using map', args.map

    scores = arena(args.map, args.programs)
    print 'scores are {}'.format(scores)
