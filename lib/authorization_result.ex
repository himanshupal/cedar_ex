defmodule CedarPolicy.AuthorizationResult do
  @moduledoc """
  Represents the result of an authorization check.
  """
  defstruct [:authorized, :errors, :reasons]

  @type t :: %__MODULE__{
          authorized: boolean(),
          errors: list(String.t()),
          reasons: list(String.t())
        }
end
