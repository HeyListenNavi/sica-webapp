from fastapi import APIRouter, HTTPException

# List of HTTPExceptions
# https://en.wikipedia.org/wiki/List_of_HTTP_status_codes#4xx_client_errors

router = APIRouter()

# Models
class Prediction:
    x_coordinate: int
    y_coordinate: int
    width: int
    height: int
    detected_object: str
    probs: float

    def __init__(self, x_coordinate: int, y_coordinate: int, width: int, height: int, detected_object: str, probs: float):
        self.x_coordinate = x_coordinate
        self.y_coordinate = y_coordinate
        self.width = width
        self.height = height
        self.detected_object = detected_object
        self.probs = probs

class Object:
    object_name: str

    def __init__(self, object_name):
        self.object_name = object_name

# Example
# ip:port/predict?base64_image=examplestring -> returns
# {
#   "x_coordinate": 10,
#   "y_coordinate": 10,
#   "width": 100,
#   "height": 100,
#   "cls": "objeto detectado",
#   "probs": 90.99
# }
@router.get("/predict")
async def predict(base64_image: str):
    # Example on how to raise an exception if something fails
    # in this case if we make the paramter equal to "exception" it'll throw an exception
    if base64_image == "exception":
        # In this case it would return
        # {
        #   "detail": "Not found"
        # }
        raise HTTPException(status_code=404, detail="Not found")
    else:
        # Example on how to use arguments
        base64_image = "Hello world"
    
    #decoded_image = io.BytesIO(base64.b64decode(base64_string))
    #source = Image.open(decoded_image)
    #results = model(source)

    # Create a response based on the prediction response model
    response = Prediction(10, 10, 100, 100, "objeto detectado", 90.99)
    
    return response

# Example
# ip:port/objects-list -> returns
# [
#   {
#       "object_name": Objeto 1"
#   },
#   {
#       "object_name": Objeto 2"
#   },
#   {
#       "object_name": Objeto 3"
#   },
# ]
@router.get("/objects-list")
async def objects():
    # Example objects list
    response = []
    response.append(Object("Objeto 1"))
    response.append(Object("Objeto 2"))
    response.append(Object("Objeto 3"))
    
    return response