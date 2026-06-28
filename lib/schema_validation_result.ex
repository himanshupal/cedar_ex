defmodule CedarPolicy.SchemaValidationResult do
  defstruct [:passed, :passed_without_warnings, :errors, :warnings]

  @type t :: %__MODULE__{
          passed: boolean(),
          passed_without_warnings: boolean(),
          errors: list(String.t()),
          warnings: list(String.t())
        }
end
