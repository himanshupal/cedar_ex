defmodule CedarPolicy.Error do
  @moduledoc """

  """

  defstruct [:source, :reason]

  @type t() :: %__MODULE__{source: atom(), reason: String.t()}
end
