defmodule CedarPolicy.RestrictedExpression do
  alias CedarPolicy.EntityUid

  @type t() ::
          {:set, list(t())}
          | {:long, integer()}
          | {:bool, boolean()}
          | {:ip, String.t()}
          | {:string, String.t()}
          | {:decimal, String.t()}
          | {:date_time, String.t()}
          | {:duration, String.t()}
          | {:entity_uid, EntityUid.t()}
          | {:record, list({String.t(), t()})}
end
