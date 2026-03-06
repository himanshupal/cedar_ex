defmodule CedarPolicy.Native.Version do
  defstruct [:major, :minor, :patch]

  @type t() :: %CedarPolicy.Native.Version{major: integer(), minor: integer(), patch: integer()}
end
