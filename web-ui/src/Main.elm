module Main exposing (..)

import Navigation
--import Html exposing (program)
import Messages exposing (AppMsg(..), Route(..))
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
      -- I can't put these together for some reason...
      if_value = List.length flags.artifacts > 0
      route = if if_value then
        ArtifactsRoute
      else
        Routing.router location

      cmd = if if_value then
        Cmd.none
      else
        fetchAll model

      model = initialModel flags.addr route flags.artifacts
    in
      ( model, cmd )

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
