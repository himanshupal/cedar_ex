defmodule CedarPolicy.Native.EntityUid do
  defstruct [:type_name, :id]

  @type t() :: %CedarPolicy.Native.EntityUid{type_name: String.t(), id: String.t()}
end
