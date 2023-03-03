import json
import random
from typing import List
from models import CenturyPerson, Pope, translateRecordToPapacy

def run():
    with open('..\\resources\\popes.json', encoding="UTF-8") as user_file:
        file: json = user_file.read()
        popes: List[CenturyPerson] = list(map(translateRecordToPapacy, json.loads(file)))
        picked_pope = pickapope(str(random.randint(1, 266)).zfill(3), popes)
        
        printapope(picked_pope)

def printapope(pope: Pope):
    print(f"[{pope.pontiff_number}] {pope.name_english}")

def pickapope(pontiff_id: str, popes: List[CenturyPerson]) -> Pope | None:
    return next((person.pope for person in popes if person.pope.pontiff_number == pontiff_id), None)

if __name__ == '__main__':
    run()