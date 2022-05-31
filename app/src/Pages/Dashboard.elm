module Pages.Dashboard exposing (..)

import Html exposing (..)
import Html.Attributes exposing (..)
import Http exposing (..)
import Session exposing (..)
import Json.Decode as De
import Router exposing (Route(..))


type Msg
    = GotPages (Result Http.Error (Response Overview))

type alias Response data = 
    { data: Maybe data
    , links: List String
    }

type alias Overview =  List String

type PageState
    = Loading
    | Success (Response Overview)
    | Failure Http.Error


type alias Model =
    { session : Session
    , state : PageState
    }


init : Session -> ( Model, Cmd Msg )
init session =
    ( Model session Loading
    , Http.get
        { url = "/api/pages"
        , expect = Http.expectJson GotPages (responseDecoder overviewDecoder)
        }
    )


update : Msg -> Model -> ( Model, Cmd msg )
update msg model =
    case msg of
        GotPages result ->
            case result of
                Ok overview ->
                    ( { model | state = Success overview }, Cmd.none )

                Err err ->
                    ( { model | state = Failure err }, Cmd.none )


view : Model -> Html Msg
view model =
    case model.state of
        Loading ->
            loadingView

        Success overview ->
            successView overview

        Failure error ->
            failureView error


loadingView : Html msg
loadingView =
    div []
        [ text "Loading"
        ]


successView : Response Overview -> Html msg
successView overview =
    case overview.data of
        Just data ->
            div [] (List.map pageView data)
        
        Nothing ->
            div [] [text "No page yet"]


pageView : String -> Html msg
pageView page =
    div [] [text page]


failureView : Http.Error -> Html msg
failureView error =
    case error of
        BadUrl url ->
            div[] [text ("Wrong Url " ++ url)]

        Timeout ->
            div[] [text ("Wrong Url ")]

        NetworkError ->
            div[] [text ("Wrong Url ")]

        BadStatus _ ->
            div[] [text ("Wrong Url ")]

        BadBody body ->
            div[] [text ("Bad Body : " ++ body)]


-- Json

responseDecoder : De.Decoder data -> De.Decoder (Response data)
responseDecoder dataDecoder = De.map2 Response
    (De.field "data" (De.maybe dataDecoder))
    (De.field "links" (De.list De.string))

overviewDecoder : De.Decoder Overview
overviewDecoder = De.list De.string
