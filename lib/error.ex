defmodule CedarPolicy.Error do
  @moduledoc """
  Represents an error in the Cedar policy system happening due to improper data or failing to parse some input.
  """

  defstruct [:source, :reason]

  @type t() :: %__MODULE__{source: atom(), reason: String.t()}
end
