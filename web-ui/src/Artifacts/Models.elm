module Artifacts.Models exposing (..)

import Regex

import JsonRpc exposing (RpcError)


type alias ArtifactId =
  Int

type alias Loc =
  { path : String
  , row : Int
  , col : Int
  }

type alias Artifact =
  { id : ArtifactId
  , name : String
  , raw_name : String
  , path : String
  , text : String
  , partof : List String
  , parts : List String
  , loc : Maybe Loc
  , completed : Float
  , tested : Float
  , config: ArtifactConfig
  }

type alias ArtifactConfig =
  { partsExpanded : Bool
  , partofExpanded : Bool
  , pathExpanded : Bool
  , locExpanded : Bool
  , textExpanded : Bool
  }

type alias ArtifactsResponse =
  { result: Maybe (List Artifact)
  , error: Maybe RpcError
  }

defaultConfig : ArtifactConfig
defaultConfig =
  { partsExpanded = False
  , partofExpanded = False
  , pathExpanded = False
  , locExpanded = False
  , textExpanded = False
  }


artifactsUrl =
  "#artifacts" 

--artifactUrl : ArtifactId -> String
--artifactUrl id =
--  "#artifacts/" ++ (toString id)

artifactNameUrl name =
  "#artifacts/" ++ name

-- get the real name from a raw name
realName : String -> String
realName name =
  let
    replaced = Regex.replace Regex.All (Regex.regex " ") (\_ -> "") name
  in
    String.toUpper replaced

