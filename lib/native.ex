defmodule CedarPolicy.Native do
  use Rustler, otp_app: :cedar_policy, crate: "cedar_ex"

  alias CedarPolicy.EntityUid
  alias CedarPolicy.Entity
  alias CedarPolicy.Record

  @spec new() :: reference()
  def new(), do: not_loaded()

  @spec add_policy(
          state :: reference(),
          policy :: String.t(),
          id :: String.t() | nil
        ) :: reference()

  def add_policy(state, policy, id \\ nil)
  def add_policy(_state, _policy, _id), do: not_loaded()

  @spec add_policy_json(
          state :: reference(),
          policy :: String.t(),
          id :: String.t() | nil
        ) :: reference()

  def add_policy_json(state, policy, id \\ nil)
  def add_policy_json(_state, _policy, _id), do: not_loaded()

  @spec add_template(
          state :: reference(),
          template :: String.t(),
          id :: String.t() | nil
        ) :: reference()

  def add_template(state, template, id \\ nil)
  def add_template(_state, _template, _id), do: not_loaded()

  def link(_state, _template_id, _policy_id, _values), do: not_loaded()

  @spec add_entities(
          state :: reference(),
          entities :: list(Entity.t()),
          schema :: String.t() | nil
        ) :: reference()

  def add_entities(_state, _entities, _schema \\ nil)
  def add_entities(_state, _entities, _schema), do: not_loaded()

  @spec add_entities(
          state :: reference(),
          entities :: String.t(),
          schema :: String.t() | nil
        ) :: reference()

  def add_entities_json(_state, _entities, _schema \\ nil)
  def add_entities_json(_state, _entities, _schema), do: not_loaded()

  @spec validate(state :: reference(), schema :: String.t(), strict :: boolean() | nil) ::
          boolean()
  def validate(_state, _schema, _strict \\ nil)
  def validate(_state, _schema, _strict), do: not_loaded()

  @spec validate_json(state :: reference(), schema :: String.t(), strict :: boolean() | nil) ::
          boolean()
  def validate_json(_state, _schema, _strict \\ nil)
  def validate_json(_state, _schema, _strict), do: not_loaded()

  @spec get_lang_version() :: Version.t()
  def get_lang_version(), do: not_loaded()

  @spec get_sdk_version() :: Version.t()
  def get_sdk_version(), do: not_loaded()

  @spec verify(
          state :: reference(),
          principal :: EntityUid.t(),
          action :: EntityUid.t(),
          resource :: EntityUid.t(),
          context :: Record.t(),
          schema :: String.t() | nil
        ) :: boolean()
  def verify(state, p, a, r, c, s \\ nil)
  def verify(_state, _p, _a, _r, _c, _s), do: not_loaded()

  defp not_loaded, do: :erlang.nif_error(:nif_not_loaded)
end
