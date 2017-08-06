#!/usr/bin/env python2
from __future__ import division

import sys, os, subprocess, argparse, glob, itertools

import arena, rwjson

def rank_scores(scores):
    ranks = {}
    for i in range(len(scores)):
        ranks[scores[i][1]] = i
        for j in range(i+1,len(scores)):
            if scores[i][0] == scores[j][0]:
                ranks[scores[i][1]] = j
    return ranks

def battle(max_size, programs):
    permutations = itertools.permutations(programs)
    pairs = set([])
    triples = set([])
    cumulative = {}
    for p in programs:
        cumulative[p] = 0
    for p in permutations:
        pairs.add(tuple(sorted(p[:2])))
        pairs.add(tuple(reversed(sorted(p[:2]))))
        if len(programs) > 2:
            for pperm in itertools.permutations(p[:3]):
                triples.add(tuple(pperm))
    for mapfile in glob.glob('maps/*.json'):
        with open(mapfile) as f:
            themap = rwjson.readJson(f)
        if len(themap['rivers']) > max_size:
            print mapfile,'is too long with size', len(themap['rivers'])
            continue
        print '\ntesting map', mapfile, 'with size', len(themap['rivers'])
        for pair in pairs:
            scores = arena.arena(mapfile, pair)
            ranks = rank_scores(scores)
            print 'ranks are', ranks, 'and scores are', scores
            for p in ranks:
                cumulative[p] += ranks[p]
        for pair in triples:
            scores = arena.arena(mapfile, pair)
            ranks = rank_scores(scores)
            for p in ranks:
                cumulative[p] += ranks[p]
        print 'cumulative so far:', cumulative
    print 'cumulative score:', cumulative

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Compete some punters')
    parser.add_argument('--max', metavar='NODES', type=int, nargs=1,
                        default=100,
                        help='the largest size map to test')
    parser.add_argument('programs', metavar='PUNTER', nargs=argparse.REMAINDER,
                        help='the programs to compete')
    args = parser.parse_args()
    battle(args.max, args.programs)
