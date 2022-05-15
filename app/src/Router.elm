module Router exposing (..)

import Url
import Url.Parser exposing (Parser, map, parse, s, top)


type Route
    = Login
    | Dashboard
    | NotFound


routeParser : Parser (Route -> c) c
routeParser =
    Url.Parser.oneOf
        [ map Dashboard top
        , map Dashboard (s "dashboard")
        , map Login (s "login")
        ]


toRoute : Url.Url -> Route
toRoute url =
    Maybe.withDefault NotFound (parse routeParser url)
