
def map_to_nice(m):
    nice = {}
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
            riverid = nice['rivermap'][c['source']][c['target']]
            if nice['riverdata']['claimed'] is None:
                nice['riverdata']['claimed'] = c['punter']
        except:
            print 'invalid move:', move
