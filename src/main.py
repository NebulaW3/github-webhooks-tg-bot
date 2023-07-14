from fastapi import FastAPI, Request

app = FastAPI()


@app.post("/payload")
async def process_hook(request: Request):
    data = await request.json()
    print(data)
    return {"message": ""}
