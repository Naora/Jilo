module Main exposing (main)

import Browser exposing (Document)
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (href)
import Pages.Login as Login
import Pages.NotFound as NotFound
import Router exposing (..)
import Session exposing (..)
import Url exposing (Url)


type Model
    = NotFoundPage Session
    | LoginPage Session Login.Model


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
    intoPage url (NotFoundPage session)


intoPage : Url.Url -> Model -> ( Model, Cmd Msg )
intoPage url model =
    let
        session =
            toSession model
    in
    case toRoute url of
        Login ->
            let
                ( pageModel, pageCmds ) =
                    Login.init session "Login"
            in
            ( LoginPage session pageModel, Cmd.map GotLoginMsg pageCmds )

        Dashboard ->
            let
                ( pageModel, pageCmds ) =
                    Login.init session "Dashboard"
            in
            ( LoginPage session pageModel, Cmd.map GotLoginMsg pageCmds )

        NotFound ->
            ( NotFoundPage session, Cmd.none )


toSession : Model -> Session
toSession model =
    case model of
        LoginPage session _ ->
            session

        NotFoundPage session ->
            session


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotLoginMsg loginMsg ->
            handleLoginMsg loginMsg model

        UrlRequested urlRequest ->
            let
                session =
                    toSession model
            in
            case urlRequest of
                Browser.Internal url ->
                    ( model
                    , Nav.pushUrl session.navKey (Url.toString url)
                    )

                Browser.External url ->
                    ( model
                    , Nav.load url
                    )

        UrlChanged url ->
            intoPage url model


handleLoginMsg : Login.Msg -> Model -> ( Model, Cmd Msg )
handleLoginMsg subMsg model =
    case model of
        LoginPage _ pageModel ->
            let
                ( updatedPageModel, updatedCmd ) =
                    Login.update subMsg pageModel
            in
            ( LoginPage updatedPageModel.session updatedPageModel
            , Cmd.map GotLoginMsg updatedCmd
            )

        _ ->
            ( model, Cmd.none )


view : Model -> Document Msg
view model =
    { title = "Jilo"
    , body = [ appHeader (toSession model), viewer model ]
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
    case model of
        NotFoundPage _ ->
            NotFound.view

        LoginPage _ pageModel ->
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
