module Pages.Login exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
import Session exposing (..)
import String exposing (length)


type Msg
    = Nothing
    | NameChanged String


type alias Model =
    { session : Session
    , user : String
    }


init : Session -> String -> ( Model, Cmd Msg )
init session name =
    ( Model session name, Cmd.none )


update : Msg -> Model -> ( Model, Cmd msg )
update msg model =
    case msg of
        Nothing ->
            ( model, Cmd.none )

        NameChanged newName ->
            let
                session =
                    model.session
            in
            ( { model | session = { session | user = newName } }, Cmd.none )


view : Model -> Html Msg
view model =
    div []
        [ text ("My name is " ++ model.user)
        , input [ type_ "text", onInput NameChanged ] []
        ]
