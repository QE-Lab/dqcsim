import xml.etree.ElementTree as ET
import random

tree = ET.parse('puzzle-pieces.svg')
root = tree.getroot()

def tag(e):
    return e.tag.split('}')[-1]

def output_svg(fname, *out_groups):
    out_root = ET.Element(svg_ns + 'svg', {'width': '210mm', 'height': '297mm', 'viewBox': '0 0 210 297'})
    out_layer = ET.SubElement(out_root, svg_ns + 'g', {'id': 'layer1'})
    for out_group in out_groups:
        out_layer.append(out_group)
    out_tree = ET.ElementTree(out_root)
    out_tree.write(fname)

def translate(group, x, y):
    out_group = ET.Element(svg_ns + 'g', {
        'id': group_id + '_xlate',
        'transform': 'translate({},{})'.format(x, y)})
    out_group.append(group)
    return out_group

svg_ns = root.tag.split('}')[0] + '}'

for el in root:
    if tag(el) == 'g':
        layer = el
        break
else:
    raise ValueError('top layer not found')

pieces = {}
all_pieces = []

for group in layer:
    if not tag(group) == 'g':
        continue
    if not len(group):
        continue
    group_id = group.attrib['id']
    #print('parsing {}...'.format(group_id))
    out_group = ET.Element(svg_ns + 'g', {'id': group_id})
    entry = None
    ifaces = []
    for el in group:
        if tag(el) == 'text':
            if len(el) == 1:
                if tag(el[0]) == 'tspan':
                    txt = el[0].text
                    x = float(el.attrib['x'])
                    y = float(el.attrib['y'])
                    if txt.startswith('>'):
                        if entry is not None:
                            raise ValueError('multiple entry points!')
                        entry = (txt[1:], x, y)
                        continue
                    elif txt.startswith('<'):
                        ifaces.append((txt[1:], x, y))
                        continue
        out_group.append(el)
    if entry is None:
        raise ValueError('no entry point!')
    if entry[0] not in pieces:
        pieces[entry[0]] = []
    #print(' =', entry, '->', ', '.join(map(repr, ifaces)))

    pieces[entry[0]].append(entry + (ifaces, out_group))
    output_svg('pieces/{}.svg'.format(group_id), translate(out_group, -entry[1], -entry[2]))

    all_pieces.append(out_group)

if not 'start' in pieces:
    raise ValueError('no start found!')

output_svg('pieces/all-pieces.svg', *all_pieces)


def place_random(iface='start', x=0.0, y=0.0):
    if iface not in pieces:
        print('iface not found:', iface)
        return []
    _, offs_x, offs_y, ifaces, group = random.choice(pieces[iface])
    x -= offs_x
    y -= offs_y
    elements = [translate(group, x, y)]
    for next_iface, offs_x, offs_y in ifaces:
        elements.extend(place_random(next_iface, x + offs_x, y + offs_y))
    return elements

els = []
for i in range(5):
    els.extend(place_random('start', 5.0, 20.0 + 50.0 * i))

output_svg('pieces/random.svg', *els)
