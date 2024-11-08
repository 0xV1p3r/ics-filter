from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
import os

if not os.path.exists("./ics_files"):
    os.mkdir("./ics_files")

api = FastAPI()

api.mount("/calendars", StaticFiles(directory="ics_files"), name="calendars")

