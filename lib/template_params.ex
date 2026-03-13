defmodule CedarPolicy.TemplateParams do
  @moduledoc """
  """

  alias CedarPolicy.EntityUid

  @typedoc """
  Type definition for template parameters
  """
  @type t :: %{principal: EntityUid.t() | nil, resource: EntityUid.t() | nil}
end
