module Pages.Login exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)


type Msg
    = Nothing


type alias Model =
    { user : String
    }


init : ( Model, Cmd Msg )
init =
    ( Model "test", Cmd.none )


update : Msg -> Model -> ( Model, Cmd msg )
update msg model =
    case msg of
        Nothing ->
            ( model, Cmd.none )


view : Model -> Html Msg
view model =
    div [] [ text ("My name is " ++ model.user) ]
