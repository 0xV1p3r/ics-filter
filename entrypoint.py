import os

import uvicorn

if not os.path.exists("./ics_files"):
    os.mkdir("./ics_files")

if __name__ == "__main__":
    uvicorn.run("api:api", host="0.0.0.0", port=80, log_level="info")
    