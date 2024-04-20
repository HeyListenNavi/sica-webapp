from fastapi import FastAPI 
import routes.ImagesPredict as ImagesPredict

app = FastAPI()

app.include_router(ImagesPredict.router)
