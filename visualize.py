#!/usr/bin/env python2
import matplotlib.pyplot as plt
import puntertools as pt
from rwjson import readMessage,writeMessage,readJson

farg = ['--r',':m','-y','-c','-g']

def visualize_board(setup, punter_names=None):
    if punter_names is None:
        punter_names = range(100)
    have_legend = set([])
    for i in range(len(setup['riverdata'])):
        if setup['riverdata'][i]['claimed'] == None:
            riverid = setup['riverdata'][i]['id']
            source,target = setup['riverdata'][riverid]['sites']
            unclaimedx = [setup['sitemap'][source]['x'],setup['sitemap'][target]['x']]
            unclaimedy = [setup['sitemap'][source]['y'],setup['sitemap'][target]['y']]
            plt.plot(unclaimedx,unclaimedy,'-b')
        else:
            riverid = setup['riverdata'][i]['id']
            source,target = setup['riverdata'][riverid]['sites']
            claimx = [setup['sitemap'][source]['x'],setup['sites'][target]['x']]
            claimy = [setup['sites'][source]['y'],setup['sites'][target]['y']]
            whoclaimed = setup['riverdata'][i]['claimed']
            if whoclaimed in have_legend:
                plt.plot(claimx,claimy,farg[whoclaimed],linewidth = 6)
            else:
                # need legend for this
                plt.plot(claimx,claimy,farg[whoclaimed],linewidth = 6,
                         label=punter_names[whoclaimed])
                have_legend.add(whoclaimed)
    for j in range(len(setup['siteids'])):
        plt.plot(setup['sites'][j]['x'],setup['sites'][j]['y'],'k.',
                 markersize=10)
    for k in range(len(setup['mines'])):
        mineLoc = setup['mines'][k]
        plt.plot(setup['sites'][mineLoc]['x'],setup['sites'][mineLoc]['y'],'ro',
                 markersize=10)
    plt.legend(loc='best')

if __name__ == "__main__":
    with open('examples/gameplay0.txt') as f:
        handshake = readMessage(f)
        messageIn = readMessage(f)
        setup = pt.map_to_nice(messageIn['move']['state']['map'])
        R = len(setup['riverdata'])


    plt.figure()
    for turn in range(0,5):
        gameState = ['examples/gameplay'+str(turn)+'.txt']

        # Currently the map and the gamestate are not coupled.
        # By that I mean that the state id's which are specified in the file gameplay
        # are the same as the id's for maps/sample.json only

        with open(gameState[0]) as f:
            handshake = readMessage(f)
            game = readMessage(f)
            pt.update_nice(setup,game['move']['moves'])

        punter = game['move']['moves'][0]['claim']['punter']
        #    print "punter", punter
        turns = 1

        visualize_board(setup)
    plt.show()
