defmodule CedarPolicy.Version do
  defstruct [:major, :minor, :patch, :build, :pre]

  @type t() :: %CedarPolicy.Version{
          major: integer(),
          minor: integer(),
          patch: integer(),
          build: String.t(),
          pre: String.t()
        }
end
