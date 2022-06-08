module Pages.Dashboard exposing (..)

import Debug exposing (toString)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Http exposing (..)
import Json.Decode as De
import Json.Encode as En
import Router exposing (Route(..))
import Session exposing (..)


type Msg
    = GotPages (Result Http.Error (Response Overview))
    | CreateButtonClicked
    | CreatePage (Result Http.Error (Response String))


type alias Response data =
    { data : Maybe data
    , links : List String
    }


type alias Overview =
    List String


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
    , loadPages
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotPages result ->
            case result of
                Ok overview ->
                    ( { model | state = Success overview }, Cmd.none )

                Err err ->
                    ( { model | state = Failure err }, Cmd.none )

        CreateButtonClicked ->
            ( { model | state = Loading }, createPage )

        CreatePage result ->
            case result of
                Ok _ ->
                    ( model, loadPages )

                Err err ->
                    ( { model | state = Failure err }, Cmd.none )



-- Views


view : Model -> Html Msg
view model =
    case model.state of
        Loading ->
            loadingView
                |> withLayout

        Success overview ->
            successView overview
                |> withLayout

        Failure error ->
            failureView error
                |> withLayout


withLayout : List (Html msg) -> Html msg
withLayout children =
    div [] children


loadingView : List (Html msg)
loadingView =
    [ div [] [ text "Loading" ] ]


successView : Response Overview -> List (Html Msg)
successView overview =
    case overview.data of
        Just data ->
            [ div [] (addPageForm :: List.map pageView data) ]

        Nothing ->
            [ div [] [ text "No page yet" ] ]


addPageForm : Html Msg
addPageForm =
    button [ onClick CreateButtonClicked ] [ text "Create Page" ]


pageView : String -> Html msg
pageView page =
    div [] [ text page ]


failureView : Http.Error -> List (Html msg)
failureView error =
    let
        display =
            case error of
                BadUrl url ->
                    div [] [ text ("Bad Url " ++ url) ]

                Timeout ->
                    div [] [ text "We waited for ages" ]

                NetworkError ->
                    div [] [ text "Oups no network" ]

                BadStatus status ->
                    div [] [ text ("Bad status " ++ toString status) ]

                BadBody body ->
                    div [] [ text ("Bad Body : " ++ body) ]
    in
    [ display ]



-- API


loadPages : Cmd Msg
loadPages =
    Http.get
        { url = "/api/v1/pages"
        , expect = Http.expectJson GotPages (responseDecoder overviewDecoder)
        }


createPage : Cmd Msg
createPage =
    Http.post
        { url = "/api/v1/pages"
        , body = Http.jsonBody (pageCreateEncoder "test" "/pages/article")
        , expect = Http.expectJson CreatePage (responseDecoder De.string)
        }



-- Json


responseDecoder : De.Decoder data -> De.Decoder (Response data)
responseDecoder dataDecoder =
    De.map2 Response
        (De.field "data" (De.maybe dataDecoder))
        (De.field "links" (De.list De.string))


overviewDecoder : De.Decoder Overview
overviewDecoder =
    De.list De.string


pageCreateEncoder : String -> String -> En.Value
pageCreateEncoder name template =
    En.object
        [ ( "name", En.string name )
        , ( "template", En.string template )
        ]
