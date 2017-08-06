
def map_to_nice(m):
    nice = {}
    sitemap = {}
    for s in m['sites']:
        sitemap[s['id']] = s
    nice['sitemap'] = sitemap
    nice['sites'] = m['sites']
    nice['siteids'] = []
    for s in m['sites']:
        nice['siteids'].append(s['id'])
    nice['mines'] = m['mines']
    nice['rivers'] = m['rivers']
    riverdata = []
    rivermap = {}
    for r in m['rivers']:
        riverid = len(riverdata)
        riverdata.append({
            'id': riverid,
            'sites': (r['target'], r['source']),
            'claimed': None,
        })
        if r['source'] not in rivermap:
            rivermap[r['source']] = {}
        rivermap[r['source']][r['target']] = riverid
        if r['target'] not in rivermap:
            rivermap[r['target']] = {}
        rivermap[r['target']][r['source']] = riverid
    nice['rivermap'] = rivermap
    nice['riverdata'] = riverdata
    return nice

def update_nice(nice, moves):
    for move in moves:
        if 'pass' in move:
            continue
        try:
            c = move['claim']
            # print 'claim is', c
            riverid = nice['rivermap'][c['source']][c['target']]
            #print 'riverid is', riverid
            if nice['riverdata'][riverid]['claimed'] is None:
                nice['riverdata'][riverid]['claimed'] = c['punter']
        except:
            print 'invalid move:', move

def distances(nice, mineid):
    distances = {mineid: 0}
    current_distance = 0
    old_sites = {mineid}
    while len(distances) < len(nice['siteids']):
        current_distance += 1
        new_sites = set([])
        for site in old_sites:
            for neighbor in  nice['rivermap'][site]:
                if neighbor not in distances:
                    distances[neighbor] = current_distance+1
                    new_sites.add(neighbor)
        old_sites = new_sites
    return distances

def punter_reaches(nice, mineid, punterid):
    reached = set([])
    old_sites = {mineid}
    while len(old_sites) > 0:
        new_sites = set([])
        for site in old_sites:
            for neighbor in nice['rivermap'][site]:
                if neighbor not in reached and nice['riverdata'][nice['rivermap'][site][neighbor]]['claimed'] == punterid:
                    reached.add(neighbor)
                    new_sites.add(neighbor)
        old_sites = new_sites
    return reached

def score(nice, punterid):
    totalscore = 0
    for m in nice['mines']:
        d = distances(nice, m)
        for r in punter_reaches(nice, m, punterid):
            totalscore += d[r]**2
    return totalscore
