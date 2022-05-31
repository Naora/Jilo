module Main exposing (main)

import Browser exposing (Document)
import Browser.Navigation as Nav
import Html exposing (..)
import Html.Attributes exposing (href)
import Pages.Dashboard as Dashboard
import Pages.Login as Login
import Pages.NotFound as NotFound
import Router exposing (..)
import Session exposing (Session)
import Url exposing (Url)


type Model
    = NotFoundPage Session
    | LoginPage Login.Model
    | DashboardPage Dashboard.Model


type Msg
    = UrlRequested Browser.UrlRequest
    | UrlChanged Url.Url
    | GotLoginMsg Login.Msg
    | GotDashboardMsg Dashboard.Msg


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
    case toRoute url of
        Login ->
            withSession model
                |> Login.init "Login"
                |> updateWith LoginPage GotLoginMsg

        Dashboard ->
            withSession model
                |> Dashboard.init
                |> updateWith DashboardPage GotDashboardMsg

        NotFound ->
            ( withSession model
                |> NotFoundPage
            , Cmd.none
            )


withSession : Model -> Session
withSession model =
    case model of
        LoginPage pageModel ->
            pageModel.session

        NotFoundPage session ->
            session

        DashboardPage pageModel ->
            pageModel.session


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case ( msg, model ) of
        ( GotLoginMsg subMsg, LoginPage pageModel ) ->
            Login.update subMsg pageModel
                |> updateWith LoginPage GotLoginMsg

        ( GotDashboardMsg subMsg, DashboardPage pageModel ) ->
            Dashboard.update subMsg pageModel
                |> updateWith DashboardPage GotDashboardMsg

        ( UrlRequested urlRequest, _ ) ->
            let
                session =
                    withSession model
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

        ( UrlChanged url, _ ) ->
            intoPage url model

        ( _, _ ) ->
            ( model, Cmd.none )


updateWith : (subModel -> Model) -> (subMsg -> Msg) -> ( subModel, Cmd subMsg ) -> ( Model, Cmd Msg )
updateWith toPage toMsg ( subModel, subCmd ) =
    ( toPage subModel
    , Cmd.map toMsg subCmd
    )


view : Model -> Document Msg
view model =
    { title = "Jilo"
    , body = [ appHeader model, viewer model ]
    }


appHeader : Model -> Html msg
appHeader model =
    let
        session =
            withSession model
    in
    header []
        [ nav []
            [ ul []
                [ li [] [ a [ href "/" ] [ text "home" ] ]
                , li [] [ a [ href "/login" ] [ text "login" ] ]
                , li [] [ a [ href "/test" ] [ text "test" ] ]
                , li [] [ a [ href "/user" ] [ text session.user ] ]
                ]
            ]
        ]


viewer : Model -> Html Msg
viewer model =
    case model of
        NotFoundPage _ ->
            NotFound.view

        LoginPage pageModel ->
            Login.view pageModel
                |> Html.map GotLoginMsg

        DashboardPage pageModel ->
            Dashboard.view pageModel
                |> Html.map GotDashboardMsg


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
