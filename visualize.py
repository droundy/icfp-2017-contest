#!/usr/bin/env python2
import json
import csv
import numpy as np
import matplotlib.pyplot as plt
from rwjson import readMessage,writeMessage


maps = ['examples/setup.sample','maps/sample.json','maps/lambda.json',\
        'maps/Sierpinski-triangle.json','maps/tube.json','maps/boston-sparse.json']
        
gameState = ['examples/gametest']


with open(maps[0]) as f:
    handshake = readMessage(f)
    setup = readMessage(f)

    mines = setup['map']['mines']
    sites = setup['map']['sites']
    rivers = setup['map']['rivers']
    
with open(gameState[0]) as f:
    handshake = readMessage(f)
    setup = readMessage(f)
    oppClaim = setup['move']['moves'][0]['claim']# The integer represents the moves made
    source = oppClaim['source']
    target = oppClaim['target']


message = writeMessage(handshake)
print message

riverClaimed = []
for i in range(len(rivers)):
    if target == rivers[i]['target'] and source ==rivers[i]['source']:
        riverClaimed.append(i)



plt.figure()
for i in range(len(rivers)):
    if i in riverClaimed:
        C = [sites[rivers[i]['target']]['x'],sites[rivers[i]['source']]['x']]
        D = [sites[rivers[i]['target']]['y'],sites[rivers[i]['source']]['y']]
        plt.plot(C,D,'-y')
    else:
        A = [sites[rivers[i]['target']]['x'],sites[rivers[i]['source']]['x']]
        B = [sites[rivers[i]['target']]['y'],sites[rivers[i]['source']]['y']]
        plt.plot(A,B,'-b')
for j in range(len(sites)):
    plt.plot(sites[j]['x'],sites[j]['y'],'k.')

for k in range(len(mines)):
    plt.plot(sites[mines[k]]['x'],sites[mines[k]]['y'],'-ro')
plt.show()
