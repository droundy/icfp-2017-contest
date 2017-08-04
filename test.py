import json
import csv
import numpy as np


test_file = open('examples/setup.sample')

stuff = ['map','sites','rivers','mines']

print stuff[2]

with open('examples/setup.sample') as f:
	for l in f:
		parsed_json = json.loads(l)		
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

			
#~ test_file = open('gamestate.txt' , 'w')
#~ for item in sites:
	#~ test_file.write("%s\n" % item)
	#~ test_file.write("\n")
		
print sites
print rivers
print mines




