import sqlite3
import sys
import random

conn = sqlite3.connect("hoc.db")
c = conn.cursor()

c.execute("SELECT * FROM connections;")
connections = {}
connections[0] = "ERROR"
for i in range(1,1099):
    connections[i] = []

for i in c:
    connections[i[0]].append(i[1])
    connections[i[1]].append(i[0])
try:
    length = int(sys.argv[1])
except IndexError:
    print("Uso: python gen_random_sets.py NUMERO_DE_CIUDADES ")
    sys.exit(1)
path = []
k = random.randint(1,1098)
while length > 0:
    j = random.choice(connections[k])
    if j not in path:
        path.append(j)
        k = j
        length -= 1

print(path)
random.shuffle(path)
print(path)
