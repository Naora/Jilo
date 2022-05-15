module Pages.NotFound exposing (..)

import Html exposing (Html, h3, text)


view : Html msg
view =
    h3 [] [ text "Oops! The page you requested was not found!" ]
