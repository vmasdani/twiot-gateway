module Main exposing (..)

import Browser.Hash as Hash
import Browser
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Url
import Svg exposing (svg, path)
import Url.Parser as Parser exposing (Parser)
import Material.Icons
import Material.Icons.Types exposing (Coloring(..))

type Route = Home | Devices | NotFound

route : Parser (Route -> a) a
route =
  Parser.oneOf
    [ Parser.map Home Parser.top
    , Parser.map Devices (Parser.s "devices")
    ]

toRoute : String -> Route
toRoute string =
  case Url.fromString string of
    Nothing ->
      NotFound

    Just url ->
      Maybe.withDefault NotFound (Parser.parse route url)

-- MAIN


main : Program () Model Msg
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


type alias Model =
  { key : Nav.Key
  , url : Url.Url
  }


init : () -> Url.Url -> Nav.Key -> ( Model, Cmd Msg )
init flags url key =
  ( Model key url, Cmd.none )



-- UPDATE


type Msg
  = LinkClicked Browser.UrlRequest
  | UrlChanged Url.Url


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
  case msg of
    LinkClicked urlRequest ->
      case urlRequest of
        Browser.Internal url ->
          ( model, Nav.pushUrl model.key (Url.toString url) )

        Browser.External href ->
          ( model, Nav.load href )

    UrlChanged url ->
      ( { model | url = url }
      , Cmd.none
      )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
  Sub.none



-- VIEW


view : Model -> Browser.Document Msg
view model =
  { title = "URL Interceptor"
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
                [ li [ class "nav-item" ]
                  [ a [ attribute "aria-current" "page", class "nav-link active", href "#" ]
                    [ Html.text "Dashboard" ]
                  ]
                , li [ class "nav-item" ]
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
          Home -> dashboardView model
          Devices -> devicesView model
          _ -> notFoundView
      ] 
  }


viewLink : String -> Html msg
viewLink path =
  li [] [ a [ href path ] [ text path ] ]

dashboardView model =
  div [ class "m-3" ]
    [ h3 [] [ text "Dashboard" ]
    , hr [] []
    , div [] 
        [ h3 [] [ text "Control valve" ] ]
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
    ]

sensorsView model =
  div [ class "m-3" ]
    [ h3 [] [ text "Sensors" ]
    ]



notFoundView =
  div [] [ text "Not Found." ]