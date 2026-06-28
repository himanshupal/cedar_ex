defmodule CedarPolicy.AuthorizationResult do
  defstruct [:authorized, :errors, :reasons]

  @type t :: %__MODULE__{
          authorized: boolean(),
          errors: list(String.t()),
          reasons: list(String.t())
        }
end
