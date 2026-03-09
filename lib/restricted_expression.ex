defmodule CedarPolicy.RestrictedExpression do
  alias CedarPolicy.Entity

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
          | {:record, list({String.t(), t()})}
end
