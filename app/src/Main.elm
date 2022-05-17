module Main exposing (main)

import Browser exposing (Document)
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (href)
import Html.Events exposing (onClick)
import Pages.Login as Login
import Pages.NotFound as NotFound
import Router exposing (..)
import Session exposing (..)
import Url exposing (Url)


type Page
    = NotFoundPage
    | LoginPage Login.Model


type alias Model =
    { session : Session
    , page : Page
    }


type Msg
    = UrlRequested Browser.UrlRequest
    | UrlChanged Url.Url
    | GotLoginMsg Login.Msg


type alias Flags =
    ()


init : Flags -> Url -> Nav.Key -> ( Model, Cmd Msg )
init _ url navKey =
    let
        session =
            Session navKey "Koko" 19
    in
    intoPage url (Model session NotFoundPage)


intoPage : Url.Url -> Model -> ( Model, Cmd Msg )
intoPage url model =
    case toRoute url of
        Login ->
            let
                ( pageModel, pageCmds ) =
                    Login.init model.session "Login"
            in
            ( { model | page = LoginPage pageModel }, Cmd.map GotLoginMsg pageCmds )

        Dashboard ->
            let
                ( pageModel, pageCmds ) =
                    Login.init model.session "Dashboard"
            in
            ( { model | page = LoginPage pageModel }, Cmd.map GotLoginMsg pageCmds )

        NotFound ->
            ( { model | page = NotFoundPage }, Cmd.none )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotLoginMsg loginMsg ->
            handleLoginMsg loginMsg model

        UrlRequested urlRequest ->
            case urlRequest of
                Browser.Internal url ->
                    ( model
                    , Nav.pushUrl model.session.navKey (Url.toString url)
                    )

                Browser.External url ->
                    ( model
                    , Nav.load url
                    )

        UrlChanged url ->
            intoPage url model


handleLoginMsg : Login.Msg -> Model -> ( Model, Cmd Msg )
handleLoginMsg subMsg model =
    case model.page of
        LoginPage pageModel ->
            let
                ( updatedPageModel, updatedCmd ) =
                    Login.update subMsg pageModel
            in
            ( Model updatedPageModel.session (LoginPage updatedPageModel)
            , Cmd.map GotLoginMsg updatedCmd
            )

        _ ->
            ( model, Cmd.none )


view : Model -> Document Msg
view model =
    { title = "Jilo"
    , body = [ appHeader model.session, viewer model ]
    }


appHeader : Session -> Html msg
appHeader session =
    header []
        [ nav []
            [ ul []
                [ li [] [ a [ href "/" ] [ text "home" ] ]
                , li [] [ a [ href "/login" ] [ text "login" ] ]
                , li [] [ a [ href "/user" ] [ text session.user ] ]
                ]
            ]
        ]


viewer : Model -> Html Msg
viewer model =
    case model.page of
        NotFoundPage ->
            NotFound.view

        LoginPage pageModel ->
            Login.view pageModel
                |> Html.map GotLoginMsg


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none


main : Program () Model Msg
main =
    Browser.application
        { init = init
        , view = view
        , update = update
        , subscriptions = subscriptions
        , onUrlChange = UrlChanged
        , onUrlRequest = UrlRequested
        }
