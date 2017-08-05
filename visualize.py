#!/usr/bin/env python2
import json
import csv
import numpy as np
import matplotlib.pyplot as plt


maps = ['sample.json','lambda.json','Sierpinski-triangle.json','tube.json','boston-sparse.json']

# Read a message from a JSON file
def readMessage(f):
    return json.loads(f.read())

# Wite a message in JSON format
def writeMessage(obj):
    output = json.dumps(obj,sort_keys = True)
    outlength = len(output)
    return str(outlength) + ':' + output # outlength does not count ':'

with open('maps/'+maps[4]) as f:
    setup = readMessage(f)
    mines = setup['mines']
    sites = setup['sites']
    rivers = setup['rivers']


plt.figure()
for i in range(len(rivers)):
    A = [sites[rivers[i]['target']]['x'],sites[rivers[i]['source']]['x']]
    B = [sites[rivers[i]['target']]['y'],sites[rivers[i]['source']]['y']]
    plt.plot(A,B,'-b')
for j in range(len(sites)):
    plt.plot(sites[j]['x'],sites[j]['y'],'k.')

for k in range(len(mines)):
    plt.plot(sites[mines[k]]['x'],sites[mines[k]]['y'],'-ro')
plt.show()
