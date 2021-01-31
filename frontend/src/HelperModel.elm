module HelperModel exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline as Pipeline
import Json.Encode as Encode


type alias WaterBody =
    { id : Int
    , waterOn : Bool
    }


initialWaterBody : WaterBody
initialWaterBody =
    { id = 0, waterOn = False }


waterBodyDecoder : Decoder WaterBody
waterBodyDecoder =
    Decode.succeed WaterBody
        |> Pipeline.required "id" Decode.int
        |> Pipeline.required "water_on" Decode.bool


waterBodyEncoder : WaterBody -> Encode.Value
waterBodyEncoder waterBody =
    Encode.object
        [ ( "id", Encode.int waterBody.id )
        , ( "water_on", Encode.bool waterBody.waterOn )
        ]
