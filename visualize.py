#!/usr/bin/env python2
import matplotlib.pyplot as plt
import puntertools as pt
from rwjson import readMessage,writeMessage,readJson

def visualize_board(setup):
    for i in range(len(setup['riverdata'])):
        if setup['riverdata'][i]['claimed'] == None:
            riverid = setup['riverdata'][i]['id']
            source,target = setup['riverdata'][riverid]['sites']
            unclaimedx = [setup['sitemap'][source]['x'],setup['sitemap'][target]['x']]
            unclaimedy = [setup['sitemap'][source]['y'],setup['sitemap'][target]['y']]#   
            plt.plot(unclaimedx,unclaimedy,'-b')
        elif setup['riverdata'][i]['claimed'] == punter:
            riverid = setup['riverdata'][i]['id']
            source,target = setup['riverdata'][riverid]['sites']
            claimx = [setup['sitemap'][source]['x'],setup['sites'][target]['x']]
            claimy = [setup['sites'][source]['y'],setup['sites'][target]['y']]
            plt.plot(claimx,claimy,farg[punter],linewidth = 2)
    for j in range(len(setup['siteids'])):
        plt.plot(setup['sites'][j]['x'],setup['sites'][j]['y'],'k.')
    for k in range(len(setup['mines'])):
        mineLoc = setup['mines'][k]
        plt.plot(setup['sites'][mineLoc]['x'],setup['sites'][mineLoc]['y'],'-ro')
        
with open('examples/gameplay0.txt') as f:
    handshake = readMessage(f)
    messageIn = readMessage(f)
    setup = pt.map_to_nice(messageIn['move']['state']['map'])
    R = len(setup['riverdata'])

farg = ['-y','-m','-m','-y','-w']
    
for turn in range(R):
    gameState = ['examples/gameplay'+str(turn)+'.txt']
    
    with open(gameState[0]) as f:
        handshake = readMessage(f)
        game = readMessage(f)
        pt.update_nice(setup,game['move']['moves'])
        punter = game['move']['moves'][0]['claim']['punter']
    visualize_board(setup)

plt.show()


