import os

import uvicorn
from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles

if not os.path.exists("./ics_files"):
    os.mkdir("./ics_files")

api = FastAPI()

api.mount("/calendars", StaticFiles(directory="ics_files"), name="calendars")

if __name__ == "__main__":
    uvicorn.run("api:api", host="0.0.0.0", port=80, log_level="info")
    