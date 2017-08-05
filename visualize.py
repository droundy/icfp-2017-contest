#!/usr/bin/env python2
import json
import csv
import numpy as np
import matplotlib.pyplot as plt
import puntertools as pt
from rwjson import readMessage,writeMessage,readJson


maps = ['examples/setup.sample','maps/sample.json','maps/lambda.json',\
        'maps/Sierpinski-triangle.json','maps/tube.json','maps/boston-sparse.json']
        
gameState = ['examples/gameplay']


with open(maps[1]) as f:
    setup = pt.map_to_nice(readJson(f))

with open(gameState[0]) as f:
    handshake = readMessage(f)
    game = readMessage(f)
    pt.update_nice(setup,game['move']['moves'])

    
    
plt.figure()
for i in range(len(setup['riverdata'])):
    if setup['riverdata'][i]['claimed'] == None:
        riverid = setup['riverdata'][i]['id']
        source,target = setup['riverdata'][riverid]['sites']
        unclaimedx = [setup['sitemap'][source]['x'],setup['sitemap'][target]['x']]
        unclaimedy = [setup['sitemap'][source]['y'],setup['sitemap'][target]['y']]#   
        plt.plot(unclaimedx,unclaimedy,'-b')
    else:
        riverid = setup['riverdata'][i]['id']
        source,target = setup['riverdata'][riverid]['sites']
        claimx = [setup['sitemap'][source]['x'],setup['sites'][target]['x']]
        claimy = [setup['sites'][source]['y'],setup['sites'][target]['y']]#
#        print i, ":", ucx[0], ucy[0]," to ",ucx[1],ucy[1]
        plt.plot(claimx,claimy,'-y')
for j in range(len(setup['siteids'])):
    
    plt.plot(setup['sites'][j]['x'],setup['sites'][j]['y'],'k.')

for k in range(len(setup['mines'])):
    mineLoc = setup['mines'][k]
    plt.plot(setup['sites'][mineLoc]['x'],setup['sites'][mineLoc]['y'],'-ro')
plt.show()
