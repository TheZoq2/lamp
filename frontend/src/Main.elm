import Html exposing (..)
import Html
import Html.Events exposing (..)
import Http
import Json.Encode
import Json.Decode


type LedCommand
    = Color Int Int Int
    | Toggle



type alias Model =
    {}

init : (Model, Cmd Msg)
init =
    ({}, Cmd.none)




-- Update

type Msg
    = SendCommand LedCommand
    | CommandResponse
    | NetworkError Http.Error


update : Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        SendCommand command ->
            (model, sendCommand command)
        CommandResponse ->
            (model, Cmd.none)
        NetworkError e ->
            let
                _ = Debug.log "Network error: " e
            in
                (model, Cmd.none)


checkHttpAttempt : (a -> Msg) -> Result Http.Error a -> Msg
checkHttpAttempt msgFunc result =
    case result of
        Ok val ->
            msgFunc val
        Err e ->
            NetworkError e

sendCommand : LedCommand -> Cmd Msg
sendCommand command =
    let
        encoder = case command of
            Color r b g ->
                Json.Encode.object
                    [ ("Color", Json.Encode.list <| List.map Json.Encode.int [r,g,b])
                    ]
            Toggle ->
                Json.Encode.string "Toggle"

        request = 
            Http.post "led" (Http.jsonBody encoder) Json.Decode.value
    in
        Http.send
            (checkHttpAttempt (\_ -> CommandResponse))
            request


-- View

view : Model -> Html Msg
view model =
    div []
        [ h1 [] [text "LED controller"]
        , button [onClick (SendCommand (Color 255 255 255))] [text "White"]
        , button [onClick (SendCommand (Color 255 0 0))] [text "Red"]
        , button [onClick (SendCommand (Color 0 255 0))] [text "Green"]
        , button [onClick (SendCommand (Color 0 0 255))] [text "Blue"]
        , button [onClick (SendCommand (Color 255 172 68))] [text "Warm white"]
        , button [onClick (SendCommand Toggle)] [text "Toggle"]
        ]




-- subscriptions

subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.none


-- Main

main : Program Never Model Msg
main =
    Html.program
        { init = init
        , update = update
        , view = view
        , subscriptions = subscriptions
        }


