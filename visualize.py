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


with open(maps[4]) as f:
    setup = pt.map_to_nice(readJson(f))

#print setup
print setup['siteids']

plt.figure()
for i in range(len(setup['riverdata'])):
    if setup['riverdata'][i]['claimed'] == None:
        riverid = setup['riverdata'][i]['id']
#        print riveri
        source,target = setup['riverdata'][riverid]['sites']
        print target, source, len(setup['sites'])
        claimedx = [setup['sitemap'][target]['x'],setup['sitemap'][source]['x']]
        claimedy = [setup['sitemap'][target]['y'],setup['sitemap'][source]['y']]#        
        plt.plot(claimedx,claimedy,'-b')
    else:
        ucx = [setup['sites'][target]['x'],setup['sites'][source]['x']]
        ucy = [setup['sites'][target]['y'],setup['sites'][source]['y']]#
        plt.plot(ucx,ucy,'-y')
for j in range(len(setup['siteids'])):
    
    plt.plot(setup['sites'][j]['x'],setup['sites'][j]['y'],'k.')

for k in range(len(setup['mines'])):
    mineLoc = setup['mines'][k]
    plt.plot(setup['sites'][mineLoc]['x'],setup['sites'][mineLoc]['y'],'-ro')
plt.show()
