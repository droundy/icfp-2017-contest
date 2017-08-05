#!/usr/bin/env python2
import json
import csv
import numpy as np

class Empty:
	pass

with open('examples/setup.sample') as f:
	#~ contents = f.read()
	for l in f:
		setup = json.loads(l)
		setup['map']['rivers']
		newsetup = Empty()
		newsetup.map = setup['map']
		
		print parsed_json	
		parsed_map = parsed_json['map']
		parsed_sites = parsed_map['sites']
		mines = parsed_map['mines']

		parsed_rivers = parsed_map['rivers']
		
		sites = np.zeros((len(parsed_sites),3))
		rivers = np.zeros((len(parsed_rivers),2))
		
		for site in range(len(parsed_sites)):
			site_info = parsed_sites[site]
			sites[site,0] = site_info['id']
			sites[site,1] = site_info['x']
			sites[site,2] = site_info['y']
		
		for river in range(len(parsed_rivers)):
			river_info = parsed_rivers[river]
			rivers[river,0] = river_info['source']
			rivers[river,1] = river_info['target']



		
#~ print sites
#~ print rivers
print mines

output = json.dumps({"mines":mines},sort_keys = True)


test_file = open('gamestate1.txt' , 'w')
for item in output:
	test_file.write("%s" % item)


with open('gamestate.txt') as f:
	contents = f.read()

print contents

