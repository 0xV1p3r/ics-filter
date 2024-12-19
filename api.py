from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

api = FastAPI()

api.mount("/calendars", StaticFiles(directory="ics_files"), name="calendars")
