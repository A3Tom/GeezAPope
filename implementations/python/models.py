from dataclasses import dataclass
import json
from typing import Dict

@dataclass
class Pope:
    def __init__(self, pontiff_number: str, pontificate: str, personal_name: str,
                 name_english: str, name_latin: str, date_of_birth: str, birthplace: str,
                 age_at_start_of_papacy: int, age_at_end_of_papacy: int):
        self.pontiff_number = pontiff_number
        self.pontificate = pontificate
        self.personal_name = personal_name
        self.name_english = name_english
        self.name_latin = name_latin
        self.date_of_birth = date_of_birth
        self.birthplace = birthplace
        self.age_at_start_of_papacy = age_at_start_of_papacy
        self.age_at_end_of_papacy = age_at_end_of_papacy

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__)
    
@dataclass
class CenturyPerson:
    def __init__(self, century: str, pope: Pope, current: bool):
        self.century = century
        self.pope = pope
        self.current = current


def translateRecordToPapacy(record: Dict) -> CenturyPerson:
    pope = translatePopeRecordToPope(record["person"])
    return CenturyPerson(
        record["century"],
        pope,
        record["current"]
    )

def translatePopeRecordToPope(pope_record) -> Pope:
    return Pope(
        pope_record["Pontiff Number"],
        pope_record["Pontificate"],
        pope_record["Personal Name"],
        pope_record["Name in English"],
        pope_record["Name in Latin"],
        pope_record["Date of Birth"],
        pope_record["Birthplace"],
        pope_record["Age at start of Papacy"],
        pope_record["Age at end of Papacy"]
    )
