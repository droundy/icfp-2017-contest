#!/usr/bin/env python2
from __future__ import division

import sys, os, subprocess, argparse, glob, itertools, random

import arena, rwjson

def rank_scores(scores):
    ranks = {}
    for i in range(len(scores)):
        ranks[scores[i][1]] = i
        for j in range(i+1,len(scores)):
            if scores[i][0] == scores[j][0]:
                ranks[scores[i][1]] = j
    return ranks

def battle(max_size, programs, vis):
    permutations = itertools.permutations(programs)
    pairs = {};
    for i in range(len(programs)+1):
        pairs[i] = set([])
    cumulative = {}
    games = {}
    for p in programs:
        cumulative[p] = 0
        games[p] = 0
    for p in permutations:
        for l in range(2,len(programs)+1):
            pairs[l].add(tuple(p[:l]))
    groups = []
    for l in range(2,len(programs)+1):
        pairs[l] = list(pairs[l])
        random.shuffle(pairs[l])
        groups += pairs[l][:len(pairs[2])]

    jobs = []
    for mapfile in glob.glob('maps/*.json'):
        with open(mapfile) as f:
            themap = rwjson.readJson(f)
        if len(themap['rivers']) > max_size:
            print mapfile,'is too long with size', len(themap['rivers'])
            continue
        for pair in groups:
            jobs.append((mapfile, pair))
    random.shuffle(jobs)
    for mapfile, pair in jobs:
        print '  {} {}'.format(mapfile, pair)
        with open(mapfile) as f:
            themap = rwjson.readJson(f)
            scores = arena.arena(mapfile, pair, vis=vis)
            ranks = rank_scores(scores)
            for p in ranks:
                cumulative[p] += ranks[p]
                games[p] += 1
            for (s,p) in scores:
                print '    {} [{:.2f}]: {} ({})'.format(ranks[p],
                                                        cumulative[p]/games[p],p,s)
    for p in programs:
        print '{}: {}'.format(cumulative[p], p)

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Compete some punters')
    parser.add_argument('--max', metavar='NODES', type=int, action='store',
                        default=100,
                        help='the largest size map to test')
    parser.add_argument('--visualize', action='store_true', help='visualize game')
    parser.add_argument('programs', metavar='PUNTER', nargs=argparse.REMAINDER,
                        help='the programs to compete')
    args = parser.parse_args()
    battle(args.max, args.programs, vis=args.visualize)
