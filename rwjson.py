#!/usr/bin/env python2
import json
import csv
import numpy as np

# Read a message from a JSON file
def readMessage(f):
    length = ''

    while (True):       # read into f
        c = f.read(1)
        if c == ':':
            break       # stop reading if colon
        length += c
        
    length = int(length)
    return json.loads(f.read(length))

# Wite a message in JSON format
def writeMessage(obj):
    output = json.dumps(obj,sort_keys = True)
    outlength = len(output)
    return str(outlength) + ':' + output # outlength does not count ':'




with open('examples/setup.sample') as f:
    handshake = readMessage(f)
    setup = readMessage(f)


with open('examples/writeTest', 'w') as f:
    message = writeMessage(setup)

print message

#~ print handshake 
#~ print '\n'
#~ print setup['map']['sites'], '\n'


# for each node in sites give id
numbSites = len(setup['map']['sites'])
sites = [None]*numbSites
for i in range(numbSites):
    sites[i] = setup['map']['sites'][i]      # sites is now a dictionary
    
    # print the i'th site just to make sure this is working....
    #~ print "sites", i, " = ", sites[i], '\n'
    

