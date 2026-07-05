defmodule CedarPolicy.SchemaValidationResult do
  @moduledoc """
  Validation result of a schema validation operation.
  It contains information about whether the validation passed, whether it passed without warnings, and any errors or warnings that were encountered during the validation process.
  """

  defstruct [:passed, :passed_without_warnings, :errors, :warnings]

  @type t :: %__MODULE__{
          passed: boolean(),
          passed_without_warnings: boolean(),
          errors: list(String.t()),
          warnings: list(String.t())
        }
end
