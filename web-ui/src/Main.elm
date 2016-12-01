module Main exposing (..)

import Navigation
--import Html exposing (program)
import Messages exposing (AppMsg(..), Route)
import Models exposing (Model, initialModel)
import View exposing (view)
import Update exposing (update)
import Routing
import Artifacts.Commands exposing (fetchAll)
import Artifacts.Models exposing (Artifact)

type alias Flags =
  { addr: String
  , artifacts: List Artifact
  }

init : Flags -> Navigation.Location -> (Model, Cmd AppMsg)
init flags location =
    let
      model = initialModel 
        flags.addr 
        (Routing.router location) 
        flags.artifacts
    in
      ( model, fetchAll model )

subscriptions : Model -> Sub AppMsg
subscriptions model =
  Sub.none

-- MAIN

main =
    Navigation.programWithFlags Routing.routerMsg
      { init = init
      , view = view
      , update = update
      , subscriptions = subscriptions
      }
