module Route exposing (..)

import Browser.Navigation as Nav
import Url
import Url.Parser exposing (parse, s, top)


type alias Navigation =
    { url : Url.Url
    , key : Nav.Key
    }


type Route
    = Login
    | Dashboard
    | NotFound


routeParser : Url.Parser.Parser (Route -> c) c
routeParser =
    Url.Parser.oneOf
        [ Url.Parser.map Dashboard top
        , Url.Parser.map Dashboard (s "dashboard")
        , Url.Parser.map Login (s "login")
        ]


toRoute : Url.Url -> Route
toRoute url =
    Maybe.withDefault NotFound (parse routeParser url)
