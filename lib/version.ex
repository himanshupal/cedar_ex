defmodule CedarPolicy.Version do
  defstruct [:major, :minor, :patch]

  @type t() :: %CedarPolicy.Version{major: integer(), minor: integer(), patch: integer()}
end
