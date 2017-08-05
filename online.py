#!/usr/bin/env python2

import socket, sys, os, subprocess

import rwjson

punter_args = ['target/debug/punter']

if len(sys.argv) > 1:
    punter_args = sys.argv[1:]

BUFFER_SIZE = 1024*1024

def offline_call(args, inp):
    print 'calling', args, 'with', inp
    x = subprocess.Popen(args, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    data = rwjson.readMessage(x.stdout)
    print 'got offline handshake response', data
    x.stdin.write(rwjson.writeMessage({'you': data['me']}))
    x.stdin.write(rwjson.writeMessage(inp))
    print 'have written back to offline'
    return rwjson.readMessage(x.stdout)

s = socket.create_connection(('punter.inf.ed.ac.uk', 9002))
print "writing a handshake message"
s.send(rwjson.writeMessage({"me": "Xiphon"}))
print "sent handshake"
data = rwjson.recvMessage(s)
print "got handshake", data

setup = rwjson.recvMessage(s)
print 'setup is', setup

x = offline_call(punter_args, setup)
print 'state is', x

s.send(rwjson.writeMessage(x))
print 'have sent state back!'

while True:
    data = rwjson.recvMessage(s)
    print '\ngot respons back from server', data
    if 'stop' in data:
        print 'stopping', data
        break
    response = offline_call(punter_args, data)
    print '\ngot offline response', response
    s.send(rwjson.writeMessage(response))
