module Main exposing (..)

-- import Html.Styled.Attributes exposing (css)

import Browser
import Browser.Hash as Hash
import Browser.Navigation as Nav
import Css
import HelperModel exposing (..)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Html.Styled.Attributes exposing (css)
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import List.Extra
import Material.Icons
import Material.Icons.Types exposing (Coloring(..))
import Maybe
import Model exposing (..)
import Svg exposing (path, svg)
import Url
import Url.Parser as Parser exposing ((</>), Parser)


type Route
    = Home
    | Devices
    | DeviceDetail Int
    | NotFound


route : Parser (Route -> a) a
route =
    Parser.oneOf
        [ Parser.map Home Parser.top
        , Parser.map Devices (Parser.s "devices")
        , Parser.map DeviceDetail (Parser.s "devices" </> Parser.int)
        ]


toRoute : String -> Route
toRoute string =
    case Url.fromString string of
        Nothing ->
            NotFound

        Just url ->
            Maybe.withDefault NotFound (Parser.parse route url)



-- MAIN


main : Program Flag Model Msg
main =
    Hash.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = LinkClicked
        }



-- MODEL


type RequestStatus
    = NotAsked
    | Loading
    | Success
    | Error


type alias Flag =
    { baseUrl : String
    }


type alias Model =
    { key : Nav.Key
    , url : Url.Url
    , baseUrl : String
    , deviceTypes : List DeviceType
    , device : Device
    , devices : List Device
    , schedules : List Schedule
    , requestStatus : RequestStatus
    }


fetchBasedOnUrl : Model -> Url.Url -> Cmd Msg
fetchBasedOnUrl model url =
    -- (Debug.log <| "FetchBasedOnUrl: " ++ Url.toString url)
    case toRoute (Url.toString url) of
        Home ->
            Cmd.batch
                [ Http.get
                    { url = model.baseUrl ++ "/devices"
                    , expect = Http.expectJson GotDevices (Decode.list deviceDecoder)
                    }
                ]

        Devices ->
            Http.get
                { url = model.baseUrl ++ "/devices"
                , expect = Http.expectJson GotDevices (Decode.list deviceDecoder)
                }

        DeviceDetail id ->
            Http.get
                { url = model.baseUrl ++ "/devices/" ++ String.fromInt id
                , expect = Http.expectJson GotDevice deviceDecoder
                }

        _ ->
            Cmd.none


fetchDeviceTypes : Model -> Cmd Msg
fetchDeviceTypes model =
    Http.get
        { url = model.baseUrl ++ "/devicetypes"
        , expect = Http.expectJson GotDeviceTypes (Decode.list deviceTypeDecoder)
        }


init : Flag -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url key =
    let
        initialModel : Model
        initialModel =
            { key = key
            , url = url
            , baseUrl = flags.baseUrl
            , deviceTypes = []
            , devices = []
            , device = initialDevice
            , schedules = []
            , requestStatus = NotAsked
            }
    in
    ( initialModel
    , Cmd.batch [ fetchBasedOnUrl initialModel url, fetchDeviceTypes initialModel ]
    )



-- UPDATE


type Msg
    = LinkClicked Browser.UrlRequest
    | UrlChanged Url.Url
    | MovePage String
    | GotDevices (Result Http.Error (List Device))
    | GotDevice (Result Http.Error Device)
    | GotDeviceTypes (Result Http.Error (List DeviceType))
    | ChangeDeviceName String
    | ToggleDeviceShowInDashboard
    | SaveDeviceDetail
    | SavedDevice (Result Http.Error ())
    | ValveOpen Int Bool
    | OpenedValve (Result Http.Error ())


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        OpenedValve _ ->
            ( model, Cmd.none )

        ValveOpen id open ->
            ( model
            , Http.post
                { url = model.baseUrl ++ "/water"
                , body = Http.jsonBody (waterBodyEncoder { id = id, waterOn = open })
                , expect = Http.expectWhatever OpenedValve
                }
            )

        SavedDevice _ ->
            ( model, Nav.pushUrl model.key "/#/devices" )

        SaveDeviceDetail ->
            -- (Debug.log <| "JSON encoded:" ++ Encode.encode 0 (deviceEncoder model.device))
            --     ( model, Cmd.none )
            ( model
            , Http.post
                { url = model.baseUrl ++ "/devices"
                , body = Http.jsonBody (deviceEncoder model.device)
                , expect = Http.expectWhatever SavedDevice
                }
            )

        ToggleDeviceShowInDashboard ->
            let
                device =
                    model.device

                newDevice =
                    { device
                        | showInDashboard =
                            case device.showInDashboard of
                                Just showInDashboard ->
                                    if showInDashboard == 1 then
                                        Just 0

                                    else
                                        Just 1

                                _ ->
                                    Just 1
                    }
            in
            ( { model | device = newDevice }, Cmd.none )

        ChangeDeviceName name ->
            let
                device =
                    model.device

                newDevice =
                    { device | name = Just name }
            in
            ( { model | device = newDevice }, Cmd.none )

        MovePage urlString ->
            ( model, Nav.pushUrl model.key urlString )

        GotDevices res ->
            case res of
                Ok devices ->
                    ( { model | devices = devices }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        GotDevice res ->
            case res of
                Ok device ->
                    ( { model | device = device }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        GotDeviceTypes res ->
            case res of
                Ok deviceTypes ->
                    ( { model | deviceTypes = deviceTypes }, Cmd.none )

                _ ->
                    ( model, Cmd.none )

        LinkClicked urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model, Nav.pushUrl model.key (Url.toString url) )

                Browser.External href ->
                    ( model, Nav.load href )

        UrlChanged url ->
            (Debug.log <| Debug.toString url)
                ( { model | url = url }
                , fetchBasedOnUrl model url
                )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
    { title = "TWIoT"
    , body =
        [ nav
            [ class "navbar navbar-expand-lg navbar-dark"
            , Html.Attributes.style "background-color" "green"
            , Html.Attributes.style "color" "white"
            ]
            [ div [ class "container-fluid" ]
                [ a [ class "navbar-brand", href "#" ]
                    [ Html.text "TWIoT" ]
                , button [ attribute "aria-controls" "navbarSupportedContent", attribute "aria-expanded" "false", attribute "aria-label" "Toggle navigation", class "navbar-toggler", attribute "data-bs-target" "#navbarSupportedContent", attribute "data-bs-toggle" "collapse", type_ "button" ]
                    [ span [ class "navbar-toggler-icon" ]
                        []
                    ]
                , div [ class "collapse navbar-collapse", id "navbarSupportedContent" ]
                    [ ul [ class "navbar-nav me-auto mb-2 mb-lg-0" ]
                        [ -- li [ class "nav-item" ]
                          -- [ a [ attribute "aria-current" "page", class "nav-link active", href "#" ]
                          --     [ Html.text "Dashboard" ]
                          -- ]
                          li [ class "nav-item" ]
                            [ a [ class "nav-link active", href "#/devices" ]
                                [ Html.text "Devices" ]
                            ]

                        -- , li [ class "nav-item" ]
                        --   [ a [ class "nav-link active", href "#/sensors" ]
                        --     [ Html.text "Sensors" ]
                        --   ]
                        -- , li [ class "nav-item dropdown" ]
                        --   [ a [ attribute "aria-expanded" "false", class "nav-link dropdown-toggle active", attribute "data-bs-toggle" "dropdown", href "#", id "navbarDropdown", attribute "role" "button" ]
                        --     [ text "Dropdown          " ]
                        --   , ul [ attribute "aria-labelledby" "navbarDropdown", class "dropdown-menu" ]
                        --     [ li []
                        --       [ a [ class "dropdown-item", href "#" ]
                        --         [ text "Action" ]
                        --       ]
                        --     , li []
                        --       [ a [ class "dropdown-item", href "#" ]
                        --         [ text "Another action" ]
                        --       ]
                        --     , li []
                        --       [ hr [ class "dropdown-divider" ]
                        --         []
                        --       ]
                        --     , li []
                        --       [ a [ class "dropdown-item", href "#" ]
                        --         [ text "Something else here" ]
                        --       ]
                        --     ]
                        --   ]
                        -- , li [ class "nav-item" ]
                        --   [ a [ attribute "aria-disabled" "true", class "nav-link  active", href "#", attribute "tabindex" "-1" ]
                        --     [ text "Disabled" ]
                        --   ]
                        ]

                    -- , Html.form [ class "d-flex" ]
                    --   [ input [ attribute "aria-label" "Search", class "form-control me-2", placeholder "Search", type_ "search" ]
                    --     []
                    --   , button [ class "btn btn-outline-success", type_ "submit" ]
                    --     [ text "Search" ]
                    --   ]
                    ]
                ]
            ]

        -- , Html.text "The current URL is: "
        -- , b [] [ Html.text (Url.toString model.url) ]
        -- , div []
        --     [ Html.text
        --         <| Debug.toString
        --         <| toRoute
        --         <| (Url.toString model.url)
        --     ]
        -- , ul []
        --     [ viewLink "/#/"
        --     , viewLink "/#/home"
        --     , viewLink "/#/profile"
        --     , viewLink "/#/reviews/the-century-of-the-self"
        --     , viewLink "/#/reviews/public-opinion"
        --     , viewLink "/#/reviews/shah-of-shahs"
        --     ]
        , case toRoute (Url.toString model.url) of
            Home ->
                dashboardView model

            Devices ->
                devicesView model

            DeviceDetail id ->
                deviceDetailView model

            _ ->
                notFoundView
        ]
    }


viewLink : String -> Html msg
viewLink path =
    li [] [ a [ href path ] [ text path ] ]


dashboardView : Model -> Html Msg
dashboardView model =
    div [ class "m-3" ]
        [ --     h3 [] [ text "Da shboard" ]
          -- , hr [] []
          -- ,
          div []
            [ h3 [] [ text "Control valve" ] ]
        , div []
            (model.devices
                |> List.filter (\device -> device.showInDashboard == Just 1)
                |> List.map
                    (\device ->
                        div [ class "card shadow p-2" ]
                            [ div [ class "d-flex justify-content-center" ]
                                [ text <| Maybe.withDefault "" device.name ]
                            , div [ class "d-flex justify-content-around" ]
                                [ div [ class "btn btn-success", onClick (ValveOpen (Maybe.withDefault 0 device.id) True) ] [ Material.Icons.lock_open 24 Inherit ]
                                , div [ class "btn btn-danger", onClick (ValveOpen (Maybe.withDefault 0 device.id) False) ] [ Material.Icons.lock 24 Inherit ]
                                ]
                            ]
                    )
            )
        , hr [] []
        , div []
            [ h3 [] [ text "Energy Monitor" ] ]

        -- , div []
        --     [ button [ class "btn btn-outline-primary btn-sm" ]
        --         [ text <| "Test ", Material.Icons.settings 16 Inherit ] ]
        ]


devicesView model =
    div [ class "m-3" ]
        [ h3 [] [ text "Devices" ]
        , div []
            (List.map
                (\device ->
                    a
                        [ style "text-decoration" "none"
                        , class "text-dark"
                        , href <| "/#/devices/" ++ String.fromInt (Maybe.withDefault 0 device.id)
                        ]
                        [ div
                            [ class "card shadow p-3 my-2", style "cursor" "pointer" ]
                            [ div []
                                [ h4 []
                                    [ text <|
                                        String.join ""
                                            [ case device.name of
                                                Just name ->
                                                    if name == "" then
                                                        "Unnnamed Device"

                                                    else
                                                        name

                                                _ ->
                                                    ""
                                            ]
                                    ]
                                ]
                            , div []
                                [ text <|
                                    String.join ""
                                        [ "Type: "
                                            ++ (case
                                                    List.Extra.find
                                                        (\deviceType -> Maybe.withDefault 0 deviceType.id == Maybe.withDefault 0 device.deviceTypeId)
                                                        model.deviceTypes
                                                of
                                                    Just deviceType ->
                                                        Maybe.withDefault "" deviceType.name

                                                    _ ->
                                                        ""
                                               )
                                        , ", ID: "
                                        , String.fromInt (Maybe.withDefault 0 device.id)
                                        ]
                                ]
                            , div []
                                [ text <|
                                    String.join ""
                                        [ Maybe.withDefault "" device.mac
                                        , " | "
                                        , Maybe.withDefault "" device.ip
                                        ]
                                ]
                            , div [ class "d-flex" ]
                                [ text "Show in dashboard? "
                                , case device.showInDashboard of
                                    Just 1 ->
                                        div [ class "text-success mx-1" ] [ text "Yes" ]

                                    Just 0 ->
                                        div [ class "text-danger mx-1" ] [ text "No" ]

                                    _ ->
                                        div [] []
                                ]
                            ]
                        ]
                )
                model.devices
            )
        ]


deviceDetailView : Model -> Html Msg
deviceDetailView model =
    div [ class "m-3" ]
        [ div [ class "d-flex align-items-center" ]
            [ h3 [] [ text "Device Detail" ]
            , button
                [ onClick SaveDeviceDetail
                , class "btn btn-primary btn-sm"
                ]
                [ text "Save" ]
            ]
        , hr [] []
        , div []
            [ div [] [ text <| "ID: " ++ String.fromInt (Maybe.withDefault 0 model.device.id) ]
            , div [] [ text <| "MAC Address:" ++ Maybe.withDefault "" model.device.mac ]
            , div [] [ text <| "IP Address:" ++ Maybe.withDefault "" model.device.ip ]
            ]
        , hr [] []
        , div []
            [ div [] [ small [] [ text "Name" ] ]
            , input
                [ value
                    (case model.device.name of
                        Just name ->
                            name

                        _ ->
                            ""
                    )
                , onInput ChangeDeviceName
                , class "form-control"
                , placeholder "Device name..."
                ]
                []
            ]
        , div [ class "my-2" ]
            [ div [] [ small [] [ text "Show in Dashboard?" ] ]
            , div [ class "form-check form-switch" ]
                [ input
                    [ type_ "checkbox"
                    , class "form-check-input"
                    , onClick ToggleDeviceShowInDashboard
                    , checked
                        (case model.device.showInDashboard of
                            Just showInDashboard ->
                                showInDashboard == 1

                            _ ->
                                False
                        )
                    ]
                    []
                ]
            ]
        ]


sensorsView model =
    div [ class "m-3" ]
        [ h3 [] [ text "Sensors" ]
        ]


notFoundView =
    div [] [ text "Not Found." ]
