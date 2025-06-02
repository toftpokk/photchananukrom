import sqlite3
import json
import sys


if len(sys.argv) < 2:
    print("no input file provided")

input_file = sys.argv[1]
output_file = "database.sqlite"

conn = sqlite3.connect(output_file)

with open(input_file, 'r') as json_file:
    data = json.load(json_file)

conn.execute("""CREATE TABLE IF NOT EXISTS words (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word TEXT NOT NULL
);""")

conn.execute("""
CREATE TABLE IF NOT EXISTS definitions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    word_id INTEGER NOT NULL,
    definition TEXT NOT NULL,
    definition_header TEXT NOT NULL,
    FOREIGN KEY (word_id) 
        REFERENCES words(id)
        ON DELETE CASCADE
);""")

for item in data:
    result = conn.execute("INSERT INTO words (word) VALUES (?)", 
                 (item["word"],))
    result.lastrowid
    for dfn in item["data"]:
        try:
            conn.execute("INSERT INTO definitions (word_id, definition, definition_header) VALUES (?,?,?)",
                     (result.lastrowid, json.dumps(dfn["definition"]), dfn["word"]))
        except Exception as e:
            print("Error:",e)
            exit(1)
conn.commit()
conn.close()
