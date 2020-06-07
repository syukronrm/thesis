# TODO: generate objects independent, correlated, anticorrelated
import random
import pathlib
from os import path

random.seed(10)

MAX_EDGE_ID = 21693
MAX_D = 10
OBJECT_N = 10000

def generate_independent():
    objects = []
    for object_id in range(1, OBJECT_N + 1):
        edge_id = random.randrange(1, MAX_EDGE_ID + 1)
        dist = random.random()
        object = [1, object_id, edge_id, dist]
        for _i in range(0, 10):
            attr = random.random()
            object.append(attr)
        objects.append(object)
    return objects

def generate_correlated():
    objects = []
    for object_id in range(1, OBJECT_N + 1):
        edge_id = random.randrange(1, MAX_EDGE_ID + 1)
        dist = random.random()
        object = [1, object_id, edge_id, dist]
        base = random.random()
        for _i in range(0, 10):
            base_lower = base - (base * random.uniform(0.0, 0.2))
            base_upper = base + (base * random.uniform(0.0, 0.2))
            attr = random.uniform(base_lower, base_upper)
            object.append(attr)
        objects.append(object)
    return objects

def generate_anticorrelated():
    objects = []
    for object_id in range(1, OBJECT_N + 1):
        edge_id = random.randrange(1, MAX_EDGE_ID + 1)
        dist = random.random()
        object = [1, object_id, edge_id, dist]

        dist = []
        sum = 0.0
        for _i in range(0, 10):
            d = random.random()
            dist.append(d)
            sum = sum + d

        for d in dist:
            attr = (d / sum) * 1.0
            object.append(attr)

        objects.append(object)
    return objects

def save_file(path, objects):
    object = objects[0]
    header = ["action", "id", "edge_id", "distance"]
    for i in range(1, MAX_D + 1):
        header.append("d" + str(i))
    str_header = ' '.join(header)
    str_header += '\n';

    with open(path, "w") as file:
        file.writelines(str_header)
        for object in objects:
            line = []
            for o in object:
                line.append(str(o))
            str_line = ' '.join(line)
            str_line += '\n'
            file.writelines(str_line)

if __name__ == "__main__":
    ind_objects = generate_independent()
    cor_objects = generate_correlated()
    anticor_objects = generate_anticorrelated()

    project_dir = pathlib.Path(__file__).parent.parent.absolute()
    dataset_dir = path.join(project_dir, 'dataset/objects')

    ind_path = path.join(dataset_dir, 'ind.txt')
    save_file(ind_path, ind_objects)

    cor_path = path.join(dataset_dir, 'cor.txt')
    save_file(cor_path, cor_objects)

    anticor_path = path.join(dataset_dir, 'anticor.txt')
    save_file(anticor_path, anticor_objects)
