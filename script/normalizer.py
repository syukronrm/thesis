import pathlib
from os import path

MIN_LAT = 0
MAX_LAT = 100

MIN_LNG = 0
MAX_LNG = 100

class Node:
  def __init__(self, id, lng, lat):
    self.id = id
    self.lng = lng
    self.lat = lat

class Edge:
  def __init__(self, id, n1, n2):
    self.id = id
    self.n1 = n1
    self.n2 = n2

def normalize_nodes(nodes):
  max_lng = 0
  min_lng = 10000

  max_lat = 0
  min_lat = 10000

  for node in nodes:
    if (min_lng > node.lng): min_lng = node.lng
    if (max_lng < node.lng): max_lng = node.lng

    if (min_lat > node.lat): min_lat = node.lat
    if (max_lat < node.lat): max_lat = node.lat

  diff_lng = max_lng - min_lng
  diff_lat = max_lat - min_lat

  for node in nodes:
    node.lng = abs((node.lng - min_lng) / diff_lng) * MAX_LNG
    node.lat = abs((node.lat - min_lat) / diff_lat) * MAX_LAT

  return nodes

def get_nodes(file_nodes):
  lines = file_nodes.read().splitlines()

  nodes = []

  for line in lines:
    [id, lng, lat] = line.split(' ')
    [id, lng, lat] = [int(id), abs(float(lng)), float(lat)]

    nodes.append(Node(id, lng, lat))

  return nodes

def save_nodes(nodes, path):
  with open(path, "w") as file:
    for node in nodes:
      line = str(node.id) + ' ' + str(node.lng) + ' ' + str(node.lat) + '\n'
      file.writelines(line)

def save_edges(edges, path):
  with open(path, "w") as file:
    for edge in edges:
      line = str(edge.id) + ' ' + str(edge.n1) + ' ' + str(edge.n2) + '\n'
      file.writelines(line)

def get_edges(file_edges):
  lines = file_edges.read().splitlines()

  edges = []

  for line in lines:
    [id, n1, n2, _] = line.split(' ')
    [id, n1, n2] = [int(id), int(n1), int(n2)]

    edges.append(Edge(id, n1, n2))

  return edges

if __name__ == '__main__':
  project_dir = pathlib.Path(__file__).parent.parent.absolute()
  dataset_dir = path.join(project_dir, 'dataset/california')
  original_dir = path.join(dataset_dir, 'original')

  file_edges = open(path.join(original_dir, 'cal.cedge.txt'))
  file_nodes = open(path.join(original_dir, 'cal.cnode.txt'))

  nodes = get_nodes(file_nodes)
  nodes = normalize_nodes(nodes)
  normalized_dir = path.join(dataset_dir, 'normalized')
  save_nodes(nodes, path.join(normalized_dir, 'cal.cnode.txt'))

  edges = get_edges(file_edges)
  save_edges(edges, path.join(normalized_dir, 'cal.cedge.txt'))

  print('Done.')
