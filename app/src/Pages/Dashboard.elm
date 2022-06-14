module Pages.Dashboard exposing (..)

import Debug exposing (toString)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput, onSubmit)
import Http exposing (..)
import Json.Decode as De
import Json.Encode as En
import Router exposing (Route(..))
import Session exposing (..)
import Html.Events exposing (onClick)


type Msg
    = GetPages (Result Http.Error (Response Overview))
    | CreatePage (Result Http.Error (Response String))
    | DeletePage String
    | PageDeleted (Result Http.Error ())
    | SubmitForm
    | ChangePageName String


type alias Response data =
    { data : Maybe data
    , links : List String
    }


type alias Page = 
    { id: String
    , name: String
    }

type alias Overview = 
    { pages: List Page
    }

type PageState
    = Loading
    | Success (Response Overview)
    | Failure Http.Error


type alias Model =
    { session : Session
    , state : PageState
    , newPageName : String
    }


init : Session -> ( Model, Cmd Msg )
init session =
    ( Model session Loading ""
    , loadPages
    )


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GetPages result ->
            case result of
                Ok overview ->
                    ( { model | state = Success overview }, Cmd.none )

                Err err ->
                    ( { model | state = Failure err }, Cmd.none )
        
        DeletePage id -> 
            (model, deletePage id)

        PageDeleted result ->
            case result of
                Ok _ -> 
                    (model, loadPages)

                Err err -> 
                    ({model | state = Failure err}, Cmd.none)

        SubmitForm ->
            ( { model | state = Loading }, createPage model )

        ChangePageName newName ->
            ( { model | newPageName = newName }, Cmd.none )

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

        Success overview ->
            successView overview

        Failure error ->
            failureView error


loadingView : Html msg
loadingView =
    div [] [ text "Loading" ]


successView : Response Overview -> Html Msg
successView overview =
    case overview.data of
        Just data ->
            div [] (addPageForm :: List.map pageView data.pages)

        Nothing ->
            div [] [ text "No page yet" ]


addPageForm : Html Msg
addPageForm =
    Html.form [ onSubmit SubmitForm ]
        [ label [] [ text "Page name" ]
        , input
            [ placeholder "new page name"
            , type_ "text"
            , onInput ChangePageName
            ]
            []
        , button [type_ "submit"] [text "add page"]
        ]


pageView : Page -> Html Msg
pageView page =
    div [] 
    [ div [] [ text page.name ]
    , button [ onClick (DeletePage page.id) ] [ text "delete" ]
    ] 


failureView : Http.Error -> Html msg
failureView error =
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



-- API


deletePage : String -> Cmd Msg
deletePage id = Http.request
        { method = "DELETE"
        , headers = []
        , url = "/api/v1/pages/" ++ id
        , body = Http.emptyBody
        , expect = Http.expectWhatever PageDeleted 
        , timeout = Nothing
        , tracker = Nothing
        }

loadPages : Cmd Msg
loadPages =
    Http.get
        { url = "/api/v1/pages"
        , expect = Http.expectJson GetPages (responseDecoder overviewDecoder)
        }


createPage : Model -> Cmd Msg
createPage model =
    Http.post
        { url = "/api/v1/pages"
        , body = Http.jsonBody (pageCreateEncoder model.newPageName "/pages/article")
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
    De.map Overview (De.list pageDecoder)


pageDecoder : De.Decoder Page
pageDecoder = 
    De.map2 Page
        (De.field "id" De.string)
        (De.field "name" De.string)

pageCreateEncoder : String -> String -> En.Value
pageCreateEncoder name template =
    En.object
        [ ( "name", En.string name )
        , ( "template", En.string template )
        ]
