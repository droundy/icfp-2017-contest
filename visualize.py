#!/usr/bin/env python2
import matplotlib.pyplot as plt
import puntertools as pt
from rwjson import readMessage,writeMessage,readJson

farg = ['--r',':m','-y','-c','-g', ':k']

def visualize_board(nice, punter_names=None):
    if punter_names is None:
        punter_names = range(100)
    have_legend = set([])
    for riverdata in nice['riverdata']:
        riverid = riverdata['id']
        source,target = riverdata['sites']
        x = [nice['sitemap'][source]['x'],nice['sitemap'][target]['x']]
        y = [nice['sitemap'][source]['y'],nice['sitemap'][target]['y']]
        if riverdata['claimed'] == None:
            plt.plot(x,y,'-b')
        else:
            whoclaimed = riverdata['claimed']
            if whoclaimed in have_legend:
                plt.plot(x,y,farg[whoclaimed],linewidth = 6)
            else:
                # need legend for this
                plt.plot(x,y,farg[whoclaimed],linewidth = 6,
                         label=punter_names[whoclaimed])
                have_legend.add(whoclaimed)
    for j in range(len(nice['siteids'])):
        plt.plot(nice['sites'][j]['x'],nice['sites'][j]['y'],'k.',
                 markersize=10)
    for k in range(len(nice['mines'])):
        mineLoc = nice['mines'][k]
        plt.plot(nice['sites'][mineLoc]['x'],nice['sites'][mineLoc]['y'],'ro',
                 markersize=10)
    plt.legend(loc='best')

if __name__ == "__main__":
    with open('examples/gameplay0.txt') as f:
        handshake = readMessage(f)
        messageIn = readMessage(f)
        nice = pt.map_to_nice(messageIn['move']['state']['map'])
        R = len(nice['riverdata'])


    plt.figure()
    for turn in range(0,5):
        gameState = ['examples/gameplay'+str(turn)+'.txt']

        # Currently the map and the gamestate are not coupled.
        # By that I mean that the state id's which are specified in the file gameplay
        # are the same as the id's for maps/sample.json only

        with open(gameState[0]) as f:
            handshake = readMessage(f)
            game = readMessage(f)
            pt.update_nice(nice,game['move']['moves'])

        punter = game['move']['moves'][0]['claim']['punter']
        #    print "punter", punter
        turns = 1

        visualize_board(nice)
    plt.show()
