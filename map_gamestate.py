import puntertools as pt
from random import randint
import random
import numpy as np
from rwjson import readMessage,writeMessage,readJson

## This just picks a random river for each player and writes to a file
##  which is read by visualizer. 

maps = ['examples/setup.sample','maps/sample.json','maps/lambda.json',\
        'maps/Sierpinski-triangle.json','maps/tube.json','maps/boston-sparse.json']
        
gameState = ['examples/gameplay','examples/gameplay1.txt']

with open(maps[3]) as f: 
    setup = readJson(f)
    info = pt.map_to_nice(setup)
R = len(info['riverdata'])

playerNum = 2
for turn in range(0,R):
    playerPunting = turn % playerNum
    
    with open(gameState[0]) as f:
        handshake = readMessage(f)
    
    randRiver = randint(0,R-1)
    
    source,target = info['riverdata'][randRiver]['sites']
    state = {"map":setup, "punter": playerPunting, "punters" : playerNum}
    claim ={"claim": {"punter": playerPunting, "source" : source, "target" : target}}
    moves = [claim]
    move = {"move":{"state":state,"moves":moves}}
    
    with open('examples/gameplay'+str(turn)+'.txt','w') as f:
        nh = writeMessage(handshake)
        message = writeMessage(move)
        f.write(nh)
        f.write(message)
