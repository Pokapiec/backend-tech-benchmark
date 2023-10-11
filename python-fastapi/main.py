from fastapi import FastAPI, UploadFile, File, Depends
from databases import Database

app = FastAPI()


# Initialize the database connection
database = Database("postgresql://postgres:postgres@localhost/postgres")
database.connect()


@app.on_event("startup")
async def startup():
    await database.connect()


@app.on_event("shutdown")
async def shutdown():
    await database.disconnect()


# Use the database dependency
async def get_database() -> Database:
    return database


@app.get("/string/")
async def string():
    return "Hello world!"


@app.get("/simple-json/")
async def simple_json():
    return {
        "key1": "value1",
        "key2": "value2",
        "key3": "value3",
        "key_nest": {"kn1": "value_nest_1", "knn2": {"key": "value"}},
    }


@app.get("/query-params/")
async def query_params(param1: str, param2: str):
    return {"param1": param1, "param2": param2}


@app.get("/sql-select/")
async def sql_select(db: Database = Depends(get_database)):
    result = await db.fetch_all("SELECT * FROM public.exampletable ORDER BY id ASC")
    return result


@app.post("/file-upload/")
async def file_upload(file: UploadFile = File(...)):
    file_content = await file.read()
    return file_content
