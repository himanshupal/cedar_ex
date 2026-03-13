defmodule CedarPolicy.RestrictedExpression do
  @moduledoc """
  """

  alias CedarPolicy.Record
  alias CedarPolicy.Entity

  @typedoc """
  Type definition for restricted params
  """
  @type t() ::
          {:set, list(t())}
          | {:long, integer()}
          | {:bool, boolean()}
          | {:ip, String.t()}
          | {:string, String.t()}
          | {:decimal, String.t()}
          | {:date_time, String.t()}
          | {:duration, String.t()}
          | {:entity, Entity.t()}
          | {:record, Record.t()}
end
