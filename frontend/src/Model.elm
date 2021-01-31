module Model exposing (..)

import Json.Decode as Decode exposing (Decoder)
import Json.Decode.Pipeline as Pipeline
import Json.Encode as Encode


type alias Schedule =
    { id : Maybe Int
    , hour : Maybe Int
    , minute : Maybe Int
    , createdAt : Maybe String
    , updatedAt : Maybe String
    }


initialSchedule : Schedule
initialSchedule =
    { id = Nothing
    , hour = Just 0
    , minute = Just 0
    , createdAt = Nothing
    , updatedAt = Nothing
    }


scheduleDecoder : Decoder Schedule
scheduleDecoder =
    Decode.succeed Schedule
        |> Pipeline.required "id" (Decode.maybe Decode.int)
        |> Pipeline.required "hour" (Decode.maybe Decode.int)
        |> Pipeline.required "minute" (Decode.maybe Decode.int)
        |> Pipeline.required "created_at" (Decode.maybe Decode.string)
        |> Pipeline.required "updated_at" (Decode.maybe Decode.string)


scheduleEncoder : Schedule -> Encode.Value
scheduleEncoder schedule =
    Encode.object
        [ case schedule.id of
            Just id ->
                ( "id", Encode.int id )

            _ ->
                ( "id", Encode.null )
        , ( "hour", Encode.int (Maybe.withDefault 0 schedule.hour) )
        , ( "minute", Encode.int (Maybe.withDefault 0 schedule.hour) )
        , case schedule.createdAt of
            Just createdAt ->
                ( "created_at", Encode.string createdAt )

            _ ->
                ( "created_at", Encode.null )
        , case schedule.updatedAt of
            Just updatedAt ->
                ( "updated_at", Encode.string updatedAt )

            _ ->
                ( "updated_at", Encode.null )
        ]


type alias WateringTime =
    { id : Maybe Int
    , time : Maybe Int
    , createdAt : Maybe String
    , updatedAt : Maybe String
    }


initialWateringTime : WateringTime
initialWateringTime =
    { id = Nothing
    , time = Just 0
    , createdAt = Nothing
    , updatedAt = Nothing
    }


wateringTimeDecoder : Decoder WateringTime
wateringTimeDecoder =
    Decode.succeed WateringTime
        |> Pipeline.required "id" (Decode.maybe Decode.int)
        |> Pipeline.required "time" (Decode.maybe Decode.int)
        |> Pipeline.required "created_at" (Decode.maybe Decode.string)
        |> Pipeline.required "updated_at" (Decode.maybe Decode.string)


wateringTimeEncoder : WateringTime -> Encode.Value
wateringTimeEncoder wateringTime =
    Encode.object
        [ case wateringTime.id of
            Just id ->
                ( "id", Encode.int id )

            _ ->
                ( "id", Encode.null )
        , ( "time", Encode.int (Maybe.withDefault 0 wateringTime.time) )
        , case wateringTime.createdAt of
            Just createdAt ->
                ( "created_at", Encode.string createdAt )

            _ ->
                ( "created_at", Encode.null )
        , case wateringTime.updatedAt of
            Just updatedAt ->
                ( "updated_at", Encode.string updatedAt )

            _ ->
                ( "updated_at", Encode.null )
        ]


type alias Device =
    { id : Maybe Int
    , name : Maybe String
    , serialNumber : Maybe String
    , deviceTypeId : Maybe Int
    , mac : Maybe String
    , ip : Maybe String
    , createdAt : Maybe String
    , updatedAt : Maybe String
    , showInDashboard : Maybe Int
    }


initialDevice : Device
initialDevice =
    { id = Nothing
    , name = Just ""
    , serialNumber = Just ""
    , deviceTypeId = Nothing
    , mac = Just ""
    , ip = Just ""
    , createdAt = Nothing
    , updatedAt = Nothing
    , showInDashboard = Just 1
    }


deviceDecoder : Decoder Device
deviceDecoder =
    Decode.succeed Device
        |> Pipeline.required "id" (Decode.maybe Decode.int)
        |> Pipeline.required "name" (Decode.maybe Decode.string)
        |> Pipeline.required "serial_number" (Decode.maybe Decode.string)
        |> Pipeline.required "device_type_id" (Decode.maybe Decode.int)
        |> Pipeline.required "mac" (Decode.maybe Decode.string)
        |> Pipeline.required "ip" (Decode.maybe Decode.string)
        |> Pipeline.required "created_at" (Decode.maybe Decode.string)
        |> Pipeline.required "updated_at" (Decode.maybe Decode.string)
        |> Pipeline.required "show_in_dashboard" (Decode.maybe Decode.int)


deviceEncoder : Device -> Encode.Value
deviceEncoder device =
    Encode.object
        [ case device.id of
            Just id ->
                ( "id", Encode.int id )

            _ ->
                ( "id", Encode.null )
        , ( "name", Encode.string (Maybe.withDefault "" device.name) )
        , ( "serial_number", Encode.string (Maybe.withDefault "" device.serialNumber) )
        , ( "device_type_id", Encode.int (Maybe.withDefault 0 device.deviceTypeId) )
        , ( "mac", Encode.string (Maybe.withDefault "" device.mac) )
        , ( "ip", Encode.string (Maybe.withDefault "" device.ip) )
        , case device.createdAt of
            Just createdAt ->
                ( "created_at", Encode.string createdAt )

            _ ->
                ( "created_at", Encode.null )
        , case device.updatedAt of
            Just updatedAt ->
                ( "updated_at", Encode.string updatedAt )

            _ ->
                ( "updated_at", Encode.null )
        , ( "show_in_dashboard", Encode.int (Maybe.withDefault 1 device.showInDashboard) )
        ]


type alias DeviceType =
    { id : Maybe Int
    , name : Maybe String
    , createdAt : Maybe String
    , updatedAt : Maybe String
    }


initialDeviceType : DeviceType
initialDeviceType =
    { id = Nothing
    , name = Just ""
    , createdAt = Nothing
    , updatedAt = Nothing
    }


deviceTypeDecoder : Decoder DeviceType
deviceTypeDecoder =
    Decode.succeed DeviceType
        |> Pipeline.required "id" (Decode.maybe Decode.int)
        |> Pipeline.required "name" (Decode.maybe Decode.string)
        |> Pipeline.required "created_at" (Decode.maybe Decode.string)
        |> Pipeline.required "updated_at" (Decode.maybe Decode.string)


deviceTypeEncoder : DeviceType -> Encode.Value
deviceTypeEncoder deviceType =
    Encode.object
        [ case deviceType.id of
            Just id ->
                ( "id", Encode.int id )

            _ ->
                ( "id", Encode.null )
        , ( "name", Encode.string (Maybe.withDefault "" deviceType.name) )
        , case deviceType.createdAt of
            Just createdAt ->
                ( "created_at", Encode.string createdAt )

            _ ->
                ( "created_at", Encode.null )
        , case deviceType.updatedAt of
            Just updatedAt ->
                ( "updated_at", Encode.string updatedAt )

            _ ->
                ( "updated_at", Encode.null )
        ]
