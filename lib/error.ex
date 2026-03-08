defmodule CedarPolicy.Error do
  defstruct [:source, :reason]

  @type t() :: %CedarPolicy.Error{source: atom(), reason: String.t()}
end
